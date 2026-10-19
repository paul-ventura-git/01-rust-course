mod if_else;
mod match_control;
mod loops;
mod loop_control;

fn main() {
    println!("====================================");
    println!(" LESSON 02: RUST CONTROL STRUCTURES");
    println!("====================================");

    if_else::run();
    match_control::run();
    loops::run();
    loop_control::run();

    println!("\nLesson completed successfully!");
}
