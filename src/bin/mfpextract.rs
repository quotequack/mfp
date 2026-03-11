use mfp::resolvers::*;
use std::{env, fs};

fn main() {
    let mut args = env::args().skip(1);
    let input  = args.next().expect("usage: mfpextract <input.mfp> <output>");
    let output = args.next().expect("output path required");

    let mfp_bytes = fs::read(&input).expect("could not read file");
    let codec = resolve_byte(mfp_bytes[4]);
    let decoded = mfp::decode(&mfp_bytes).expect("could not decode mfp");
    let encoded = resolve_encode(codec, &decoded).expect("failed to encode");
    
    println!("{}",&output);
    let _ = fs::write(output, encoded).expect("failed to save");
}