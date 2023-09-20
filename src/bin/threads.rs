use std::{thread, time::Duration};

fn main() {
    let handle = thread::spawn(|| {
        println!("Hi");
        println!("Hi");
        thread::sleep(Duration::from_secs(1));
        println!("Hi");
        thread::sleep(Duration::from_secs(1));
    });

    println!("Main before");

    handle.join().unwrap();

    println!("Main after");
}
