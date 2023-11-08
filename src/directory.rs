use std::{io::Read, mem};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Header{
    count: u32,
    start: u32,
    inode_number: u32,
}

#[derive(Deserialize, Debug)]
struct EntryTmp{
    offset: u16,
    inode_offset: i16,
    inode_type: u16,
    name_size: u16,
}

pub(crate) struct Entry{
    pub(crate) inode_number: u32,
    pub(crate) inode_type: u16,
    pub(crate) inode_start: u32,
    pub(crate) inode_offset: u16,
    pub(crate) name: Vec<u8>,
}

impl Entry{
    fn decode(mut r: &mut impl Read, header: &Header) -> Self{
        let tmp: EntryTmp = bincode::deserialize_from(&mut r).unwrap();
        let mut buf = vec![0; tmp.name_size.try_into().unwrap()];
        r.read_exact(&mut buf).unwrap();
        Self{
            inode_number: (header.inode_number as i32 + tmp.inode_offset as i32) as u32,
            inode_type: tmp.inode_type,
            inode_start: header.start,
            inode_offset: tmp.offset,
            name: bincode::deserialize(&buf).unwrap(),
        }
    }
}

pub(crate) fn decode_directory(mut r: &mut impl Read, size: u32) -> Vec<Entry>{
    let mut to_read = size-3;
    let mut entries = Vec::new();
    while size > 0{
        let cur_header: Header = bincode::deserialize_from(&mut r).unwrap();
        to_read -= 12; // Size of Header
        entries.extend((0..cur_header.count+1).map(|_| {
            let ent = Entry::decode(r, &cur_header);
            to_read -= 8 + ent.name.len() as u32; // Size of an directory entry
            ent
        }));
    }
    entries
}