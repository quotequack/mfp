use crate::MfpError;

pub fn decode(data: &[u8]) -> Result<DynamicImage, MfpError> {
    // LOGIC
}

pub fn encode(img: &DynamicImage) -> Result<Vec<u8>, MfpError> {
    // LOGIC
}

// Also: add logic for your codec in: 
// lib.rs
// lines:
// 36 and 27
// header.rs
// lines:
// 9 and 20