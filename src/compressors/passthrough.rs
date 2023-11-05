pub(crate) struct Passthrough();

impl Decompress for Passthrough{
    fn new_reader(&self, r: Box<dyn Read>) -> Box<dyn Read>{
        r
    }
    fn decompress_block(&self, block: &[u8]) -> Vec<u8>{
        block.to_vec()
    }
}