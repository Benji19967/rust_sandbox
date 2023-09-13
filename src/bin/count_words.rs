use std::fs;
use std::collections::HashMap;

use clap::Parser;

// TODO: Answer all questions mentioned here:
// https://benhoyt.com/writings/count-words/

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
            let word_str = remove_punctuation(&word).to_lowercase();
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
    let mut word_counts_vec: Vec<(&String, &u32)> = word_counts.iter().collect(); 
    word_counts_vec.sort_by(|a, b| b.1.cmp(a.1));

    // Print in reverse order so we see the most common words at the bottom 
    // of the screen.
    for (word, count) in word_counts_vec.iter().rev() {
        println!("{word}: {count}");
    }

}
