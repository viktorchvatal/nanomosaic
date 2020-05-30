use log::*;
use crate::common::{
    convert_err, resize, resize_factor
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
    source_modified: bool,
    last_source_size: Option<ImgSize>,
    last_rendered_lines: Option<SelectionLines>,
}

impl MessageReceiver<LogicMessage> for LogicState {
    fn receive(&mut self, message: LogicMessage) -> Result<(), String> {
        use LogicMessage::*;
        match message {
            InitGui(channel) => Ok(self.init_gui(channel)),
            LoadImage(path) => Ok(self.load_image(&path)),
            ImageResized((id, size)) => Ok(self.image_resized(id, size)),
            MouseDown((button, x, y)) => Ok(self.mouse_down(button, x, y)),
            CompositorFinished => {
                self.compositor_free = true;
                Ok(self.render_select_image())
            },
            ReturnBuffer(_image) => Ok(()),
            SaveImage(path) => Ok(self.save_image(path)),
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
            source_modified: true,
            last_source_size: None,
            last_rendered_lines: None,
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

        self.render_all();
    }

    fn render_all(&mut self) {
        self.source_modified = true;
        self.result_modified = true;
        self.render_select_image();
        self.render_result_image();
    }

    fn image_resized(&mut self, id: ImageId, size: ImgSize) {
                match id {
            ImageId::Select => if self.select_size != size {
                self.select_size = size;
                self.source_modified = true;
                self.render_select_image();
            },
            ImageId::Result => if self.result_size != size {
                self.result_size = size;
                self.result_modified = true;
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
                self.last_source_size = None;
                self.last_rendered_lines = None;
                self.render_all();
            },
            Err(msg) => {
                warn!("Loading image {} failed: {}", path, msg);        
            }
        };
    }

    fn render_select_image(&mut self) {
        let lines = self.selection_lines();

        if let Some(size) = self.last_source_size {
            if size == self.select_size {
                // Image in GUI is already rendered, just update selection lines
                if let Some(last_lines) = self.last_rendered_lines {
                    send_glib(&self.gui, GuiMessage::RenderLines(last_lines));
                };
                self.last_rendered_lines = Some(lines);
                send_glib(&self.gui, GuiMessage::RenderLines(lines));
                return;
            }
        }
        let img = resize(&self.image, self.select_size);
        self.last_source_size = Some(self.select_size);
        self.last_rendered_lines = Some(lines);
        send_glib(&self.gui, GuiMessage::RenderSource(img));
        send_glib(&self.gui, GuiMessage::RenderLines(lines));
    }

    fn selection_lines(&self) -> SelectionLines {
        let factor = resize_factor(self.image.size(), self.select_size);

        SelectionLines {
            x1: (self.start.x as f64*factor) as isize,
            x2: (self.end.x as f64*factor) as isize,
            y1: (self.start.y as f64*factor) as isize,
            y2: (self.end.y as f64*factor) as isize
        }        
    }

    fn selected_range(&self) -> Range2d<isize> {
        let x1 = min(self.start.x, self.end.x);
        let x2 = max(self.start.x, self.end.x);
        let y1 = min(self.start.y, self.end.y);
        let y2 = max(self.start.y, self.end.y);
        Range2d::new(x1..x2, y1..y2)
    }

    fn save_image(&mut self, path: String) {
        debug!("Save image path: {}", &path);

        let buffer = self.get_selected_patch();

        send(
            &self.compositor, 
            CompositeMessage::SaveMosaic((buffer, path))
        );
    }    

    fn render_result_image(&self) {
        if !self.result_modified { return; }
        if !self.compositor_free { return; }

        let buffer = self.get_selected_patch();

        send(
            &self.compositor, 
            CompositeMessage::CompositeMosaic((buffer, self.result_size))
        );
    }

    fn get_selected_patch(&self) -> ImgBuf<Rgba> {
        let range = self.selected_range();

        if range.width() > 0 && range.height() > 0 {
            let patch_size = ImgSize::new(range.width() as usize, range.height() as usize);
            let mut buffer = ImgBuf::<Rgba>::new(patch_size);
            let output_range = buffer.range();
            map_range(&self.image, &mut buffer, range, output_range, |i, _| i);    
            buffer
        } else {
            ImgBuf::<Rgba>::new_init(ImgSize::new(1, 1), [0, 0, 0, 0])
        }        
    }
}

fn load_image(path: &str) -> Result<ImgBuf<Rgba>, String> {
    let buf = convert_err(open(path))?.into_rgba();
    let size = ImgSize::new(buf.width() as usize, buf.height() as usize);
    Ok(ImgBuf::from_vec(size, bytes_to_rgba(buf.into_vec())))
}

fn bytes_to_rgba(input: Vec<u8>) -> Vec<Rgba> {
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

