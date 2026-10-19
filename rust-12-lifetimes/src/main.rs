mod lifetime_elision;
mod lifetime_functions;
mod lifetime_structs;

fn main() {
    println!("===============================");
    println!(" LESSON 12: LIFETIMES");
    println!("===============================");

    lifetime_functions::run();
    lifetime_structs::run();
    lifetime_elision::run();

    println!("\nLesson completed successfully!");
}
