use std::error::Error;
use std::path::PathBuf;

pub fn sample_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("data")
        .join("sample.txt")
}

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("\n--- Paths and file metadata ---");

    let path = sample_path();
    let metadata = std::fs::metadata(&path)?;
    println!("Sample path: {}", path.display());
    println!("File size: {} bytes", metadata.len());
    println!("Is a file: {}", metadata.is_file());
    Ok(())
}
