mod async_functions;
mod executor;
mod futures;

fn main() {
    println!("==============================================");
    println!(" LESSON 19: ASYNCHRONOUS PROGRAMMING");
    println!("==============================================");

    let result = executor::block_on(async_functions::run());
    futures::run();
    println!("Async workflow result: {result}");
    println!("\nLesson completed successfully!");
}
