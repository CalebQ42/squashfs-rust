mod test;
mod superblock;
mod inode;
mod metadata;
mod folder;
mod directory;

use std::{io::{Read, Seek, SeekFrom}, sync::Mutex, time, ops::Deref};

use bincode;
use directory::decode_directory;
use flate2::read::ZlibDecoder;
use folder::Folder;
use inode::Inode;
use lzma::LzmaReader;
use metadata::MetadataReader;
use superblock::Superblock;

pub struct Squashfs<R: Read+Seek> {
    reader: Mutex<Box<R>>,
    superblock: Superblock,
    root: Folder,
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
        let root = {
            let i = read_inode(r.as_mut(), superblock.root_ref, &superblock);
            folder_from_inode(r.as_mut(), i, &superblock)
        };
        Self {
            reader: Mutex::new(r),
            superblock,
            root,
        }
    }

    pub fn mod_time(&self) -> time::SystemTime{
        time::SystemTime::UNIX_EPOCH + time::Duration::from_secs(self.superblock.mod_time as u64)
    }

    fn read_inode(&self, reference: u64) -> Inode{
        read_inode(self.reader.lock().unwrap().as_mut(), reference, &self.superblock)
    }

    fn folder_from_inode(&self, i: Inode) -> Folder{
        folder_from_inode(self.reader.lock().unwrap().as_mut(), i, &self.superblock)
    }
}

fn read_inode<'a, R: Read+Seek>(rdr: &'a mut R, reference: u64, superblock: &'a Superblock) -> Inode{
    let start = (reference>>16)+superblock.inode_start;
    let mut reader = MetadataReader::new(rdr, superblock.compression);
    reader.read_exact(&mut vec![0u8; (reference & 0xFFFF) as usize]).unwrap();
    Inode::decode(&mut reader, superblock.block_size)
}

fn folder_from_inode<'a, R: Read+Seek>(rdr: &'a mut R, i: Inode, superblock: &'a Superblock) -> Folder{
    let entries: Vec<directory::Entry>;
    if i.header.inode_type == inode::DIR_TYPE{
        if let Some(dat) = i.data.downcast_ref::<inode::types::DirInode>(){
            rdr.seek(SeekFrom::Start(dat.block_start as u64 + superblock.dir_start)).unwrap();
            let mut reader = MetadataReader::new(rdr, superblock.compression);
            reader.read_exact(&mut vec![0u8; dat.block_offset as usize]).unwrap();
            entries = decode_directory(&mut reader, dat.size);
        }else{
            panic!("Not actually a DirInode!");
        }
    }else if i.header.inode_type == inode::EXT_DIR_TYPE{
        if let Some(dat) = i.data.downcast_ref::<inode::types::ExtDirInode>(){
            rdr.seek(SeekFrom::Start(dat.block_start as u64 + superblock.dir_start)).unwrap();
            let mut reader = MetadataReader::new(rdr, superblock.compression);
            reader.read_exact(&mut vec![0u8; dat.block_offset as usize]).unwrap();
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

fn get_decompressor_for<'a, R: Read>(r: &'a mut R, decomp_type: u16) -> Box<dyn Read>{
    match decomp_type{
        0 => Box::new(r),
        1 => Box::new(ZlibDecoder::new(r)),
        2 => Box::new(LzmaReader::new_decompressor(r)),
        3 => panic!("LZO compression is not supported."),
        4 => Box::new(LzmaReader::new_decompressor(r)),
        5 => Box::new(lz4::Decoder::new(r).unwrap()),
        6 => Box::new(zstd::Decoder::new(r).unwrap()),
        _ => panic!("Invalid compression type: {}", decomp_type),
    }
}