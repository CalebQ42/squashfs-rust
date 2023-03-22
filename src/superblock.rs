use std::io::Read;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct Superblock{
    magic: u32,
    inode_count: u32,
    mod_time: u32,
    block_size:u32,
    frag_count: u32,
    compression: u32,
    block_log: u16,
    flags: u16,
    id_count: u16,
    ver_maj: u16,
    ver_min: u16,
    root_ref: u64,
    size: u64,
    id_start: u64,
    xattr_start: u64,
    inode_start: u64,
    dir_start: u64,
    frag_start: u64,
    export_start: u64,
}

pub(crate) fn read_from(rdr: &mut dyn Read) -> Superblock{
    bincode::deserialize_from(rdr).unwrap()
}