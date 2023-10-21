use std::collections::HashMap;
use std::path::{Path, PathBuf};

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
    path: PathBuf,
    /// Find the most common k words
    #[arg(short = 'k', long = "num-most-frequent")]
    k: Option<usize>,
}

fn remove_punctuation(s: &str) -> String {
    s.replace(&['(', ')', ',', '\"', '.', ';', ':', '\'', '*'][..], "")
}

fn compute_word_counts(path: &Path) -> HashMap<String, u32> {
    let mut word_counts: HashMap<String, u32> = HashMap::new();

    // Read file content into a string
    // TODO: This is deeply nested, can we extract a function?
    if let Ok(lines) = read_lines(path) {
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

    word_counts
}

/// Taking `<T: Into<String>` allows us to pass a hashmap with keys of string literals (or
/// anything that can be converted _into_ a `String`). 
/// 
/// Example: 
/// ```
/// get_most_frequent_words(HashMap::from(["hello", 8]), None)
/// ```
/// rather than
/// ```
/// get_most_frequent_words(HashMap::from([String::from("hello"), 8]), None)
/// ```
/// In the function we then use `s.into()` to convert from a literal to a `String`.
///
pub fn get_most_frequent_words<T: Into<String>>(word_counts: HashMap<T, u32>, k: Option<usize>) -> Vec<(String, u32)> {
    // TODO: Print in order. Could use a Tree Map to keep words sorted.
    // TODO: Or if we only print the k most common ones we don't need to sort but
    // can just iterate the values and keep the most common ones.
    let mut word_counts_vec: Vec<(String, u32)> = word_counts.into_iter().map(|(k, v)| (k.into(), v)).collect();
    word_counts_vec.sort_by(|(_, cnt1), (_, cnt2)| cnt1.cmp(&cnt2).reverse());
    match k {
        // TODO: This will panic if there are less than k unique words
        Some(k) => word_counts_vec[..k].to_vec(), 
        None => word_counts_vec
    }
}

fn main() {
    let args = Args::parse();

    let word_counts = compute_word_counts(&args.path);
    let word_counts_vec_most_frequent = get_most_frequent_words(word_counts, args.k);

    // Print in reverse order so we see the most common words at the bottom
    // of the screen.
    for (word, count) in word_counts_vec_most_frequent.iter().rev() {
        println!("{word}: {count}");
    }
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

    #[test]
    fn most_frequent() {
        assert_eq!(get_most_frequent_words(HashMap::from([
            ("hello", 3), 
            ("hi", 7), 
            ("howdy", 5)
        ]), Some(1)), vec![(String::from("hi"), 7)]);
    }

    #[test]
    fn most_frequent_2() {
        assert_eq!(get_most_frequent_words(HashMap::from([
            ("hello", 3), 
            ("hi", 7), 
            ("howdy", 5)
        ]), Some(2)), vec![
            (String::from("hi"), 7),
            (String::from("howdy"), 5)
        ]);
    }
    #[test]
    fn most_frequent_all() {
        assert_eq!(get_most_frequent_words(HashMap::from([
            ("hello", 3), 
            ("hi", 7), 
            ("howdy", 5)
        ]), None), vec![
            (String::from("hi"), 7),
            (String::from("howdy"), 5),
            (String::from("hello"), 3) 
        ]);
    }
}
