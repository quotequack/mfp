use crate::MfpError;

pub const MAGIC: [u8; 4] = [0x4D, 0x46, 0x50, 0x00]; 
pub const HEADER_SIZE: usize = 9;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]

// Add new codecID here:
pub enum CodecId {
    Png  = 0x01,
    Jpeg = 0x02,
    Bmp  = 0x03,
}

impl TryFrom<u8> for CodecId {
    type Error = MfpError;
    fn try_from(byte: u8) -> Result<Self, MfpError> {
        match byte {
            // Append new codecID here:
            0x01 => Ok(CodecId::Png),
            0x02 => Ok(CodecId::Jpeg),
            0x03 => Ok(CodecId::Bmp),
            other => Err(MfpError::UnknownCodec(other)),
        }
    }
}

pub struct Header {
    pub codec: CodecId,
    pub payload_len: u32,
}

impl Header {
    // Parse file
    pub fn parse(data: &[u8]) -> Result<Self, MfpError> {
        if data.len() < HEADER_SIZE {
            return Err(MfpError::UnexpectedEof);
        }
        if data[0..4] != MAGIC {
            return Err(MfpError::InvalidMagic);
        }
        let codec = CodecId::try_from(data[4])?;
        let payload_len = u32::from_le_bytes(data[5..9].try_into().unwrap());
        Ok(Header { codec, payload_len })
    }

    // Package to file
    pub fn to_bytes(&self) -> [u8; HEADER_SIZE] {
        let mut out = [0u8; HEADER_SIZE];
        out[0..4].copy_from_slice(&MAGIC);
        out[4] = self.codec as u8;
        out[5..9].copy_from_slice(&self.payload_len.to_le_bytes());
        out
    }
}