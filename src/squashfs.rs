mod test;
mod superblock;
mod compressors;
mod inode;

use std::{io::{Read, Seek}, sync::Mutex, time};

use bincode;
use superblock::Superblock;
use compressors::*;

pub struct Squashfs<R: Read+Seek> {
    reader: Mutex<Box<R>>,
    superblock: Superblock,
    root_inode: InodeRef,
    decompressor: Box<dyn Decompress>,
}

impl<R: Read+Seek> Squashfs<R> {
    pub fn new(r: R) -> Self {
        let mut r = Box::new(r);
        let superblock: Superblock = bincode::deserialize_from(r.as_mut()).unwrap();
        if superblock.magic != 0x73717368 {
            panic!("Invalid magic number. Are you sure this is a squashfs archive?");
        }else if superblock.block_log != ((superblock.block_size as f32).log2() as u16){
            panic!("Block size and block log don't match. Archive is probably corrupted.");
        }else if superblock.ver_maj != 4 || superblock.ver_min != 0{
            panic!("Unsupported squashfs version: {:?}.{:?}, or archive is corrupted", superblock.ver_maj, superblock.ver_min);
        }
        let root_inode = InodeRef::parse(superblock.root_ref);
        let decompressor: Box<dyn Decompress> = match superblock.compression{
            0 => Box::new(Passthrough{}),
            1 => Box::new(ZlibDecomp{}),
            2 => Box::new(XzLzmaDecomp{}),
            3 => panic!("LZO compression is not supported... yet."),
            4 => Box::new(XzLzmaDecomp{}),
            5 => Box::new(Lz4Decomp{}),
            6 => Box::new(ZstdDecomp{}),
            _ => panic!("Invalid compression type: {}", superblock.compression),
        };
        Self {
            reader: Mutex::new(r),
            superblock,
            root_inode,
            decompressor,
        }
    }

    pub fn mod_time(&self) -> time::SystemTime{
        time::SystemTime::UNIX_EPOCH + time::Duration::from_secs(self.superblock.mod_time as u64)
    }
}

struct InodeRef{
    start: u64,
    offset: u64,
}

impl InodeRef{
    fn parse(reference: u64) -> Self{
        Self{start: reference >> 16, offset: reference & 0xFFFFFFFFFFFF0000}
    }
}