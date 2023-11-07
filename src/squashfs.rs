mod test;
mod superblock;
mod decompressors;
mod inode;
mod metadata;

use std::{io::{Read, Seek, SeekFrom}, sync::Mutex, time};

use bincode;
use inode::Inode;
use metadata::MetadataReader;
use superblock::Superblock;
use decompressors::*;

pub struct Squashfs<R: Read+Seek> {
    reader: Mutex<Box<R>>,
    superblock: Superblock,
    root_inode: Inode,
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
        let reader = Mutex::new(r);
        let root_inode = {
            let mut root_inode_reader = MetadataReader::new(&reader, &decompressor, (superblock.root_ref>>16) + superblock.inode_start);
            root_inode_reader.seek(SeekFrom::Current((superblock.root_ref & 0xFFFF) as i64)).unwrap();
            Inode::decode(&mut root_inode_reader, superblock.block_size)
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

    fn read_inode(&self, reference: u64) -> Inode{
        let start = reference>>16+self.superblock.inode_start;
        let mut reader = MetadataReader::new(&self.reader, &self.decompressor, start);
        reader.seek(SeekFrom::Current((reference & 0xFFFF) as i64)).unwrap();
        Inode::decode(&mut reader, self.superblock.block_size)
    }
}