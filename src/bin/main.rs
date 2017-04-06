extern crate mixer;

use std::env;

fn main() {
    match env::args().nth(1) {
        None => println!("Please supply a path to a mix file!"),
        Some(path) => {
            let headers = mixer::get_entry_headers(path);
            for hdr in headers {
                println!("{:?}", hdr);
            }
        }
    }
}