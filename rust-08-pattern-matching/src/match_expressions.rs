#[derive(Debug)]
enum Command {
    Start,
    Stop,
    SetSpeed(u8),
}

fn describe(command: &Command) -> &'static str {
    match command {
        Command::Start => "starting",
        Command::Stop => "stopping",
        Command::SetSpeed(speed) if *speed > 100 => "speed is out of range",
        Command::SetSpeed(_) => "setting speed",
    }
}

pub fn run() {
    println!("\n--- Exhaustive match expressions ---");

    let commands = [
        Command::Start,
        Command::SetSpeed(80),
        Command::SetSpeed(120),
        Command::Stop,
    ];
    for command in &commands {
        println!("{command:?}: {}", describe(command));
    }

    let number = 7;
    let description = match number {
        0 => "zero",
        1..=5 => "between one and five",
        6 | 7 => "six or seven",
        _ => "another number",
    };
    println!("{number} is {description}.");
}
