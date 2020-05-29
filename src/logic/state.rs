use log::*;
use crate::{common::{log_err, convert_err}};
use crate::message::{GuiMessage, LogicMessage, Rgba, ImageId, MessageReceiver};
use glib::{Sender as GlibSender};
use image::{open};
use nanocv::{ImgBuf, ImgSize, Img, Vec2d};
use nanocv::filter::{
    resize_nearest_new, map_range, mirror_horizontal_new, mirror_vertical_new
};

pub struct LogicState {
    gui: Option<GlibSender<GuiMessage>>,
    image: ImgBuf<Rgba>,
    select_size: ImgSize,
    result_size: ImgSize,
}

impl MessageReceiver<LogicMessage> for LogicState {
    fn receive(&mut self, message: LogicMessage) -> Result<(), String> {
        match message {
            LogicMessage::InitGui(channel) => Ok(self.init_gui(channel)),
            LogicMessage::LoadImage(path) => Ok(self.load_image(&path)),
            LogicMessage::ImageResized((id, size)) => Ok(self.image_resized(id, size)),
        }
    }
}

impl LogicState {
    pub fn new() -> Self {
        Self {
            gui: None,
            image: ImgBuf::<Rgba>::new_init(ImgSize::new(1, 1), [0, 0, 0, 0]),
            select_size: ImgSize::new(1, 1),
            result_size: ImgSize::new(1, 1),
        }
    }

    fn image_resized(&mut self, id: ImageId, size: ImgSize) {
                match id {
            ImageId::Select => if self.select_size != size {
                self.select_size = size;
                self.render_select_image();
            },
            ImageId::Result => if self.result_size != size {
                self.result_size = size;
                self.render_result_image();
            },
        }
    }

    fn init_gui(&mut self, channel: GlibSender<GuiMessage>) {
        self.gui = Some(channel);
        debug!("Logic: GUI channel initialized.")        
    }

    fn load_image(&mut self, path: &str) {
        match load_image(&path) {
            Ok(img) => {
                info!("Image {} x {} loaded", img.width(), img.height());        
                self.image = img;
            },
            Err(msg) => {
                warn!("Loading image {} failed: {}", path, msg);        
            }
        };
    }

    fn render_select_image(&self) {
        if let Some(ref gui) = self.gui {
            let resized = resize(&self.image, self.select_size);
            log_err(gui.send(GuiMessage::Render((ImageId::Select, resized))));
        }    
    }

    fn render_result_image(&self) {
        if let Some(ref gui) = self.gui {
            let resized = resize(&self.image, self.result_size/2);
            let mosaic = create_mosaic(&resized);
            log_err(gui.send(GuiMessage::Render((ImageId::Result, mosaic))));
        }    
    }
}

fn resize(source: &ImgBuf<Rgba>, target_size: ImgSize) -> ImgBuf<Rgba> {
    if target_size.x == 0 || target_size.y == 0 {
        return ImgBuf::new(target_size);
    }

    let factor_x = target_size.x as f64/source.width() as f64;
    let factor_y = target_size.y as f64/source.height() as f64;
    let factor = if factor_x > factor_y {factor_y} else {factor_x};

    let target = ImgSize::new(
        (source.width() as f64*factor) as usize,
        (source.height() as f64*factor) as usize,
    );

    resize_nearest_new(source, target)
}

fn load_image(path: &str) -> Result<ImgBuf<Rgba>, String> {
    let buf = convert_err(open(path))?.into_rgba();
    let size = ImgSize::new(buf.width() as usize, buf.height() as usize);
    Ok(ImgBuf::from_vec(size, byte_vector_to_rgba(buf.into_vec())))
}

fn byte_vector_to_rgba(input: Vec<u8>) -> Vec<Rgba> {
    let pixels = input.len()/4;
    let mut result = vec![[0, 0, 0, 0]; pixels];
    let mut offset = 0;

    while offset < input.len() {
        result[offset/4] = [
            input[offset], 
            input[offset + 1],
            input[offset + 2],
            input[offset + 3],
        ];
        offset += 4;
    }

    result
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