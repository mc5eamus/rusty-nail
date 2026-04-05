//! # Compressor
//!
//! A library for compressing and decompressing data in memory and from/to files
//! using the Gzip format.

pub mod file;
pub mod memory;

pub use file::{compress_file, decompress_file};
pub use memory::{compress, decompress};

/// Errors that can occur during compression or decompression.
#[derive(Debug)]
pub enum CompressorError {
    Io(std::io::Error),
}

impl std::fmt::Display for CompressorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompressorError::Io(e) => write!(f, "I/O error: {e}"),
        }
    }
}

impl std::error::Error for CompressorError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CompressorError::Io(e) => Some(e),
        }
    }
}

impl From<std::io::Error> for CompressorError {
    fn from(e: std::io::Error) -> Self {
        CompressorError::Io(e)
    }
}
