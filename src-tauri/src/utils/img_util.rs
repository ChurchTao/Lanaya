use super::string_util;
use anyhow::Result;
use arboard::ImageData;
use image::ImageEncoder;
use std::io::{BufReader, BufWriter, Cursor};

pub fn rgba8_to_base64(img: &ImageData) -> String {
    let mut bytes: Vec<u8> = Vec::new();
    image::codecs::png::PngEncoder::new(BufWriter::new(Cursor::new(&mut bytes)))
        .write_image(
            &img.bytes,
            img.width as u32,
            img.height as u32,
            image::ColorType::Rgba8,
        )
        .unwrap();
    string_util::base64_encode(bytes.as_slice())
}

pub fn base64_to_rgba8(base64: &str) -> Result<ImageData> {
    let bytes = string_util::base64_decode(base64);
    let reader =
        image::io::Reader::with_format(BufReader::new(Cursor::new(bytes)), image::ImageFormat::Png);
    match reader.decode() {
        Ok(img) => {
            let rgba = img.into_rgba8();
            let (width, height) = rgba.dimensions();
            Ok(ImageData {
                width: width as usize,
                height: height as usize,
                bytes: rgba.into_raw().into(),
            })
        }
        Err(_) => Err(anyhow::anyhow!("decode image error")),
    }
}
