extern crate mixer;

use std::io::BufReader;
use std::fs::File;
use std::env;

fn main() {
    match env::args().nth(1) {
        None => println!("Please supply a path to a mix file!"),
        Some(path) => {
            let f = File::open(&path).expect(&format!("Failed to open {}", &path));
            let br = BufReader::new(f);
            mixer::read(br).expect(&format!("Failed to read {}", path));
        }
    }
}