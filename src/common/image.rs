use nanocv::{ImgBuf, ImgSize, Img};
use nanocv::filter::{resize_nearest_new};
use std::cmp::max;
use crate::message::Rgba;

pub fn resize(source: &ImgBuf<Rgba>, target_size: ImgSize) -> ImgBuf<Rgba> {
    if target_size.x == 0 || target_size.y == 0 || source.size().x == 0 || source.size().y == 0 {
        return ImgBuf::new_init(ImgSize::new(1, 1), [0, 0, 0, 0]);
    }

    let factor = resize_factor(source.size(), target_size);

    let target = ImgSize::new(
        max(1, (source.width() as f64*factor) as usize),
        max(1, (source.height() as f64*factor) as usize),
    );

    resize_nearest_new(source, target)
}

pub fn resize_factor(actual: ImgSize, target: ImgSize) -> f64 {
    if actual.x == 0 || actual.y == 0 {
        return 1.0;
    }

    let factor_x = target.x as f64/actual.x as f64;
    let factor_y = target.y as f64/actual.y as f64;
    let factor = if factor_x > factor_y {factor_y} else {factor_x};

    factor
}