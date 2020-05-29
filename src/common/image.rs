use nanocv::{ImgBuf, ImgSize, Img};
use nanocv::filter::{resize_nearest_new};
use crate::message::Rgba;

pub fn resize(source: &ImgBuf<Rgba>, target_size: ImgSize) -> ImgBuf<Rgba> {
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