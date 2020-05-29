use log::*;
use crate::common::{
    convert_err, resize, resize_factor, draw_full_horizontal_line,
    draw_full_vertical_line
};
use crate::message::*;
use glib::{Sender as GlibSender};
use image::{open};
use nanocv::{ImgBuf, ImgSize, Img, Vec2d, Range2d, filter::map_range};
use std::cmp::{min, max};

pub struct LogicState {
    gui: Option<GlibSender<GuiMessage>>,
    image: ImgBuf<Rgba>,
    select_size: ImgSize,
    result_size: ImgSize,
    compositor: CompositorSender,
    start: Vec2d<isize>,
    end: Vec2d<isize>,
    result_modified: bool,
    compositor_free: bool,
}

impl MessageReceiver<LogicMessage> for LogicState {
    fn receive(&mut self, message: LogicMessage) -> Result<(), String> {
        use LogicMessage::*;
        match message {
            InitGui(channel) => Ok(self.init_gui(channel)),
            LoadImage(path) => Ok(self.load_image(&path)),
            ImageResized((id, size)) => Ok(self.image_resized(id, size)),
            MouseDown((button, x, y)) => Ok(self.mouse_down(button, x, y)),
        }
    }
}

impl LogicState {
    pub fn new(compositor: CompositorSender) -> Self {
        Self {
            gui: None,
            image: ImgBuf::<Rgba>::new_init(ImgSize::new(1, 1), [0, 0, 0, 0]),
            select_size: ImgSize::new(1, 1),
            result_size: ImgSize::new(1, 1),
            compositor,
            start: Vec2d::new(0, 0),
            end: Vec2d::new(0, 0),
            result_modified: true,
            compositor_free: true,
        }
    }

    fn mouse_down(&mut self, button: u32, x: f64, y: f64) {
        let factor = resize_factor(self.image.size(), self.select_size);
        let point = Vec2d::new((x as f64/factor) as isize, (y as f64/factor) as isize);

        match button {
            1 => self.start = point,
            3 => self.end = point,
            _ => {}
        }

        self.render_select_image();
        self.render_result_image();
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
                self.start = Vec2d::new(0, 0);
                self.end = self.image.range().end();
            },
            Err(msg) => {
                warn!("Loading image {} failed: {}", path, msg);        
            }
        };
    }

    fn render_select_image(&self) {
        let mut img = resize(&self.image, self.select_size);
        let factor = resize_factor(self.image.size(), self.select_size);
        draw_full_horizontal_line(&mut img, (self.start.y as f64*factor) as isize);
        draw_full_vertical_line(&mut img, (self.start.x as f64*factor) as isize);
        draw_full_horizontal_line(&mut img, (self.end.y as f64*factor) as isize);
        draw_full_vertical_line(&mut img, (self.end.x as f64*factor) as isize);
        send_glib(&self.gui, GuiMessage::RenderSource(img));
    }

    fn selected_range(&self) -> Range2d<isize> {
        let x1 = min(self.start.x, self.end.x);
        let x2 = max(self.start.x, self.end.x);
        let y1 = min(self.start.y, self.end.y);
        let y2 = max(self.start.y, self.end.y);
        Range2d::new(x1..x2, y1..y2)
    }

    fn render_result_image(&self) {
        let range = self.selected_range();

        let buffer = if range.width() > 0 && range.height() > 0 {
            let patch_size = ImgSize::new(range.width() as usize, range.height() as usize);
            let mut buffer = ImgBuf::<Rgba>::new(patch_size);
            let output_range = buffer.range();
            map_range(&self.image, &mut buffer, range, output_range, |i, _| i);    
            buffer
        } else {
            ImgBuf::<Rgba>::new_init(ImgSize::new(1, 1), [0, 0, 0, 0])
        };

        send(
            &self.compositor, 
            CompositeMessage::CompositeMosaic((buffer, self.result_size))
        );
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

