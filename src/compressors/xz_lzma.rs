pub(crate) struct XzLzmaDecomp();

impl Decompress for XzLzmaDecomp{
    fn new_reader(&self, r: Box<dyn Read>) -> Box<dyn Read>{
        Box::new(LzmaReader::new_decompressor(r).unwrap())
    }
    fn decompress_block(&self, block: &[u8]) -> Vec<u8>{
        let mut buf: Vec<u8> = Vec::new();
        LzmaReader::new_decompressor(block).unwrap().read_to_end(&mut buf).unwrap();
        buf
    }
}