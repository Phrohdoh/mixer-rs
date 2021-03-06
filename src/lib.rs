extern crate byteorder;
use byteorder::{LittleEndian, ReadBytesExt};

use std::io::{Read, Seek, SeekFrom, BufReader};
use std::path::Path;
use std::fs::File;

extern crate crc;
use crc::{crc32, Hasher32};

pub enum HashType {
    Crc32,
    Custom,
}

struct MixHeader {
    num_files: u16,
    header_end_offset: u64,
}

#[derive(Debug)]
pub struct EntryHeader {
    pub hash: u64,
    pub offset: u32,
    pub len: u32,
}

pub fn read_entry_headers<P: AsRef<Path> + std::fmt::Debug>(file_path: P) -> Vec<EntryHeader> {
    let f = File::open(&file_path).unwrap();
    let br = BufReader::new(f);
    read(br).unwrap()
}

pub fn calc_hash_of(string: &str, hash_type: HashType) -> u64 {
    match hash_type {
        HashType::Crc32 => {
            crc32::checksum_ieee(string.as_bytes()) as u64
        },
        HashType::Custom => {
            let upper = {
                let mut u = string.to_uppercase();
                if u.len() % 4 != 0 {
                    u = format!("{:\0<more$}", u, more = u.len() + (4 - u.len() % 4));
                }

                u
            };

            let mut len = upper.len() >> 2;
            let mut res: u64 = 0;

            let bytes = upper.as_bytes();
            let mut rdr = BufReader::new(bytes);

            while (len != 0) {
                len -= 1;
                res = ((res << 1) | (res >> 31)) + (rdr.read_u32::<LittleEndian>().unwrap() as u64);
            }

            res
        }
    }
}

fn read<T: Read + Seek>(mut data: T) -> Result<Vec<EntryHeader>, std::io::Error> {
    let is_cnc_mix = data.read_u16::<LittleEndian>()? != 0;

    let mut is_encrypted = false;

    if !is_cnc_mix {
        is_encrypted = (data.read_u16::<LittleEndian>()? & 0x2) != 0;
    }

    let mix_header = if is_encrypted {
        panic!("TODO: encrypted mix!");
    } else {
        let header_offset = if is_cnc_mix { 0 } else { 4 };
        read_header(&mut data, header_offset)?
    };

    let mut entry_headers = Vec::new();
    for i in 0..mix_header.num_files {
        entry_headers.push(read_entry_header(&mut data)?);
    }

    Ok(entry_headers)
}

fn read_header<T: Read + Seek>(data: &mut T, abs_offset: u64) -> Result<MixHeader, std::io::Error> {
    let _ = data.seek(SeekFrom::Start(abs_offset))?;
    let num_files = data.read_u16::<LittleEndian>()?;
    let _ = data.seek(SeekFrom::Current(4));

    let header_end_offset = abs_offset + 6 + (num_files as u64 * 12);
    Ok(MixHeader { num_files, header_end_offset })
}

fn read_entry_header<T: Read + Seek>(data: &mut T) -> Result<EntryHeader, std::io::Error> {
    let hash = data.read_u32::<LittleEndian>()? as u64;
    let offset = data.read_u32::<LittleEndian>()?;
    let len = data.read_u32::<LittleEndian>()?;

    Ok(EntryHeader { hash, offset, len })
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(0, 0);
    }
}
