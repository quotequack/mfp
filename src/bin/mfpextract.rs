use mfp::{decoders, header::CodecId};
use std::{env, fs};

fn main() {
    let mut args = env::args().skip(1);
    let input  = args.next().expect("usage: mfpextract <input.mfp> <output>");
    let output = args.next().expect("output path required");


    let mfp_bytes = fs::read(&input).expect("could not read file");
    let codec = match mfp_bytes[4] {
        0x01 => CodecId::Png,
        0x02 => CodecId::Jpeg,
        0x03 => CodecId::Bmp,
        0x04 => CodecId::Qoi,
        other => Err(mfp::MfpError::UnknownCodec(other)).expect("Failed to fail"),
    };
    let decoded = mfp::decode(&mfp_bytes).expect("could not decode mfp");
    let encoded = match codec {
        CodecId::Png  => decoders::png::encode(&decoded).expect("failed to encode"),
        CodecId::Jpeg => decoders::jpeg::encode(&decoded).expect("failed to encode"),
        CodecId::Bmp  => decoders::bmp::encode(&decoded).expect("failed to encode"),
        CodecId::Qoi => decoders::qoi::encode(&decoded).expect("failed to encode"),
    };
    
    println!("{}",&output);
    let _ = fs::write(output, encoded).expect("failed to save");
}