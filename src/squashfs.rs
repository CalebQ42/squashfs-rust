use std::{fs::File, io::{BufReader, Read}, sync::Mutex};

use bincode;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Superblock {
    magic: u32,
}

pub struct Squashfs {
    reader: Mutex<Box<dyn Read>>,
    superblock: Superblock,
}

impl Squashfs {
    pub fn from_file(path: &str) -> Self {
        Self::from_read(Box::new(BufReader::new(File::open(path).unwrap())))
    }
    
    pub fn from_read(r: Box<dyn Read>) -> Self {
        let reader = Mutex::new(r);
        let superblock: Superblock = bincode::deserialize_from(reader.lock().unwrap().as_mut()).unwrap();
        Squashfs {
            reader,
            superblock,
        }
    }
}