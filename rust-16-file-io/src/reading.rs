use std::error::Error;

use crate::paths_and_metadata::sample_path;

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("\n--- Reading text files ---");

    let contents = std::fs::read_to_string(sample_path())?;
    println!("{contents}");
    Ok(())
}
