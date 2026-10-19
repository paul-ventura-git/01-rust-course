#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

impl Message {
    fn describe(&self) -> String {
        match self {
            Self::Quit => String::from("quit"),
            Self::Move { x, y } => format!("move to ({x}, {y})"),
            Self::Write(text) => format!("write '{text}'"),
            Self::ChangeColor(r, g, b) => format!("change color to rgb({r}, {g}, {b})"),
        }
    }
}

pub fn run() {
    println!("\n--- Enums and data-carrying variants ---");

    let messages = [
        Message::Quit,
        Message::Move { x: 4, y: -2 },
        Message::Write(String::from("Hello, enum")),
        Message::ChangeColor(20, 80, 160),
    ];

    for message in &messages {
        println!("{message:?}: {}", message.describe());
    }
}
