use std::io::{Read, Seek, SeekFrom};

use crate::decompressors::Decompress;

pub(crate) struct MetadataReader<'a, R: Read+Seek> {
    r: Box<&'a mut R>,
    decomp: &'a Box<dyn Decompress>,
    buffer: Vec<u8>,
    cur_offset: usize,
}

impl<'a, R: Read+Seek> MetadataReader<'a, R> {
    pub fn new(r: &'a mut R, decomp: &'a Box<dyn Decompress>, init_offset: u64) -> Self {
        let mut r = Box::new(r);
        r.seek(SeekFrom::Start(init_offset)).unwrap();
        let size: u16 = bincode::deserialize_from(r.as_mut()).unwrap();
        let real_size = size & 0x7FFF;
        let mut buffer = vec![0u8; real_size as usize];
        r.read_exact(&mut buffer).unwrap();
        if size == real_size{
            buffer = decomp.decompress_block(&buffer);
        }
        Self {
            r,
            decomp,
            buffer,
            cur_offset: 0,
        }
    }

    pub fn read_next_block(&mut self) -> (){
        let size: u16 = bincode::deserialize_from(self.r.as_mut()).unwrap();
        let real_size = size & 0x7FFF;
        self.buffer = vec![0u8; real_size as usize];
        self.r.read_exact(&mut self.buffer).unwrap();
        if size == real_size{
            self.buffer = self.decomp.decompress_block(&self.buffer);
        }
        self.cur_offset = 0;
    }
}

impl<'a, R: Read+Seek> Read for MetadataReader<'a, R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut read: usize = 0;
        while read<buf.len(){
            let to_read = (buf.len() - read).min(self.buffer.len() - self.cur_offset);
            buf[read..read+to_read].copy_from_slice(&self.buffer[self.cur_offset..self.cur_offset+to_read]);
            self.cur_offset += to_read;
            if self.cur_offset == self.buffer.len(){
                self.read_next_block();
            }
            read += to_read;
        }
        Ok(read)
    }
}

impl<'a, R: Read+Seek> Seek for MetadataReader<'a, R>{
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        let mut cur_offset: u64 = 0;
        match pos{
            SeekFrom::Current(offset) =>{
                if offset < 0{
                    unimplemented!();
                }
                while cur_offset < offset as u64{
                    let to_skip = (offset as u64 - cur_offset).min((self.buffer.len() - self.cur_offset) as u64);
                    if self.cur_offset+(to_skip as usize) == self.buffer.len(){
                        self.read_next_block();
                    }else{
                        self.cur_offset += to_skip as usize;
                    }
                    cur_offset += to_skip;
                }
            }
            _ => unimplemented!(),
        }
        Ok(cur_offset)
    }
}