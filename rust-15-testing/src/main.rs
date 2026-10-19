use rust_15_testing::calculator::{add, average};

fn main() {
    println!("================================");
    println!(" LESSON 15: TESTING");
    println!("================================");

    println!("2 + 3 = {}", add(2, 3));
    println!("Average of [2, 4, 6] = {:?}", average(&[2.0, 4.0, 6.0]));
    println!("Run `cargo test` to execute unit, integration, and doc tests.");
}
