use std::env;
use std::fs;
use mfp::header::CodecId;

fn main() {
    let mut args = env::args().skip(1);
    let input  = args.next().expect("usage: mfpcreate <input> <codec> <output.mfp>");
    let codec  = args.next().expect("codec required: png, jpeg, bmp");
    let output = args.next().expect("output path required");

    let codec = match codec.to_lowercase().as_str() {
        "png"  => CodecId::Png,
        "jpeg" | "jpg" => CodecId::Jpeg,
        "bmp"  => CodecId::Bmp,
        other  => panic!("unknown codec: {}", other),
    };

    let img = image::open(&input).expect("could not open image");
    let mfp_bytes = mfp::encode(&img, codec).expect("could not encode mfp");
    fs::write(&output, mfp_bytes).expect("could not write output");

    println!("saved {}", output);
}