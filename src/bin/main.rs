extern crate mixer;

use std::env;

fn main() {
    match env::args().nth(1) {
        None => println!("Please supply a path to a mix file!"),
        Some(path) => {
            let headers = mixer::read_entry_headers(path);

            let local_db_hash = mixer::calc_hash_of("local mix database.dat", mixer::HashType::Custom);
            println!("found {} -> {}", local_db_hash, headers.iter().find(|h| h.hash == local_db_hash).is_some());

            let e1_dot_shp_hash_classic = mixer::calc_hash_of("e1.shp", mixer::HashType::Custom);
            println!("found {} -> {}", e1_dot_shp_hash_classic, headers.iter().find(|h| h.hash == e1_dot_shp_hash_classic).is_some());

            let e1_dot_shp_hash_crc32 = mixer::calc_hash_of("e1.shp", mixer::HashType::Crc32);
            println!("found {} -> {}", e1_dot_shp_hash_crc32, headers.iter().find(|h| h.hash == e1_dot_shp_hash_crc32).is_some());
        }
    }
}