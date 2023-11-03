use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct Superblock{
    pub(crate) magic: u32,
    pub(crate) inode_count: u32,
    pub(crate) mod_time: u32,
    pub(crate) block_size:u32,
    pub(crate) frag_count: u32,
    pub(crate) compression: u32,
    pub(crate) block_log: u16,
    pub(crate) flags: u16,
    pub(crate) id_count: u16,
    pub(crate) ver_maj: u16,
    pub(crate) ver_min: u16,
    pub(crate) root_ref: u64,
    pub(crate) size: u64,
    pub(crate) id_start: u64,
    pub(crate) xattr_start: u64,
    pub(crate) inode_start: u64,
    pub(crate) dir_start: u64,
    pub(crate) frag_start: u64,
    pub(crate) export_start: u64,
}