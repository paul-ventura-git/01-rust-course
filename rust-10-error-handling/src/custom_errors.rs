use std::error::Error;
use std::fmt;

#[derive(Debug)]
enum GuessError {
    NotANumber,
    OutOfRange { value: i32 },
}

impl fmt::Display for GuessError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotANumber => write!(formatter, "the guess is not a number"),
            Self::OutOfRange { value } => {
                write!(formatter, "the guess {value} is outside 1..=100")
            }
        }
    }
}

impl Error for GuessError {}

fn validate_guess(input: &str) -> Result<u8, Box<dyn Error>> {
    let value = input.parse::<i32>().map_err(|_| GuessError::NotANumber)?;
    if !(1..=100).contains(&value) {
        return Err(Box::new(GuessError::OutOfRange { value }));
    }
    Ok(value as u8)
}

pub fn run() {
    println!("\n--- Custom error types ---");

    for input in ["42", "0", "hello"] {
        match validate_guess(input) {
            Ok(value) => println!("Accepted guess: {value}"),
            Err(error) => println!("Rejected '{input}': {error}"),
        }
    }
}
