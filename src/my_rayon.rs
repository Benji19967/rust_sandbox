// Rayon for concurrency: https://github.com/rayon-rs/rayon

use rayon::prelude::*;

pub fn sum_of_squares_sequential(input: &Vec<u64>) -> u64 {
    input.iter() // <-- just change that!
         .map(|&i| i / (i + 1))
         .sum()
}

pub fn sum_of_squares(input: &Vec<u64>) -> u64 {
    input.par_iter() // <-- just change that!
         .map(|&i| i / (i + 1))
         .sum()
}



