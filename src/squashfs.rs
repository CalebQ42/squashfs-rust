mod test;
mod superblock;
mod compressors;

use std::{fs::File, io::{BufReader, Read}, sync::Mutex, time};

use bincode;
use superblock::Superblock;
use compressors::*;

pub struct Squashfs {
    reader: Mutex<Box<dyn Read>>,
    superblock: Superblock,
    root_inode: InodeRef,
    decompressor: Box<dyn Decompress>,
}

impl Squashfs {
    pub fn from_file(path: &str) -> Self {
        Self::from_read(Box::new(BufReader::new(File::open(path).unwrap())))
    }
    
    pub fn from_read(r: Box<dyn Read>) -> Self {
        let reader = Mutex::new(r);
        let superblock: Superblock = bincode::deserialize_from(reader.lock().unwrap().as_mut()).unwrap();
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
            _ => panic!("Compression is not supported... yet."),
        };
        Self {
            reader,
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