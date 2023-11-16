use std::io::{Read, Seek};

use crate::get_decompressor_for;

pub(crate) struct MetadataReader<'a, R: Read> {
    r: &'a mut R,
    decomp_type: u16,
    cur_rdr: Box<dyn Read>
}

impl<'a, R: Read> MetadataReader<'a, R> {
    pub fn new(r: &'a mut R, decomp_type: u16) -> Self {
        let size: u16 = bincode::deserialize_from(r).unwrap();
        let real_size = size & 0x7FFF;
        Self {
            r,
            decomp_type,
            cur_rdr: if size == real_size{
                get_decompressor_for(&mut r.take(real_size as u64), decomp_type)
            }else{
                Box::new(r.take(real_size as u64))
            },
        }
    }

    pub fn setup_next_reader(&mut self) -> (){
        let size: u16 = bincode::deserialize_from(&mut self.r).unwrap();
        let real_size = size & 0x7FFF;
        self.cur_rdr = if size == real_size{
            get_decompressor_for(&mut self.r.take(real_size as u64), self.decomp_type)
        }else{
            Box::new(&mut self.r.take(real_size as u64))
        } 
    }
}

impl<'a, R: Read+Seek> Read for MetadataReader<'a, R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut cur_res = self.cur_rdr.read(buf);
        if cur_res.is_err(){
            return cur_res;
        }
        let mut cur_read = cur_res.unwrap();
        while cur_read < buf.len(){
            cur_res = self.cur_rdr.read(&mut buf[cur_read..]);
            if cur_res.is_err(){
                return cur_res;
            }
            cur_read += cur_res.unwrap();
        }
        Ok(cur_read)
    }
}