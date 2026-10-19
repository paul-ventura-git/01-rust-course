use std::sync::{Arc, Mutex};
use std::thread;

pub fn run() {
    println!("\n--- Shared state with Arc<Mutex<T>> ---");

    let total = Arc::new(Mutex::new(0));
    let mut workers = Vec::new();

    for _ in 0..4 {
        let total = Arc::clone(&total);
        workers.push(thread::spawn(move || {
            let mut value = total.lock().expect("counter mutex should not be poisoned");
            *value += 1;
        }));
    }

    for worker in workers {
        worker.join().expect("counter thread should complete");
    }
    println!(
        "Final counter: {}",
        *total.lock().expect("counter mutex should not be poisoned")
    );
}
