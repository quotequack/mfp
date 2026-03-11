use crate::{CodecId,decoders,MfpError};
use image::DynamicImage;

pub fn resolve_decode(codec: CodecId, payload: &[u8]) -> Result<image::DynamicImage, MfpError> {
    let out: Result<image::DynamicImage, MfpError> = match codec {
        // Add your codec here
        CodecId::Png  => decoders::png::decode(payload),
        CodecId::Jpeg => decoders::jpeg::decode(payload),
        CodecId::Bmp  => decoders::bmp::decode(payload),
        CodecId::Qoi => decoders::qoi::decode(payload),
    };
    out
}

pub fn resolve_encode(codec: CodecId, payload: &DynamicImage) -> Result<Vec<u8>, MfpError> {
    let out: Result<Vec<u8>, MfpError> = match codec {
        // Add your codec here
        CodecId::Png  => decoders::png::encode(payload),
        CodecId::Jpeg => decoders::jpeg::encode(payload),
        CodecId::Bmp  => decoders::bmp::encode(payload),
        CodecId::Qoi => decoders::qoi::encode(payload),
    };
    out
}

pub fn resolve_name(codec: String) -> CodecId {
    let codec: CodecId = match codec.to_lowercase().as_str() {
        "png"  => CodecId::Png,
        "jpeg" | "jpg" => CodecId::Jpeg,
        "bmp" | "bitmap"  => CodecId::Bmp,
        "qoi" => CodecId::Qoi,
        other  => panic!("unknown codec: {}", other),
    };
    codec
}

pub fn resolve_byte(byte: u8) -> CodecId {
    let codec: CodecId = match byte {
        0x01 => CodecId::Png,
        0x02 => CodecId::Jpeg,
        0x03 => CodecId::Bmp,
        0x04 => CodecId::Qoi,
        other => Err(MfpError::UnknownCodec(other)).expect("Failed to fail"),
    };
    codec
}