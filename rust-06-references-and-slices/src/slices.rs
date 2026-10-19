fn sum(values: &[i32]) -> i32 {
    values.iter().sum()
}

pub fn run() {
    println!("\n--- Array slices ---");

    let numbers = [3, 5, 8, 13, 21];
    let all_numbers: &[i32] = &numbers;
    let middle = &numbers[1..4];

    println!("All values: {all_numbers:?}");
    println!("Middle slice [1..4]: {middle:?}");
    println!("Sum of middle slice: {}", sum(middle));
    println!(
        "The same function accepts the full array: {}",
        sum(&numbers)
    );
}
