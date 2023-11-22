mod metadata;
mod decompress;
mod superblock;

use std::{io::{Read, Seek}, sync::Mutex};

use decompress::Decompress;
use superblock::Superblock;

pub struct Squashfs<'a, R: Read+Seek>{
    rdr: Mutex<&'a mut R>,
    sup: Superblock,
    decomp: Box<dyn Decompress>,
}

impl<'a, R: Read+Seek> Squashfs<'a, R>{
    pub fn new(mut rdr: &'a mut R) -> Self{
        let sup: Superblock = bincode::deserialize_from(&mut rdr).unwrap();
        if !sup.valid_magic(){
            panic!("Invalid magic. Is this a squashfs archive?");
        }else if !sup.valid_block_log(){
            panic!("Block log does not match block size. Archive may be corrupted.");
        }else if !sup.valid_version(){
            panic!("Unsupported squashfs version. Archive may be corrupted.");
        }
        let decomp: Box<dyn Decompress> = match sup.compressor{
            0 => Box::new(decompress::Passthrough{}),
            1 => Box::new(decompress::Zlib{}),
            2 => Box::new(decompress::XzLzma{}),
            3 => Box::new(decompress::Lzo{
                max_size: sup.block_size
            }),
            4 => Box::new(decompress::XzLzma{}),
            5 => Box::new(decompress::Lz4{}),
            6 => Box::new(decompress::Zstd{}),
            _ => panic!("Unsupported compressor. Archive may be corrupted."),
        };
        Self{
            rdr: Mutex::new(rdr),
            sup,
            decomp,
        }
    }
}