pub(crate) struct ZstdDecomp();

impl Decompress for ZstdDecomp{
    fn new_reader(&self, r: Box<dyn Read>) -> Box<dyn Read>{
        Box::new(zstd::Decoder::new(r).unwrap())
    }
    fn decompress_block(&self, block: &[u8]) -> Vec<u8>{
        zstd::decode_all(block).unwrap()
    }
}