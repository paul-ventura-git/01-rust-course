async fn make_future() -> &'static str {
    "a future is a value representing work that may finish later"
}

pub fn run() {
    println!("\n--- Futures and executor responsibilities ---");

    let future = make_future();
    println!("Creating a future does not run it; polling drives it forward.");
    println!("The standard library provides Future but not a general async runtime.");
    drop(future);
}
