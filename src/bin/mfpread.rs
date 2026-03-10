use std::env;
use std::fs;
use std::process::Command;
use tempfile::tempdir;

fn main() {
    let path = env::args().nth(1).expect("usage: mfpread <file.mfp>");
    let mfp_bytes = fs::read(&path).expect("could not read file");
    let image = mfp::decode(&mfp_bytes).expect("could not decode mfp");

    let tmp = tempdir().expect("failed to create temp dir");
    let tmp_path = tmp.path().join("image.png");
    image.save(&tmp_path).expect("could not save temp png");

    let viewer = match std::env::consts::OS {
        "linux" | "freebsd" | "openbsd" | "netbsd" => "xdg-open",
        "macos" => "open",
        "windows" => "start",
        _ => panic!("unsupported OS"),
    };

    Command::new(viewer)
        .arg(&tmp_path)
        .spawn().expect("failed to open")
        .wait().expect("failed to wait");
}