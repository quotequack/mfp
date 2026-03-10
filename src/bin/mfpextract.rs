use mfp::decoders;
use std::{fs,env};

fn main() {
    let path = env::args().nth(1).expect("usage: mfpextract <file.mfp>");
    let mfp_bytes = fs::read(&path).expect("could not read file");
    let codec = match mfp_bytes[4] {
        0x01 => ".png",
        0x02 => ".jpg",
        0x03 => ".bmp",
        0x04 => ".qoi",
        other => Err(mfp::MfpError::UnknownCodec(other)).expect("Failed to fail"),
    };
    let decoded = mfp::decode(&mfp_bytes).expect("could not decode mfp");
    let encoded = match codec {
        ".png"  => decoders::png::encode(&decoded).expect("failed to encode"),
        ".jpg" => decoders::jpeg::encode(&decoded).expect("failed to encode"),
        ".bmp"  => decoders::bmp::encode(&decoded).expect("failed to encode"),
        ".qoi" => decoders::qoi::encode(&decoded).expect("failed to encode"),
        _ => Err(mfp::MfpError::UnknownCodec(mfp_bytes[4])).expect("Failed to fail")
    };
    
    let path = format!("./extract{}",codec);
    println!("{}",&path);
    let _ = fs::write(path, encoded);
}