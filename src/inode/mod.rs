mod types;

use std::{any::Any, io::Read};

use self::types::*;
use bincode;

pub(crate) const DIR_TYPE: u16 = 1;
pub(crate) const FILE_TYPE: u16 = 2;
pub(crate) const SYM_TYPE: u16 = 3;
pub(crate) const BLOCK_DEV_TYPE: u16 = 4;
pub(crate) const CHAR_DEV_TYPE: u16 = 5;
pub(crate) const FIFO_TYPE: u16 = 6;
pub(crate) const SOCKET_TYPE: u16 = 7;
pub(crate) const EXT_DIR_TYPE: u16 = 8;
pub(crate) const EXT_FILE_TYPE: u16 = 9;
pub(crate) const EXT_SYM_TYPE: u16 = 10;
pub(crate) const EXT_BLOCK_DEV_TYPE: u16 = 11;
pub(crate) const EXT_CHAR_DEV_TYPE: u16 = 12;
pub(crate) const EXT_FIFO_TYPE: u16 = 13;
pub(crate) const EXT_SOCKET_TYPE: u16 = 14;

pub(crate) struct Inode{
    header: InodeHeader,
    data: Box<dyn Any>,
}

impl Inode{
    pub(crate) fn decode(r: &mut impl Read, block_size: u32) -> Self{
        let mut r = Box::new(r);
        let header: InodeHeader = bincode::deserialize_from(r.as_mut()).unwrap();
        let data: Box<dyn Any> = match header.inode_type{
            DIR_TYPE =>{
                let tmp: DirInode = bincode::deserialize_from(r).unwrap();
                Box::new(tmp)
            },
            FILE_TYPE => {
                let tmp: FileInode = FileInode::decode(&mut r, block_size);
                Box::new(tmp)
            },
            SYM_TYPE => {
                let tmp: SymInode = SymInode::decode(&mut r);
                Box::new(tmp)
            },
            BLOCK_DEV_TYPE => {
                let tmp: DevInode = bincode::deserialize_from(r).unwrap();
                Box::new(tmp)
            },
            CHAR_DEV_TYPE => {
                let tmp: DevInode = bincode::deserialize_from(r).unwrap();
                Box::new(tmp)
            },
            FIFO_TYPE => {
                let tmp: IPCInode = bincode::deserialize_from(r).unwrap();
                Box::new(tmp)
            },
            SOCKET_TYPE => {
                let tmp: IPCInode = bincode::deserialize_from(r).unwrap();
                Box::new(tmp)
            },
            EXT_DIR_TYPE => {
                let tmp: ExtDirInode = ExtDirInode::decode(&mut r);
                Box::new(tmp)
            },
            EXT_FILE_TYPE => {
                let tmp: ExtFileInode = ExtFileInode::decode(&mut r, block_size);
                Box::new(tmp)
            },
            EXT_SYM_TYPE => {
                let tmp: ExtSymInode = ExtSymInode::decode(&mut r);
                Box::new(tmp)
            },
            EXT_BLOCK_DEV_TYPE => {
                let tmp: ExtDevInode = bincode::deserialize_from(r).unwrap();
                Box::new(tmp)
            },
            EXT_CHAR_DEV_TYPE => {
                let tmp: ExtDevInode = bincode::deserialize_from(r).unwrap();
                Box::new(tmp)
            },
            EXT_FIFO_TYPE => {
                let tmp: ExtIPCInode = bincode::deserialize_from(r).unwrap();
                Box::new(tmp)
            },
            EXT_SOCKET_TYPE => {
                let tmp: ExtIPCInode = bincode::deserialize_from(r).unwrap();
                Box::new(tmp)
            },
            _ => panic!("Unsupported inode type: {}", header.inode_type),
        };
        Self{
            header,
            data,
        }
    }
}