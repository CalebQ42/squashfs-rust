use std::io::Read;

use flate2::read::ZlibDecoder;
use lzma::LzmaReader;

pub(crate) trait Decompress{
    fn new_reader(&self, r: Box<dyn Read>) -> Box<dyn Read>;
    fn decompress_block(&self, block: &[u8]) -> Vec<u8>;
}

pub(crate) struct Passthrough();

impl Decompress for Passthrough{
    fn new_reader(&self, r: Box<dyn Read>) -> Box<dyn Read>{
        r
    }
    fn decompress_block(&self, block: &[u8]) -> Vec<u8>{
        block.to_vec()
    }
}

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

pub(crate) struct ZstdDecomp();

impl Decompress for ZstdDecomp{
    fn new_reader(&self, r: Box<dyn Read>) -> Box<dyn Read>{
        Box::new(zstd::Decoder::new(r).unwrap())
    }
    fn decompress_block(&self, block: &[u8]) -> Vec<u8>{
        zstd::decode_all(block).unwrap()
    }
}