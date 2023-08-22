use std::sync::Arc;
use miniz_oxide::deflate::{compress_to_vec, compress_to_vec_zlib};
use miniz_oxide::inflate::{decompress_to_vec, decompress_to_vec_zlib};
use rusty_leveldb_arc::{Compressor, CompressorList, Options, DB};


struct ZlibCompressor(u8);

impl ZlibCompressor {
    /// level 0-10
    pub fn new(level: u8) -> Self {
        assert!(level <= 10);
        Self(level)
    }
}

impl Compressor for ZlibCompressor {
    fn encode(&self, block: Vec<u8>) -> rusty_leveldb_arc::Result<Vec<u8>> {
        Ok(compress_to_vec_zlib(&block, self.0))
    }

    fn decode(&self, block: Vec<u8>) -> rusty_leveldb_arc::Result<Vec<u8>> {
        decompress_to_vec_zlib(&block).map_err(|e| rusty_leveldb_arc::Status {
            code: rusty_leveldb_arc::StatusCode::CompressionError,
            err: e.to_string(),
        })
    }
}

struct RawZlibCompressor(u8);

impl RawZlibCompressor {
    /// level 0-10
    pub fn new(level: u8) -> Self {
        assert!(level <= 10);
        Self(level)
    }
}

impl Compressor for RawZlibCompressor {
    fn encode(&self, block: Vec<u8>) -> rusty_leveldb_arc::Result<Vec<u8>> {
        Ok(compress_to_vec(&block, self.0))
    }

    fn decode(&self, block: Vec<u8>) -> rusty_leveldb_arc::Result<Vec<u8>> {
        decompress_to_vec(&block).map_err(|e| rusty_leveldb_arc::Status {
            code: rusty_leveldb_arc::StatusCode::CompressionError,
            err: e.to_string(),
        })
    }
}

pub fn mcpe_options(compression_level: u8) -> Options {
    let mut opt = Options::default();
    opt.compressor = 0;
    let mut list = CompressorList::new();
    list.set_with_id(0, RawZlibCompressor::new(compression_level));
    list.set_with_id(1, ZlibCompressor::new(compression_level));
    opt.compressor_list = Arc::new(list);
    opt
}

fn main() {
    let path = "mcpe_db";
    let compression_level = 10;
    let opt = mcpe_options(compression_level);
    let mut db = DB::open(path, opt).unwrap();
    db.put(b"~local_player", b"NBT data goes here").unwrap();
    let value = db.get(b"~local_player").unwrap();
    assert_eq!(&value, b"NBT data goes here")
}
