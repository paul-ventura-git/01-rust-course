struct Counter {
    next_value: u32,
    end: u32,
}

impl Counter {
    fn new(end: u32) -> Self {
        Self { next_value: 1, end }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_value > self.end {
            return None;
        }
        let current = self.next_value;
        self.next_value += 1;
        Some(current)
    }
}

pub fn run() {
    println!("\n--- Implementing Iterator ---");

    let counter = Counter::new(5);
    println!("Counter values: {:?}", counter.collect::<Vec<_>>());
}
