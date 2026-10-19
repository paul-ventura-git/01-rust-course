pub fn run() {
    println!("\n--- if let, while let, and match guards ---");

    let configuration = Some(3);
    if let Some(level) = configuration {
        println!("Configured level: {level}");
    } else {
        println!("No level configured.");
    }

    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("Popped {top}");
    }

    let pair = (2, 5);
    match pair {
        (x, y) if x < y => println!("{x} is less than {y}."),
        (x, y) => println!("{x} is not less than {y}."),
    }
}
