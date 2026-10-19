mod box_pointer;
mod drop_trait;
mod interior_mutability;
mod reference_counting;

fn main() {
    println!("====================================");
    println!(" LESSON 17: SMART POINTERS");
    println!("====================================");

    box_pointer::run();
    reference_counting::run();
    interior_mutability::run();
    drop_trait::run();

    println!("\nLesson completed successfully!");
}
