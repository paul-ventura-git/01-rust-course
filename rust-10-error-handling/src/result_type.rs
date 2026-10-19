use std::num::ParseIntError;

fn parse_port(input: &str) -> Result<u16, ParseIntError> {
    input.parse()
}

pub fn run() {
    println!("\n--- Result<T, E> and the ? operator ---");

    match parse_port("8080") {
        Ok(port) => println!("Listening on port {port}."),
        Err(error) => println!("Could not parse port: {error}"),
    }

    match parse_port("not-a-port") {
        Ok(port) => println!("Listening on port {port}."),
        Err(error) => println!("Invalid input is reported explicitly: {error}"),
    }

    fn parse_pair(left: &str, right: &str) -> Result<(i32, i32), ParseIntError> {
        Ok((left.parse()?, right.parse()?))
    }
    println!("Parsed pair: {:?}", parse_pair("12", "34"));
}
