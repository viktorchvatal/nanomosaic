use log::*;
use crate::{common::{log_err, convert_err}, message::{GuiMessage, LogicMessage, Rgba}};
use glib::{Sender as GlibSender};
use image::{open, GrayImage};
use nanocv::{ImgBuf, ImgSize, Img};

pub struct State {
    gui: Option<GlibSender<GuiMessage>>,
    image: Option<ImgBuf<Rgba>>,
}

impl State {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            gui: None,
            image: None,
        })
    }

    pub fn receive(&mut self, message: LogicMessage) -> Result<(), String> {
        match message {
            LogicMessage::InitGui(channel) => Ok(self.init_gui(channel)),
            LogicMessage::LoadImage(path) => Ok(self.load_image(&path)),
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
                self.image = Some(img);
            },
            Err(msg) => {
                warn!("Loading image {} failed: {}", path, msg);        
            }
        };
        
        if let Some(ref gui) = self.gui {
            if let Some(ref img) = self.image {
                log_err(gui.send(GuiMessage::RenderSelect(img.clone())));
                log_err(gui.send(GuiMessage::RenderResult(img.clone())));
            }
        }
    }
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