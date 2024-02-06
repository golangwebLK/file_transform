use std::fs::File;
use std::io::{Read};
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use flate2::Compression;

pub fn compress_to_buffer(input_filename: &str) -> std::io::Result<Vec<u8>> {
    let mut input_file = File::open(input_filename)?;
    let mut compressor = GzEncoder::new(Vec::new(), Compression::default());

    std::io::copy(&mut input_file, &mut compressor)?;

    let compressed_data = compressor.finish()?;
    Ok(compressed_data)
}

pub fn decompress_from_buffer(compressed_data: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut decompressor = GzDecoder::new(compressed_data);

    let mut decompressed_data = Vec::new();
    decompressor.read_to_end(&mut decompressed_data)?;

    Ok(decompressed_data)
}
