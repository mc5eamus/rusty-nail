//! In-memory compression and decompression using Gzip.

use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use std::io::{Read, Write};

use crate::CompressorError;

/// Compresses a byte slice using Gzip and returns the compressed bytes.
///
/// # Errors
///
/// Returns a [`CompressorError`] if an I/O error occurs during compression.
pub fn compress(data: &[u8]) -> Result<Vec<u8>, CompressorError> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data)?;
    Ok(encoder.finish()?)
}

/// Decompresses a Gzip-compressed byte slice and returns the original bytes.
///
/// # Errors
///
/// Returns a [`CompressorError`] if an I/O error occurs during decompression
/// or if the input is not valid Gzip data.
pub fn decompress(data: &[u8]) -> Result<Vec<u8>, CompressorError> {
    let mut decoder = GzDecoder::new(data);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)?;
    Ok(decompressed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_empty() {
        let data = b"";
        let compressed = compress(data).expect("compression failed");
        let decompressed = decompress(&compressed).expect("decompression failed");
        assert_eq!(decompressed, data);
    }

    #[test]
    fn round_trip_text() {
        let data = b"Hello, rusty-nail!";
        let compressed = compress(data).expect("compression failed");
        let decompressed = decompress(&compressed).expect("decompression failed");
        assert_eq!(decompressed, data);
    }

    #[test]
    fn compressed_is_valid_gzip() {
        let data = b"some content";
        let compressed = compress(data).expect("compression failed");
        // Gzip magic bytes: 0x1f 0x8b
        assert_eq!(&compressed[..2], &[0x1f, 0x8b]);
    }

    #[test]
    fn compress_large_data() {
        let data: Vec<u8> = (0..=255u8).cycle().take(100_000).collect();
        let compressed = compress(&data).expect("compression failed");
        let decompressed = decompress(&compressed).expect("decompression failed");
        assert_eq!(decompressed, data);
    }
}
