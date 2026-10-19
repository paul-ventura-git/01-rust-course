mod destructuring;
mod if_let_and_guards;
mod match_expressions;

fn main() {
    println!("====================================");
    println!(" LESSON 08: PATTERN MATCHING");
    println!("====================================");

    match_expressions::run();
    destructuring::run();
    if_let_and_guards::run();

    println!("\nLesson completed successfully!");
}
