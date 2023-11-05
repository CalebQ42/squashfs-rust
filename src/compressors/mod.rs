use std::io::Read;

use flate2::read::ZlibDecoder;
use lzma::LzmaReader;

pub(crate) trait Decompress{
    fn new_reader(&self, r: Box<dyn Read>) -> Box<dyn Read>;
    fn decompress_block(&self, block: &[u8]) -> Vec<u8>;
}

include!("passthrough.rs");
include!("zlib.rs");
include!("xz_lzma.rs");
include!("lz4.rs");
include!("zstd.rs");