pub(crate) struct ZlibDecomp();

impl Decompress for ZlibDecomp{
    fn new_reader(&self, r: Box<dyn Read>) -> Box<dyn Read>{
        Box::new(flate2::read::ZlibDecoder::new(r))
    }
    fn decompress_block(&self, block: &[u8]) -> Vec<u8>{
        let mut buf: Vec<u8> = Vec::new();
        ZlibDecoder::new(block).read_to_end(&mut buf).unwrap();
        buf
    }
}