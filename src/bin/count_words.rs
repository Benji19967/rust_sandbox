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
    println!("{}", data);

    // Create a Hashmap for words + counts 

    // Iterate string and populate hashmap

    // Print out results
}
