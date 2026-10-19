use std::sync::mpsc;
use std::thread;

pub fn run() {
    println!("\n--- Message passing with channels ---");

    let (sender, receiver) = mpsc::channel();
    let worker = thread::spawn(move || {
        for message in ["one", "two", "three"] {
            sender
                .send(String::from(message))
                .expect("receiver should remain connected");
        }
    });

    for message in receiver {
        println!("Received: {message}");
    }
    worker.join().expect("sender thread should complete");
}
