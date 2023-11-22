use std::io::Read;

use flate2::read::ZlibDecoder;
use lzma::LzmaReader;

pub(crate) trait Decompress{
    fn decode_block(&self, data: &[u8]) -> Vec<u8>;
}

pub(crate) struct Passthrough();

impl Decompress for Passthrough{
    fn decode_block(&self, data: &[u8]) -> Vec<u8>{
        data.to_vec()
    }
}

pub(crate) struct Zlib();

impl Decompress for Zlib{
    fn decode_block(&self, data: &[u8]) -> Vec<u8>{
        let mut buf = Vec::new();
        ZlibDecoder::new(data).read_to_end(&mut buf).unwrap();
        buf
    }
}

pub(crate) struct XzLzma();

impl Decompress for XzLzma{
    fn decode_block(&self, data: &[u8]) -> Vec<u8>{
        let mut buf = Vec::new();
        LzmaReader::new_decompressor(data).unwrap().read_to_end(&mut buf).unwrap();
        buf
    }
}

pub(crate) struct Lzo{
    pub(crate) max_size: u32
}

impl Decompress for Lzo{
    fn decode_block(&self, data: &[u8]) -> Vec<u8>{
        let lzo = minilzo_rs::LZO::init().unwrap();
        lzo.decompress_safe(data, self.max_size as usize).unwrap()
    }
}

pub(crate) struct Lz4();

impl Decompress for Lz4{
    fn decode_block(&self, data: &[u8]) -> Vec<u8>{
        let mut buf = vec![0u8; data.len()];
        lz4::Decoder::new(data).unwrap().read_exact(&mut buf).unwrap();
        buf
    }
}

pub(crate) struct Zstd();

impl Decompress for Zstd{
    fn decode_block(&self, data: &[u8]) -> Vec<u8> {
        zstd::decode_all(data).unwrap()
    }
}