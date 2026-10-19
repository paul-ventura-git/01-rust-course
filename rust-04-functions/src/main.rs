mod basic_functions;
mod parameters;
mod return_values;
mod scope;
mod higher_order;

fn main() {
    println!("====================================");
    println!(" LESSON 04: RUST FUNCTIONS");
    println!("====================================");

    basic_functions::run();
    parameters::run();
    return_values::run();
    scope::run();
    higher_order::run();

    println!("\nLesson completed successfully!");
}
