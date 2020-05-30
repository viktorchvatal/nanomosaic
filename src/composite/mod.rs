use log::*;
use glib::{Sender as GlibSender};
use crate::message::*;
use crate::common::{resize, convert_err};
use nanocv::{ImgBuf, ImgSize, Img, Vec2d};
use nanocv::filter::{
    map_range, mirror_horizontal_new, mirror_vertical_new
};
use image::RgbaImage;

pub struct CompositorState { 
    logic: LogicSender,
    gui: Option<GlibSender<GuiMessage>>,    
}

impl MessageReceiver<CompositeMessage> for CompositorState {
    fn receive(&mut self, message: CompositeMessage) -> Result<(), String> {
        match message {
            CompositeMessage::InitGui(channel) => Ok(self.init_gui(channel)),
            CompositeMessage::CompositeMosaic((img, size)) => Ok(self.composite(img, size)),
            CompositeMessage::SaveMosaic((img, path)) => self.save(img, &path),
        }
    }
}

impl CompositorState {
    pub fn new(logic: LogicSender) -> Self {
        Self {logic, gui: None}
    }    

    fn init_gui(&mut self, channel: GlibSender<GuiMessage>) {
        self.gui = Some(channel);
        debug!("Compositor: GUI channel initialized.")        
    }    

    fn composite(&self, img: ImgBuf<Rgba>, size: ImgSize) {
        let resized = resize(&img, size/2);
        let mosaic = create_mosaic(&resized);
        send_glib(&self.gui, GuiMessage::RenderTarget(mosaic));        
        send(&self.logic, LogicMessage::CompositorFinished)
    }

    fn save(&self, img: ImgBuf<Rgba>, path: &str) -> Result<(), String> {
        let mosaic = create_mosaic(&img);
        let size = mosaic.size();
        let pixels = mosaic.into_vec();
        let bytes = rgba_to_bytes(pixels);

        let result = RgbaImage::from_vec(size.x as u32, size.y as u32, bytes)
            .ok_or("Could not allocate image data")?;

        convert_err(result.save(path))
    }    
}

fn create_mosaic(image: &ImgBuf<Rgba>) -> ImgBuf<Rgba> {
    let mut result = ImgBuf::new(image.size()*2);
    let mirror_x = mirror_horizontal_new(image);
    let mirror_y = mirror_vertical_new(image);
    let mirror_xy = mirror_vertical_new(&mirror_x);
    let src = image.range();
    let (w, h) = (image.width() as isize, image.height() as isize);

    map_range(image, &mut result, src, src, |x, _| x);
    map_range(&mirror_x, &mut result, src, src + Vec2d::new(w, 0), |x, _| x);
    map_range(&mirror_y, &mut result, src, src + Vec2d::new(0, h), |x, _| x);
    map_range(&mirror_xy, &mut result, src, src + Vec2d::new(w, h), |x, _| x);

    result
}

fn rgba_to_bytes(input: Vec<Rgba>) -> Vec<u8> {
    let count = input.len()*4;
    let mut result = vec![0u8; count];
    let mut offset = 0;

    while offset < count {
        let input = input[offset/4];
        result[offset + 0] = input[0];
        result[offset + 1] = input[1];
        result[offset + 2] = input[2];
        result[offset + 3] = input[3];

        offset += 4;
    }

    result
}
