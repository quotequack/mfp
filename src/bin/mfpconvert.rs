use std::env;
use std::fs;
use mfp::header::CodecId;
use mfp::decoders;

fn main() {
    let mut args = env::args().skip(1);
    let input  = args.next().expect("usage: mfp convert <input> <codec> <output.mfp>");
    let codec  = args.next().expect("codec required");
    let output = args.next().expect("output path required");

    let img = image::open(&input).expect("could not open image");

    let codec: CodecId = match codec.to_lowercase().as_str() {
        "png"  => CodecId::Png,
        "jpeg" | "jpg" => CodecId::Jpeg,
        "bmp" | "bitmap"  => CodecId::Bmp,
        "qoi" => CodecId::Qoi,
        other  => panic!("unknown codec: {}", other),
    };

    let encoded = match codec {
        CodecId::Png  => decoders::png::encode(&img).expect("failed to encode"),
        CodecId::Jpeg => decoders::jpeg::encode(&img).expect("failed to encode"),
        CodecId::Bmp  => decoders::bmp::encode(&img).expect("failed to encode"),
        CodecId::Qoi => decoders::qoi::encode(&img).expect("failed to encode"),
    };

    fs::write(&output, encoded).expect("could not write output");

    println!("saved {}", output);
}