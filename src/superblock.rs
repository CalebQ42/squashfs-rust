use serde::Deserialize;

static magic: u32 = 0x73717368;

#[derive(Debug, Deserialize)]
pub(crate) struct Superblock{
    pub(crate) magic: u32,
    pub(crate) inode_count: u32,
    pub(crate) mod_time: u32,
    pub(crate) block_size: u32,
    pub(crate) frag_count: u32,
    pub(crate) compressor: u16,
    pub(crate) block_log: u16,
    pub(crate) flags: u16,
    pub(crate) id_count: u16,
    pub(crate) ver_maj: u16,
    pub(crate) ver_min: u16,
    pub(crate) root_inode: u64,
    pub(crate) bytes_used: u64,
    pub(crate) id_table_start: u64,
    pub(crate) xattr_table_start: u64,
    pub(crate) inode_table_start: u64,
    pub(crate) dir_table_start: u64,
    pub(crate) frag_table_start: u64,
    pub(crate) export_table_start: u64,
}

impl Superblock{
    pub(crate) fn valid_magic(&self) -> bool{
        self.magic == magic
    }
    pub(crate) fn valid_block_log(&self) -> bool{
        (self.block_size as f32).log2().floor() as u16 == self.block_log
    }
    pub(crate) fn valid_version(&self) -> bool{
        self.ver_maj == 4 && self.ver_min == 0
    }
}