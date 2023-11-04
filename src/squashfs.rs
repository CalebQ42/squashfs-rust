mod test;
mod superblock;

use std::{fs::File, io::{BufReader, Read}, sync::Mutex, time};

use bincode;
use superblock::Superblock;

pub struct Squashfs {
    reader: Mutex<Box<dyn Read>>,
    superblock: Superblock,
    root_inode: InodeRef,
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
        }else if (superblock.block_size as f32).log2() != (superblock.block_log as f32){
            panic!("Block size and block log don't match. Archive is probably corrupted.");
        }
        let root_inode = InodeRef::parse(superblock.root_ref);
        match superblock.compression {
            0 => (),
            _ => panic!("Compression is not supported... yet."),
        }
        Self {
            reader,
            superblock,
            root_inode
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