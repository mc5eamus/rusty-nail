//! File-based compression and decompression using Gzip.

use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use std::fs::File;
use std::io::{BufReader, BufWriter, copy};
use std::path::Path;

use crate::CompressorError;

/// Compresses the file at `input_path` and writes the result to `output_path`.
///
/// # Errors
///
/// Returns a [`CompressorError`] if the input or output file cannot be opened,
/// or if an I/O error occurs during compression.
pub fn compress_file(
    input_path: impl AsRef<Path>,
    output_path: impl AsRef<Path>,
) -> Result<(), CompressorError> {
    let input = File::open(input_path)?;
    let output = File::create(output_path)?;

    let mut reader = BufReader::new(input);
    let mut encoder = GzEncoder::new(BufWriter::new(output), Compression::default());

    copy(&mut reader, &mut encoder)?;
    encoder.finish()?;

    Ok(())
}

/// Decompresses the Gzip file at `input_path` and writes the result to `output_path`.
///
/// # Errors
///
/// Returns a [`CompressorError`] if the input or output file cannot be opened,
/// or if an I/O error occurs during decompression or if the file is not valid Gzip.
pub fn decompress_file(
    input_path: impl AsRef<Path>,
    output_path: impl AsRef<Path>,
) -> Result<(), CompressorError> {
    let input = File::open(input_path)?;
    let output = File::create(output_path)?;

    let mut decoder = GzDecoder::new(BufReader::new(input));
    let mut writer = BufWriter::new(output);

    copy(&mut decoder, &mut writer)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn write_temp_file(content: &[u8]) -> NamedTempFile {
        let mut file = NamedTempFile::new().expect("failed to create temp file");
        file.write_all(content).expect("failed to write temp file");
        file
    }

    #[test]
    fn round_trip_file() {
        let original_content = b"Hello from a file!";
        let input_file = write_temp_file(original_content);

        let compressed_file = NamedTempFile::new().expect("failed to create temp file");
        let decompressed_file = NamedTempFile::new().expect("failed to create temp file");

        compress_file(input_file.path(), compressed_file.path())
            .expect("compression failed");
        decompress_file(compressed_file.path(), decompressed_file.path())
            .expect("decompression failed");

        let result = std::fs::read(decompressed_file.path()).expect("failed to read result");
        assert_eq!(result, original_content);
    }

    #[test]
    fn round_trip_empty_file() {
        let input_file = write_temp_file(b"");

        let compressed_file = NamedTempFile::new().expect("failed to create temp file");
        let decompressed_file = NamedTempFile::new().expect("failed to create temp file");

        compress_file(input_file.path(), compressed_file.path())
            .expect("compression failed");
        decompress_file(compressed_file.path(), decompressed_file.path())
            .expect("decompression failed");

        let result = std::fs::read(decompressed_file.path()).expect("failed to read result");
        assert_eq!(result, b"");
    }

    #[test]
    fn compress_file_missing_input() {
        let result = compress_file("/nonexistent/path/file.txt", "/tmp/out.gz");
        assert!(result.is_err());
    }
}
