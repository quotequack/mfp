pub mod decoders;
pub mod header;
pub mod resolvers;

use header::{CodecId, Header, HEADER_SIZE};
use image::DynamicImage;
use thiserror::Error;
use resolvers::*;

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
    #[error("payload len mismatch: expected {expected}, got {actual}")] 
    InvalidPayloadLen { expected: usize, actual: usize },
}

pub fn decode(data: &[u8]) -> Result<DynamicImage, MfpError> {
    let header = Header::parse(&data[..HEADER_SIZE.min(data.len())])?;
    let payload = &data[HEADER_SIZE..];

    if data[HEADER_SIZE..].len() != header.payload_len as usize {
        return Err(
            MfpError::InvalidPayloadLen{
                expected: header.payload_len as usize,
                actual: data[HEADER_SIZE..].len()
            }
        )
    }

    resolve_decode(header.codec, payload)
}

pub fn encode(img: &DynamicImage, codec: CodecId) -> Result<Vec<u8>, MfpError> {
    let payload = resolve_encode(codec, img)?;
    let header = Header {
        codec,
        payload_len: payload.len() as u32,
    };

    let mut out = header.to_bytes().to_vec();
    out.extend_from_slice(&payload);
    Ok(out)
}