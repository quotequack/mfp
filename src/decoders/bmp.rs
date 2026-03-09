use image::{DynamicImage, ImageFormat};
use crate::MfpError;

pub fn decode(data: &[u8]) -> Result<DynamicImage, MfpError> {
    image::load_from_memory_with_format(data, ImageFormat::Bmp)
        .map_err(MfpError::DecodeError)
}

pub fn encode(img: &DynamicImage) -> Result<Vec<u8>, MfpError> {
    let mut buf = std::io::Cursor::new(Vec::new());
    img.write_to(&mut buf, ImageFormat::Bmp)
        .map_err(MfpError::DecodeError)?;
    Ok(buf.into_inner())
}