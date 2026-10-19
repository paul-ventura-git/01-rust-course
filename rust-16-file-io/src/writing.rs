use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("\n--- Writing a temporary file ---");

    let path: PathBuf =
        std::env::temp_dir().join(format!("rust-course-lesson-16-{}.txt", std::process::id()));
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)?;
    writeln!(file, "Written with the standard library.")?;
    drop(file);

    let contents = fs::read_to_string(&path)?;
    let cleanup_result = fs::remove_file(&path);
    println!("{}", contents.trim_end());
    cleanup_result?;
    Ok(())
}
