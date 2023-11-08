mod test;
mod superblock;
mod decompressors;
mod inode;
mod metadata;
mod folder;
mod directory;

use std::{io::{Read, Seek, SeekFrom}, sync::Mutex, time};

use bincode;
use directory::decode_directory;
use folder::Folder;
use inode::Inode;
use metadata::MetadataReader;
use superblock::Superblock;
use decompressors::*;

pub struct Squashfs<R: Read+Seek> {
    reader: Mutex<Box<R>>,
    superblock: Superblock,
    root: Folder,
    decompressor: Box<dyn Decompress>,
}

impl<R: Read+Seek> Squashfs<R> {
    pub fn new(r: R) -> Self {
        let mut r = Box::new(r);
        let superblock: Superblock = bincode::deserialize_from(r.as_mut()).unwrap();
        if superblock.magic != 0x73717368 {
            panic!("Invalid magic number. Are you sure this is a squashfs archive, or is it just corrupted?");
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
        let root = {
            let i = read_inode(r.as_mut(), superblock.root_ref, &superblock, &decompressor);
            folder_from_inode(r.as_mut(), i, &superblock, &decompressor)
        };
        Self {
            reader: Mutex::new(r),
            superblock,
            root,
            decompressor,
        }
    }

    pub fn mod_time(&self) -> time::SystemTime{
        time::SystemTime::UNIX_EPOCH + time::Duration::from_secs(self.superblock.mod_time as u64)
    }

    fn read_inode(&self, reference: u64) -> Inode{
        read_inode(self.reader.lock().unwrap().as_mut(), reference, &self.superblock, &self.decompressor)
    }

    fn folder_from_inode(&self, i: Inode) -> Folder{
        folder_from_inode(self.reader.lock().unwrap().as_mut(), i, &self.superblock, &self.decompressor)
    }
}

fn read_inode<'a, R: Read+Seek>(rdr: &'a mut R, reference: u64, superblock: &'a Superblock, decompressor: &'a Box<dyn Decompress>) -> Inode{
    let start = (reference>>16)+superblock.inode_start;
    let mut reader = MetadataReader::new(rdr, decompressor, start);
    reader.seek(SeekFrom::Current((reference & 0xFFFF) as i64)).unwrap();
    Inode::decode(&mut reader, superblock.block_size)
}

fn folder_from_inode<'a, R: Read+Seek>(rdr: &'a mut R, i: Inode, superblock: &'a Superblock, decompressor: &'a Box<dyn Decompress>) -> Folder{
    let entries: Vec<directory::Entry>;
    if i.header.inode_type == inode::DIR_TYPE{
        if let Some(dat) = i.data.downcast_ref::<inode::types::DirInode>(){
            rdr.seek(SeekFrom::Start(dat.block_start as u64 + superblock.dir_start)).unwrap();
            let mut reader = MetadataReader::new(rdr, decompressor, dat.block_start as u64);
            reader.seek(SeekFrom::Current(dat.block_offset as i64)).unwrap();
            entries = decode_directory(&mut reader, dat.size);
        }else{
            panic!("Not actually a DirInode!");
        }
    }else if i.header.inode_type == inode::EXT_DIR_TYPE{
        if let Some(dat) = i.data.downcast_ref::<inode::types::ExtDirInode>(){
            rdr.seek(SeekFrom::Start(dat.block_start as u64 + superblock.dir_start)).unwrap();
            let mut reader = MetadataReader::new(rdr, decompressor, dat.block_start as u64);
            reader.seek(SeekFrom::Current(dat.block_offset as i64)).unwrap();
            entries = decode_directory(&mut reader, dat.size);
        }else{
            panic!("Not actually a ExtDirInode!");
        }
    }else{
        panic!("Cannot decode non-directory inode into a folder")
    }
    Folder{
        inode: i,
        entries,
    }
}