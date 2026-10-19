fn apply_to_three<F>(operation: F) -> i32
where
    F: Fn(i32) -> i32,
{
    operation(3)
}

pub fn run() {
    println!("\n--- Closures and capture ---");

    let multiplier = 4;
    let multiply = |value| value * multiplier;
    println!("3 multiplied by captured {multiplier}: {}", multiply(3));

    let offset = 10;
    println!(
        "Closure passed as an argument: {}",
        apply_to_three(|value| value + offset)
    );

    let mut calls = 0;
    let mut count_call = || {
        calls += 1;
        calls
    };
    println!("Call counts: {}, {}", count_call(), count_call());
}
