mod associated_types;
mod generic_functions;
mod trait_bounds;
mod traits;

fn main() {
    println!("==========================================");
    println!(" LESSON 11: GENERICS AND TRAITS");
    println!("==========================================");

    generic_functions::run();
    traits::run();
    trait_bounds::run();
    associated_types::run();

    println!("\nLesson completed successfully!");
}
