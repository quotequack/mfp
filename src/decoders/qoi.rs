use crate::MfpError;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb, RgbaImage};
use qoi::{encode_to_vec,decode_to_vec};

pub fn decode(data: &[u8]) -> Result<DynamicImage, MfpError> {
    let (header, decoded) = decode_to_vec(&data)
      .expect("invalid qoi data");
    let img: RgbaImage = ImageBuffer::from_raw(header.width, header.height, decoded)
      .expect("buffer mismatch");
    Ok(DynamicImage::ImageRgba8(img))
}

pub fn encode(img: &DynamicImage) -> Result<Vec<u8>, MfpError> {
    let width = img.width();
    let height = img.height();
    let data = ImageBuffer::into_raw(img.clone().into_rgba8());
    Ok(encode_to_vec(&data, width, height).expect("failed to encode qoi"))
}