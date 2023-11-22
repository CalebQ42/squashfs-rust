use std::io::{Read, Result};

use crate::decompress::Decompress;

pub(crate) struct MetadataReader<'a, R: Read, D: Decompress>{
    rdr: &'a mut R,
    decomp: &'a D,
    cur_pos: usize,
    buf: Vec<u8>,
}

impl<'a, R: Read, D: Decompress> MetadataReader<'a, R, D> {
    pub(crate) fn new(mut rdr: &'a mut R, decomp: &'a D) -> Self {
        let size: u16 = bincode::deserialize_from(&mut rdr).unwrap();
        let real_size = size & 0x7fff;
        let mut buf = vec![0u8; real_size as usize];
        rdr.read_exact(&mut buf).unwrap();
        if real_size == size{
            buf = decomp.decode_block(&mut buf);
        }
        Self {
            rdr,
            decomp,
            cur_pos: 0,
            buf: buf,
        }
    }

    fn advance(&mut self) {
        let size: u16 = bincode::deserialize_from(&mut self.rdr).unwrap();
        let real_size = size & 0x7fff;
        self.buf = vec![0u8; real_size as usize];
        self.rdr.read_exact(&mut self.buf).unwrap();
        if real_size == size{
            self.buf = self.decomp.decode_block(&mut self.buf);
        }
    }
}

impl<'a, R: Read, D: Decompress> Read for MetadataReader<'a, R, D>{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let mut has_read = 0usize;
        while has_read < buf.len(){
            let mut to_read = buf.len() - has_read;
            if to_read > (self.buf.len() - self.cur_pos){
                to_read = self.buf.len() - self.cur_pos;
            }
            buf[has_read..has_read+to_read].copy_from_slice(&self.buf[self.cur_pos..self.cur_pos+to_read]);
            self.cur_pos += to_read;
            if self.cur_pos == self.buf.len(){
                self.advance();
            }
            has_read += to_read;
        }
        Ok(has_read)
    }
}