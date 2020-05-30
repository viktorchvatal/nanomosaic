use gdk_pixbuf::{Pixbuf, Colorspace};
use std::{cell::RefCell, rc::Rc};
use nanocv::{ImgBuf, ImgSize, Img};
use crate::message::Rgba;

pub fn create_pixbuf(width: usize, height: usize) -> Pixbuf {
    Pixbuf::new(
        Colorspace::Rgb, true, 8, width as i32, height as i32
    ).expect("No enough memory to create pixbuf.")
}

pub fn update_pixbuf(
    image: &ImgBuf<Rgba>,
    pixbuf: Rc<RefCell<Pixbuf>>,    
) {
    if pixbuf_size(pixbuf.clone()) != image.size() {
        pixbuf.replace(create_pixbuf(image.width(), image.height()));
    }    

    copy_rgba_to_pixbuf(image, &pixbuf.borrow());
}

fn pixbuf_size(pixbuf: Rc<RefCell<Pixbuf>>) -> ImgSize {
    let inner: &Pixbuf = &pixbuf.borrow();
    ImgSize::new(inner.get_width() as usize, inner.get_height() as usize)
}

fn copy_rgba_to_pixbuf(image: &ImgBuf<Rgba>, pixbuf: &Pixbuf) {
    let pixbuf_data = unsafe { pixbuf.get_pixels() };
    let (w, h) = (image.width(), image.height());
    let stride = pixbuf.get_rowstride() as usize;

    for line in 0..h {
        let line_pixels = image.line_ref(line);
        let buf_pixels = &mut pixbuf_data[line*stride..line*stride + w*4];

        let mut offset = 0;

        while offset < buf_pixels.len() {
            let pixel = line_pixels[offset/4];
            let target = &mut buf_pixels[offset .. (offset + 4)];
            target[0] = pixel[0];
            target[1] = pixel[1];
            target[2] = pixel[2];
            target[3] = pixel[3];
            offset += 4;
        }        
    }
}

pub fn horizontal_line(pixbuf: &Pixbuf, line: isize) {
    if line >= 0 && line < pixbuf.get_height() as isize {
        let line = line as usize;
        let width = pixbuf.get_width() as usize;
        let pixbuf_data = unsafe { pixbuf.get_pixels() };
        let stride = pixbuf.get_rowstride() as usize;
        let line_pixels = &mut pixbuf_data[line*stride..line*stride + width*4];        
        let mut offset = 0;

        while offset < line_pixels.len() {
            invert_pixel(line_pixels, offset);
            offset += 4;
        }       
    }
}

pub fn vertical_line(pixbuf: &Pixbuf, column: isize) {
    if column >= 0 && column < pixbuf.get_width() as isize {
        let column = column as usize;
        let pixbuf_data = unsafe { pixbuf.get_pixels() };
        let pixel_offset = column*4;
        let stride = pixbuf.get_rowstride() as usize;

        for line in 0..pixbuf.get_height() as usize {
            let offset = line*stride + pixel_offset;
            invert_pixel(pixbuf_data, offset);
        }
    }
}

fn invert_pixel(data: &mut [u8], offset: usize) {
    data[offset + 0] = 255 - data[offset + 0];
    data[offset + 1] = 255 - data[offset + 1];
    data[offset + 2] = 255 - data[offset + 2];
}