//! CLI tool for compressing and decompressing files or in-memory data using Gzip.
//!
//! # Usage
//!
//! ```text
//! cli compress <input> <output>
//! cli decompress <input> <output>
//! cli demo
//! ```
//!
//! Run `cli --help` to see available commands.

use std::process;

use compressor::{compress, compress_file, decompress, decompress_file};

fn print_usage() {
    eprintln!("Usage:");
    eprintln!("  cli compress <input_file> <output_file.gz>");
    eprintln!("  cli decompress <input_file.gz> <output_file>");
    eprintln!("  cli demo");
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(String::as_str) {
        Some("compress") => {
            let input = args.get(2).ok_or("missing input path")?;
            let output = args.get(3).ok_or("missing output path")?;
            compress_file(input, output)?;
            println!("Compressed '{input}' → '{output}'");
        }
        Some("decompress") => {
            let input = args.get(2).ok_or("missing input path")?;
            let output = args.get(3).ok_or("missing output path")?;
            decompress_file(input, output)?;
            println!("Decompressed '{input}' → '{output}'");
        }
        Some("demo") => {
            let message = "Hello from rusty-nail! This data is being compressed in memory.";
            println!("Original  ({:>4} bytes): {message}", message.len());

            let compressed = compress(message.as_bytes())?;
            println!("Compressed ({:>3} bytes): {:?}", compressed.len(), &compressed[..10]);

            let decompressed = decompress(&compressed)?;
            println!(
                "Decompressed ({:>2} bytes): {}",
                decompressed.len(),
                String::from_utf8_lossy(&decompressed)
            );
        }
        _ => {
            print_usage();
            process::exit(1);
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}
