mod paths_and_metadata;
mod reading;
mod writing;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("===============================");
    println!(" LESSON 16: FILE I/O");
    println!("===============================");

    paths_and_metadata::run()?;
    reading::run()?;
    writing::run()?;

    println!("\nLesson completed successfully!");
    Ok(())
}
