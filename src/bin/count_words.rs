use std::fs;
use std::collections::HashMap;

use clap::Parser;

// See questions unders `/docs/COUNT_WORDS.md`
// TODO: Make the program multi-threaded and count the word frequency for many
// files. Aggregate the frequency of each word into one count at the end.
// Compare performance between the single-threaded and multi-threaded versions.

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
    // TODO: How can I read line by line instead of the entire file?
    let data = fs::read_to_string(&args.path).expect("Unable to read file");

    // Create a Hashmap for words + counts 
    let mut word_counts: HashMap<String, u32> = HashMap::new();

    // Iterate string and populate hashmap
    for line in data.trim().lines() {
        for word in line.split_whitespace() {
            let word_str = remove_punctuation(&word).to_lowercase();
            if !word_str.is_empty() {
                *word_counts.entry(word_str).or_insert(0) += 1;
            }
        }
    }

    // Print out results
    // TODO: Print in order. Could use a Tree Map to keep words sorted.
    // TODO: Or if we only print the k most common ones we don't need to sort but 
    // can just iterate the values and keep the most common ones.
    let mut word_counts_vec: Vec<(String, u32)> = word_counts.into_iter().collect(); 
    word_counts_vec.sort_by(|(_, cnt1), (_, cnt2)| cnt1.cmp(&cnt2).reverse());

    // Print in reverse order so we see the most common words at the bottom 
    // of the screen.
    for (word, count) in word_counts_vec.iter().rev() {
        println!("{word}: {count}");
    }

    // What if we are only interested in the top k most frequent elements?
}
