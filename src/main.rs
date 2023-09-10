// This is the crate root file for this crate

use crate::my_functions::{
    plus_one, five, print_height, print_hardcoded_string, print_an_integer
};
// use crate::my_functions::* // bad practice

pub mod my_functions;


const CENTIMETERS_IN_METERS: i32 = 100;

fn main() {
    // --- FUNCTIONS ---
    println!("Hello World!");
    print_hardcoded_string();
    print_an_integer(7);

    // Example of casting an i32 to a f32
    print_height(1.88 * CENTIMETERS_IN_METERS as f32, "cm");

    println!("{}", five());
    println!("5 + 1 is: {}", plus_one(5));
}

