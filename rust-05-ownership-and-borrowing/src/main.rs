mod ownership;
mod move_semantics;
mod cloning;
mod borrowing;
mod mutable_borrowing;
mod ownership_functions;

fn main() {
    println!("=== Lesson 05: Ownership and Borrowing ===\n");

    ownership::run();
    move_semantics::run();
    cloning::run();
    borrowing::run();
    mutable_borrowing::run();
    ownership_functions::run();
}
