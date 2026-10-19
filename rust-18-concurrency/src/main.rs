mod channels;
mod shared_state;
mod threads;

fn main() {
    println!("================================");
    println!(" LESSON 18: CONCURRENCY");
    println!("================================");

    threads::run();
    channels::run();
    shared_state::run();

    println!("\nLesson completed successfully!");
}
