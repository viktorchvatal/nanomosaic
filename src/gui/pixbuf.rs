use gdk_pixbuf::{Pixbuf, Colorspace};

pub fn create_pixbuf(width: usize, height: usize) -> Pixbuf {
    Pixbuf::new(
        Colorspace::Rgb, false, 8, width as i32, height as i32
    ).expect("No enough memory to create pixbuf.")
}
