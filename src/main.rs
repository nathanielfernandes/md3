use std::fs::read;

use md3::id3v1::ID3v1;
use md3::id3v2::ID3v2;

fn main() {
    let bytes = read("therecipe.mp3").unwrap();

    let id3v1 = ID3v2::from(&bytes).unwrap();

    println!("Title: {:?}", id3v1);
}
