//! Integration tests for the `compressor` crate using fixture files.

use std::path::PathBuf;

use compressor::{compress, compress_file, decompress, decompress_file};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("workspace root")
        .join("tests")
        .join("fixtures")
        .join(name)
}

#[test]
fn memory_round_trip_fixture() {
    let original = std::fs::read(fixture_path("sample.txt")).expect("fixture not found");
    let compressed = compress(&original).expect("compression failed");
    let decompressed = decompress(&compressed).expect("decompression failed");
    assert_eq!(decompressed, original);
}

#[test]
fn file_round_trip_fixture() {
    let input = fixture_path("sample.txt");

    let compressed = tempfile::NamedTempFile::new().expect("tmp file");
    let decompressed = tempfile::NamedTempFile::new().expect("tmp file");

    compress_file(&input, compressed.path()).expect("compression failed");
    decompress_file(compressed.path(), decompressed.path()).expect("decompression failed");

    let original = std::fs::read(&input).expect("fixture not found");
    let result = std::fs::read(decompressed.path()).expect("result not found");
    assert_eq!(result, original);
}

#[test]
fn compressed_output_is_smaller_than_input() {
    // The fixture file contains repetitive plain text, which compresses well.
    let original = std::fs::read(fixture_path("sample.txt")).expect("fixture not found");
    let compressed = compress(&original).expect("compression failed");
    assert!(
        compressed.len() < original.len(),
        "expected compressed ({}) < original ({})",
        compressed.len(),
        original.len()
    );
}
