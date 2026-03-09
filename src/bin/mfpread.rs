use std::env;
use std::fs;
use std::process::Command;

fn main() {
    let path = env::args().nth(1).expect("usage: mfpopen <file.mfp>");

    let mfp_bytes = fs::read(&path).expect("could not read file");

    let image = mfp::decode(&mfp_bytes).expect("could not decode mfp");

    let tmp = std::env::temp_dir().join("mfpopen_preview.png");
    image.save(&tmp).expect("could not save temp png");

    Command::new("xdg-open")
        .arg(&tmp)
        .spawn()
        .expect("xdg-open failed");
}