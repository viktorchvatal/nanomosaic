use nanocv::{ImgBuf, ImgMut, Img};
use crate::message::Rgba;

pub fn draw_full_horizontal_line(image: &mut ImgBuf<Rgba>, pos: isize) {
    if pos >= 0 && pos < image.height() as isize {
        let line = image.line_mut(pos as usize);
        for i in 0..line.len() { line[i] = negative(line[i]) }
    }
}

pub fn draw_full_vertical_line(image: &mut ImgBuf<Rgba>, pos: isize) {
    if pos >= 0 && pos < image.width() as isize {
        for line in 0..image.height() {
            let data = image.line_mut(line);
            data[pos as usize] = negative(data[pos as usize]);
        }
    }
}

fn negative(color: [u8; 4]) -> [u8; 4] {
    [255 - color[0], 255 - color[1], 255 - color[2], color[3]]
}