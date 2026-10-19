#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Self) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

pub fn run() {
    println!("\n--- Methods and associated functions ---");

    let desk = Rectangle {
        width: 120,
        height: 60,
    };
    let notebook = Rectangle {
        width: 20,
        height: 30,
    };
    println!("Desk: {desk:?}; area: {}", desk.area());
    println!("Desk can hold notebook: {}", desk.can_hold(&notebook));
    println!("Square: {:?}", Rectangle::square(10));
}
