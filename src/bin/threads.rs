use std::{thread, sync::mpsc::channel};

fn main() {
    let (rx, tx) = channel::<String>();

    let handle = thread::spawn(move || {
        println!("Hi from thread");
        rx.send("Hello from channel".to_string()).unwrap();
    });

    println!("Hi before join");
    handle.join().unwrap();
    println!("Hi after join");

    let message = tx.recv().unwrap();
    println!("Message: {}", message);
}
