use log::*;
use glib::{Sender as GlibSender};
use crate::message::{CompositeMessage, GuiMessage, MessageReceiver, Rgba, send_glib, ImageId};
use crate::common::resize;
use nanocv::{ImgBuf, ImgSize, Img, Vec2d};
use nanocv::filter::{
    map_range, mirror_horizontal_new, mirror_vertical_new
};

pub struct CompositorState { 
    gui: Option<GlibSender<GuiMessage>>,    
}

impl MessageReceiver<CompositeMessage> for CompositorState {
    fn receive(&mut self, message: CompositeMessage) -> Result<(), String> {
        match message {
            CompositeMessage::InitGui(channel) => Ok(self.init_gui(channel)),
            CompositeMessage::CompositeMosaic((img, size)) => Ok(self.composite(img, size)),
        }
    }
}

impl CompositorState {
    pub fn new() -> Self {
        Self {
            gui: None
        }
    }    

    fn init_gui(&mut self, channel: GlibSender<GuiMessage>) {
        self.gui = Some(channel);
        debug!("Compositor: GUI channel initialized.")        
    }    

    fn composite(&self, img: ImgBuf<Rgba>, size: ImgSize) {
        let resized = resize(&img, size/2);
        let mosaic = create_mosaic(&resized);
        send_glib(&self.gui, GuiMessage::Render((ImageId::Result, mosaic)));        
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