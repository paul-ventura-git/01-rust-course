pub fn run() {
    println!("\n--- Iterator creation and consumption ---");

    let values = [2, 4, 6];
    let mut iterator = values.iter();
    println!("First: {:?}", iterator.next());
    println!("Second: {:?}", iterator.next());
    println!("Remaining: {:?}", iterator.collect::<Vec<_>>());

    let owned: Vec<i32> = values.into_iter().map(|value| value * 10).collect();
    println!("Owned values: {owned:?}");
}
