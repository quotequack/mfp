pub mod decoders;
pub mod header;

use header::{CodecId, Header, HEADER_SIZE};
use image::{DynamicImage, error};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MfpError {
    #[error("invalid magic bytes")]
    InvalidMagic,
    #[error("unknown codec id: {0:#x}")]
    UnknownCodec(u8),
    #[error("unexpected end of data")]
    UnexpectedEof,
    #[error("decode error: {0}")]
    DecodeError(#[from] image::ImageError),
    #[error("payload len mismatch: expecter {expected}, got {actual}")]
    InvalidPayloadLen { expected: usize, actual: usize },
}

pub fn decode(data: &[u8]) -> Result<DynamicImage, MfpError> {
    let header = Header::parse(&data[..HEADER_SIZE.min(data.len())])?;
    let payload = &data[HEADER_SIZE..];

    match header.codec {
        CodecId::Png  => decoders::png::decode(payload),
        CodecId::Jpeg => decoders::jpeg::decode(payload),
        CodecId::Bmp  => decoders::bmp::decode(payload),
    }
}

pub fn encode(img: &DynamicImage, codec: CodecId) -> Result<Vec<u8>, MfpError> {
    let payload = match codec {
        CodecId::Png  => decoders::png::encode(img)?,
        CodecId::Jpeg => decoders::jpeg::encode(img)?,
        CodecId::Bmp  => decoders::bmp::encode(img)?,
    };

    let header = Header {
        codec,
        payload_len: payload.len() as u32,
    };

    let mut out = header.to_bytes().to_vec();
    out.extend_from_slice(&payload);
    Ok(out)
}