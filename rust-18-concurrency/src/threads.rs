use std::thread;

pub fn run() {
    println!("\n--- Spawning and joining threads ---");

    let values = Vec::from([1, 2, 3, 4]);
    let worker = thread::spawn(move || values.iter().sum::<i32>());
    let total = worker.join().expect("worker thread should complete");
    println!("Worker thread calculated {total}.");

    let message = String::from("scoped thread");
    thread::scope(|scope| {
        scope.spawn(|| println!("Borrowed data in a {message}."));
    });
}
