use std::fs;
use std::collections::HashMap;

use clap::Parser;


#[derive(Parser)]
struct Args {
    /// The path to the file to read
    #[arg(short, long)]
    path: String,
}

fn remove_punctuation(s: &str) -> String {
    s.replace(&['(', ')', ',', '\"', '.', ';', ':', '\'', '*'][..], "")
}

fn main() {
    // Read CLI input: which file are we counting words in?
    let args = Args::parse();

    // Read file content into a string
    let data = fs::read_to_string(&args.path).expect("Unable to read file");

    // Create a Hashmap for words + counts 
    // TODO: How can I make the keys `&str`?
    let mut word_counts: HashMap<String, u32> = HashMap::new();

    // Iterate string and populate hashmap
    for line in data.trim().lines() {
        for word in line.split_whitespace() {
            let word_str = remove_punctuation(&word);
            if !word_str.is_empty() {
                println!("{}", word_str);
            }
            let entry = word_counts.entry(word_str).or_insert(0);
            *entry += 1;

        }
    }

    // Print out results
    // TODO: Print in order. Could use a Tree Map to keep words sorted.
    // TODO: Or if we only print the k most common ones we don't need to sort but can
    // just iterate the values and keep the most common ones.
    for (k, v) in word_counts.iter() {
        println!("{k}: {v}");
    }

}
