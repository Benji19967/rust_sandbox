use std::fs;

use clap::Parser;


#[derive(Parser)]
struct Args {
    /// The path to the file to read
    #[arg(short, long)]
    path: String,
}

fn main() {
    // Read CLI input: which file are we counting words in?
    let args = Args::parse();

    // Read file content into a string
    let data = fs::read_to_string(&args.path).expect("Unable to read file");

    let mut counter: u32 = 0;

    // Create a Hashmap for words + counts 
    for line in data.trim().lines() {
        for word in line.split_whitespace() {
            println!("{}", word);
            counter += 1;
            if counter == 100 {
                return;
            }
        }
    }

    // Iterate string and populate hashmap

    // Print out results
}
