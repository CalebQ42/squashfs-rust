pub(crate) struct Lz4Decomp();

impl Decompress for Lz4Decomp{
    fn new_reader(&self, r: Box<dyn Read>) -> Box<dyn Read>{
        Box::new(lz4::Decoder::new(r).unwrap())
    }
    fn decompress_block(&self, block: &[u8]) -> Vec<u8>{
        let mut buf: Vec<u8> = Vec::new();
        lz4::Decoder::new(block).unwrap().read_to_end(&mut buf).unwrap();
        buf
    }
    
}