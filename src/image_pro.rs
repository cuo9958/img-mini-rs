extern crate image;
/**
 * 图片处理库
 * 根据参数做不同的操作，有不同的方法可以调用
 * 入口统一，参数稳定
 */
// use image::buffer::{EnumeratePixels, Pixels};
use image::imageops::FilterType;
use image::{DynamicImage, GenericImage, ImageBuffer, ImageDecoder, ImageFormat, SubImage};
use std::str;

pub fn image_pipe(from_buf: &[u8], c: &str) -> Vec<u8> {
    match c {
        "zoom" => return zoom_image(from_buf, 100, 100),
        _ => println!("使用默认方法"),
    }
    return image_fn(from_buf);
}

/**
 * 图片处理方法,只需要压缩
 */
fn image_fn(from_buf: &[u8]) -> Vec<u8> {
    println!("压缩图片");
    let img = image::load_from_memory_with_format(from_buf, ImageFormat::Jpeg).unwrap();
    let mut buffer = Vec::new();
    img.write_to(&mut buffer, ImageFormat::Jpeg).unwrap();
    buffer
}

/**
 * 缩放图片
 */
fn zoom_image(from_buf: &[u8], width: u32, height: u32) -> Vec<u8> {
    println!("缩放图片,{},{}", width, height);
    let img = image::load_from_memory_with_format(from_buf, ImageFormat::Jpeg).unwrap();
    let img2 = img.resize(width, height, FilterType::Lanczos3);
    let mut buffer = Vec::new();
    img2.write_to(&mut buffer, ImageFormat::Jpeg).unwrap();
    buffer
}

/**
 * x、y位置处裁剪固定大小图片
 */
// fn cut_image(from_buf: &[u8], width: u32, height: u32, x1: u32, y1: u32) -> Vec<u8> {
//     println!("从左上角{},{}处裁剪", x1, y1);
//     let img = image::load_from_memory_with_format(from_buf, ImageFormat::Jpeg).unwrap();
//     let img2 = img.sub_image(x1, y1, width, height);
//     let mut buffer = Vec::new();
//     img2.write_to(&mut buffer, ImageFormat::Jpeg).unwrap();
//     buffer
// }
