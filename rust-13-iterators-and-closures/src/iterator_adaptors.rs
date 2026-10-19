pub fn run() {
    println!("\n--- Lazy adaptors and iterator consumers ---");

    let numbers = 1..=10;
    let even_squares: Vec<i32> = numbers
        .filter(|number| number % 2 == 0)
        .map(|number| number * number)
        .collect();
    println!("Squares of even numbers: {even_squares:?}");

    let sum: i32 = even_squares.iter().sum();
    println!("Their sum: {sum}");

    let first_large = (1..)
        .map(|number| number * number)
        .find(|square| *square > 100);
    println!("First square greater than 100: {first_large:?}");
}
