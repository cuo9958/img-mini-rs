extern crate image;

use image::buffer::{EnumeratePixels, Pixels};
use image::imageops::FilterType;
use image::{GenericImage, ImageBuffer, ImageFormat};

/**
 * 图片处理方法
 */
pub fn image_fn(from_buf: &[u8]) -> Vec<u8> {
    let img = image::load_from_memory_with_format(from_buf, ImageFormat::Jpeg).unwrap();
    let mut buffer = Vec::new();
    img.write_to(&mut buffer, ImageFormat::Jpeg).unwrap();
    buffer
}

//缩放图片
fn zoom_image(from_buf: &[u8], width: u32, height: u32) -> Vec<u8> {
    let img = image::load_from_memory_with_format(from_buf, ImageFormat::Jpeg).unwrap();

    let img2 = img.resize(width, height, FilterType::Lanczos3);
    let mut buffer = Vec::new();
    img.write_to(&mut buffer, ImageFormat::Jpeg).unwrap();
    buffer
}
