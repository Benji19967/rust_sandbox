use std::collections::HashMap;

use clap::Parser;

use rust_sandbox::my_io::read_lines;

// See questions unders `/docs/COUNT_WORDS.md`

// TODO: Make the program multi-threaded and count the word frequency for many
// files. Aggregate the frequency of each word into one count at the end.
// Compare performance between the single-threaded and multi-threaded versions.

#[derive(Parser)]
struct Args {
    /// The path to the file to read
    #[arg(short, long)]
    path: String,
    /// Find the most common k words
    #[arg(short = 'k', long = "num-most-frequent")]
    k: Option<usize>,
}

fn remove_punctuation(s: &str) -> String {
    s.replace(&['(', ')', ',', '\"', '.', ';', ':', '\'', '*'][..], "")
}

fn main() {
    let args = Args::parse();

    let mut word_counts: HashMap<String, u32> = HashMap::new();

    // Read file content into a string
    // TODO: This is deeply nested, can we extract a function?
    if let Ok(lines) = read_lines(&args.path) {
        for line in lines {
            if let Ok(line) = line {
                for word in line.trim().split_whitespace() {
                    let word_str = remove_punctuation(&word).to_lowercase();
                    if !word_str.is_empty() {
                        *word_counts.entry(word_str).or_insert(0) += 1;
                    }
                }
            }
        }
    }

    // Print out results
    // TODO: Print in order. Could use a Tree Map to keep words sorted.
    // TODO: Or if we only print the k most common ones we don't need to sort but
    // can just iterate the values and keep the most common ones.
    let mut word_counts_vec: Vec<(String, u32)> = word_counts.into_iter().collect();
    word_counts_vec.sort_by(|(_, cnt1), (_, cnt2)| cnt1.cmp(&cnt2).reverse());
    let word_counts_vec_most_frequent: Vec<(String, u32)> = match args.k {
        // TODO: This will panic if there are less than k unique words
        Some(k) => word_counts_vec[..k].to_vec(), 
        None => word_counts_vec
    };

    // Print in reverse order so we see the most common words at the bottom
    // of the screen.
    for (word, count) in word_counts_vec_most_frequent.iter().rev() {
        println!("{word}: {count}");
    }

    // What if we are only interested in the top k most frequent elements?
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_commas() {
        assert_eq!(remove_punctuation("Hello, I am Ben"), "Hello I am Ben");
    }

    #[test]
    fn keep_newlines() {
        assert_eq!(remove_punctuation("Hello\nI am Ben"), "Hello\nI am Ben");
    }
}
