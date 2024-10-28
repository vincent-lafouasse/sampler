#![allow(unused)]

use std::fs;
use std::path::Path;
use std::sync::Arc;

const WAV_PATH_909: &'static str = "./samples/909/";

fn main() {
    let path = Path::new(WAV_PATH_909);
    let path = path.join("kick.wav");
    let kick: Arc<[u8]> = fs::read(path).unwrap().into();
}
