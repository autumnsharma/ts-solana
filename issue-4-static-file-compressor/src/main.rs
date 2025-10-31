use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use flate2::write::GzEncoder;
use flate2::Compression;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run <file-to-compress>");
        return Ok(());
    }

    let input_path = &args[1];
    let mut input_file = File::open(input_path)?;
    let mut contents = Vec::new();
    input_file.read_to_end(&mut contents)?;

    let output_path = format!("{}.gz", input_path);
    let output_file = File::create(&output_path)?;
    let mut encoder = GzEncoder::new(output_file, Compression::default());
    encoder.write_all(&contents)?;
    encoder.finish()?;

    println!("Compressed '{}' -> '{}'", input_path, output_path);
    Ok(())
}
