use std::{io::Read, vec};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct InodeHeader{
    pub(crate) inode_type: u16,
    pub(crate) permissions: u16,
    pub(crate) uid_index: u16,
    pub(crate) gid_index: u16,
    pub(crate) mod_time: u32,
    pub(crate) number: u32,
}

#[derive(Deserialize, Debug)]
pub(crate) struct DirInode{
    pub(crate) block_start: u32,
    pub(crate) link_count: u16,
    pub(crate) size: u32,
    pub(crate) block_offset: u16,
    pub(crate) parent_inode: u32,
}

#[derive(Deserialize, Debug)]
struct ExtDirTmp{
    link_count: u32,
    size: u32,
    block_start: u32,
    parent_inode: u32,
    index_count: u16,
    block_offset: u16,
    xattr_index: u32,
}

#[derive(Deserialize, Debug)]
pub(crate) struct ExtDirInode{
    pub(crate) link_count: u32,
    pub(crate) size: u32,
    pub(crate) block_start: u32,
    pub(crate) parent_inode: u32,
    pub(crate) index_count: u16,
    pub(crate) block_offset: u16,
    pub(crate) xattr_index: u32,
    pub(crate) dir_indexes: Vec<DirIndex>,
}

impl ExtDirInode{
    pub(crate) fn decode(mut r: &mut impl Read) -> Self{
        let tmp: ExtDirTmp = bincode::deserialize_from(&mut r).unwrap();
        let dir_indexes = (0..tmp.index_count.into()).map(|_| DirIndex::decode(r)).collect();
        Self{
            link_count: tmp.link_count,
            size: tmp.size,
            block_start: tmp.block_start,
            parent_inode: tmp.parent_inode,
            index_count: tmp.index_count,
            block_offset: tmp.block_offset,
            xattr_index: tmp.xattr_index,
            dir_indexes,
        }
    }
}

#[derive(Deserialize, Debug)]
struct DirIndexTmp{
    index: u32,
    offset: u32,
    name_size: u32,
}

#[derive(Deserialize, Debug)]
pub(crate) struct DirIndex{
    pub(crate) index: u32,
    pub(crate) offset: u32,
    pub(crate) name_size: u32,
    pub(crate) name: Vec<u8>,
}

impl DirIndex{
    pub(crate) fn decode(mut r: &mut impl Read) -> Self{
        let tmp: DirIndexTmp = bincode::deserialize_from(&mut r).unwrap();
        let mut buf = vec![0; tmp.name_size.try_into().unwrap()];
        r.read_exact(&mut buf).unwrap();
        let name = bincode::deserialize(&buf).unwrap();
        Self{
            index: tmp.index,
            offset: tmp.offset,
            name_size: tmp.name_size,
            name,
        }
    }
}

#[derive(Deserialize, Debug)]
struct FileTmp{
    block_start: u32,
    frag_index: u32,
    block_offset: u32,
    size: u32,
}

#[derive(Deserialize, Debug)]
pub(crate) struct FileInode{
    pub(crate) block_start: u32,
    pub(crate) frag_index: u32,
    pub(crate) block_offset: u32,
    pub(crate) size: u32,
    pub(crate) block_sizes: Vec<u32>,
}

impl FileInode{
    pub(crate) fn decode(mut r: &mut impl Read, block_size: u32) -> Self{
        let tmp: FileTmp = bincode::deserialize_from(&mut r).unwrap();
        let mut block_sizes_len = tmp.size / block_size;
        if tmp.frag_index == 0xFFFFFFFF && tmp.size % block_size != 0 {
            block_sizes_len += 1;
        }
        let mut buf = vec![0; (block_sizes_len * 4).try_into().unwrap()];
        r.read_exact(&mut buf).unwrap();
        let block_sizes = bincode::deserialize(&buf).unwrap();
        Self{
            block_start: tmp.block_start,
            frag_index: tmp.frag_index,
            block_offset: tmp.block_offset,
            size: tmp.size,
            block_sizes,
        }
    }
}

#[derive(Deserialize, Debug)]
struct ExtFileTmp{
    block_start: u64,
    size: u64,
    sparse: u64,
    link_count: u32,
    frag_index: u32,
    block_offset: u32,
    xattr_index: u32,
}

#[derive(Deserialize, Debug)]
pub(crate) struct ExtFileInode{
    pub(crate) block_start: u64,
    pub(crate) size: u64,
    pub(crate) sparse: u64,
    pub(crate) link_count: u32,
    pub(crate) frag_index: u32,
    pub(crate) block_offset: u32,
    pub(crate) xattr_index: u32,
    pub(crate) block_sizes: Vec<u32>,
}

impl ExtFileInode{
    pub(crate) fn decode(mut r: &mut impl Read, block_size: u32) -> Self{
        let tmp: ExtFileTmp = bincode::deserialize_from(&mut r).unwrap();
        let mut block_sizes_len: u64 = tmp.size / block_size as u64;
        if tmp.frag_index == 0xFFFFFFFF && tmp.size % block_size as u64 != 0 {
            block_sizes_len += 1;
        }
        let mut buf = vec![0; (block_sizes_len * 4).try_into().unwrap()];
        r.read_exact(&mut buf).unwrap();
        let block_sizes = bincode::deserialize(&buf).unwrap();
        Self{
            block_start: tmp.block_start,
            size: tmp.size,
            sparse: tmp.sparse,
            link_count: tmp.link_count,
            frag_index: tmp.frag_index,
            block_offset: tmp.block_offset,
            xattr_index: tmp.xattr_index,
            block_sizes,
        }
    }
}

#[derive(Deserialize, Debug)]
struct SymTmp{
    link_count: u32,
    path_size: u32,
}

#[derive(Deserialize, Debug)]
pub(crate) struct SymInode{
    pub(crate) link_count: u32,
    pub(crate) path_size: u32,
    pub(crate) path: Vec<u8>,
}

impl SymInode{
    pub(crate) fn decode(mut r: &mut impl Read) -> Self{
        let tmp: SymTmp = bincode::deserialize_from(&mut r).unwrap();
        let mut buf = vec![0; tmp.path_size.try_into().unwrap()];
        r.read_exact(&mut buf).unwrap();
        let path = bincode::deserialize(&buf).unwrap();
        Self{
            link_count: tmp.link_count,
            path_size: tmp.path_size,
            path,
        }
    }
}


#[derive(Deserialize, Debug)]
pub(crate) struct ExtSymInode{
    pub(crate) link_count: u32,
    pub(crate) path_size: u32,
    pub(crate) path: Vec<u8>,
    pub(crate) xattr_index: u32,
}

impl ExtSymInode{
    pub(crate) fn decode(mut r: &mut impl Read) -> Self{
        let tmp: SymTmp = bincode::deserialize_from(&mut r).unwrap();
        let mut buf = vec![0; tmp.path_size.try_into().unwrap()];
        r.read_exact(&mut buf).unwrap();
        let path = bincode::deserialize(&buf).unwrap();
        let xattr_index = bincode::deserialize_from(r).unwrap();
        Self{
            link_count: tmp.link_count,
            path_size: tmp.path_size,
            path,
            xattr_index,
        }
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct DevInode{
    pub(crate) link_count: u32,
    pub(crate) dev_number: u32,
}

#[derive(Deserialize, Debug)]
pub(crate) struct ExtDevInode{
    pub(crate) link_count: u32,
    pub(crate) dev_number: u32,
    pub(crate) xattr_index: u32,
}

#[derive(Deserialize, Debug)]
pub(crate) struct IPCInode{
    pub(crate) link_count: u32,
}

#[derive(Deserialize, Debug)]
pub(crate) struct ExtIPCInode{
    pub(crate) link_count: u32,
    pub(crate) xattr_index: u32,
}