extern crate byteorder;
use byteorder::{LittleEndian, ReadBytesExt};

use std::io::{Read, Seek, SeekFrom};

struct MixHeader {
    num_files: u16,
    header_end_offset: u64,
}

struct EntryHeader {
    hash: u32,
    offset: u32,
    len: u32,
}

pub fn read<T: Read + Seek>(mut data: T) -> Result<(), std::io::Error> {
    let is_cnc_mix = data.read_u16::<LittleEndian>()? != 0;
    println!("is_cnc_mix: {}", is_cnc_mix);

    let mut is_encrypted = false;

    if !is_cnc_mix {
        is_encrypted = (data.read_u16::<LittleEndian>()? & 0x2) != 0;
    }

    println!("is_encrypted: {}", is_encrypted);

    let mh = if is_encrypted {
        panic!("TODO: encrypted mix!");
    } else {
        let header_offset = if is_cnc_mix { 0 } else { 4 };
        read_header(&mut data, header_offset)?
    };

    println!("num_files: {}", mh.num_files);
    println!("header_end_offset: {}", mh.header_end_offset);

    for i in 0..mh.num_files {
        let eh = read_entry_header(&mut data)?;
        println!("[{}] hash: {}", i, eh.hash);
        println!("[{}] offset: {}", i, eh.offset);
        println!("[{}] len: {}", i, eh.len);
        println!("");
    }

    Ok(())
}

fn read_header<T: Read + Seek>(data: &mut T, abs_offset: u64) -> Result<MixHeader, std::io::Error> {
    let _ = data.seek(SeekFrom::Start(abs_offset))?;
    let num_files = data.read_u16::<LittleEndian>()?;
    let _ = data.seek(SeekFrom::Current(4));

    let header_end_offset = abs_offset + 6 + (num_files as u64 * 12);
    Ok(MixHeader { num_files, header_end_offset })
}

fn read_entry_header<T: Read + Seek>(data: &mut T) -> Result<EntryHeader, std::io::Error> {
    let hash = data.read_u32::<LittleEndian>()?;
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
