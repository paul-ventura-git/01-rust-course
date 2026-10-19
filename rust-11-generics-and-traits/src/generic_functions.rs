use std::fmt::Debug;

fn largest<T: PartialOrd>(values: &[T]) -> Option<&T> {
    let mut largest = values.first()?;
    for value in &values[1..] {
        if value > largest {
            largest = value;
        }
    }
    Some(largest)
}

#[derive(Debug)]
struct Pair<T> {
    first: T,
    second: T,
}

impl<T: Debug> Pair<T> {
    fn show(&self) {
        println!("Pair: {:?}, {:?}", self.first, self.second);
    }
}

pub fn run() {
    println!("\n--- Generic functions, structs, and impl blocks ---");

    let numbers = [12, 4, 28, 9];
    let words = ["ownership", "borrowing", "rust"];
    println!("Largest number: {:?}", largest(&numbers));
    println!("Largest word: {:?}", largest(&words));

    Pair {
        first: String::from("generic"),
        second: String::from("pair"),
    }
    .show();
}
