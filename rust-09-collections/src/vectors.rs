pub fn run() {
    println!("\n--- Vectors ---");

    let mut scores = vec![10, 20, 30];
    scores.push(40);
    println!("Scores: {scores:?}; second score: {}", scores[1]);

    let total: i32 = scores.iter().sum();
    println!("Total: {total}");

    if let Some(first) = scores.first() {
        println!("First score: {first}");
    }

    for score in &mut scores {
        *score += 1;
    }
    println!("After adding one to each score: {scores:?}");
}
