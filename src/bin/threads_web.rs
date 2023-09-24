use std::{thread, error::Error};

fn get_request(url: String) -> Result<(), Box<dyn Error>> {
    let resp = reqwest::blocking::get(url)?.text()?;
    println!("{:#?}", resp);
    Ok(())
}

fn main() {
    let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();
    for i in 1..34 {
        let url = format!("https://www.gutenberg.org/cache/epub/{idx}/pg{idx}.txt", idx = i);
        let handle = thread::spawn(|| {
            let _ = get_request(url);
        });
        handles.push(handle);
    }
    for handle in handles {
        let _ = handle.join();
    }
}
