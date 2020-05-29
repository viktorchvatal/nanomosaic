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

