// This is the crate root file for this crate

use std::io;
use crate::my_functions::{
    plus_one, five, print_height, print_hardcoded_string, print_an_integer, fibonacci
};
// use crate::my_functions::* // bad practice
use crate::my_structs::{debug_struct, debug_struct_2};
use crate::my_enums::create_an_enum_with_data_types;

pub mod my_functions;
pub mod my_structs;
pub mod my_enums;
pub mod my_io;


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

    println!("Which fibonacci number do you want to compute?");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    let nth_fibonacci: u128 = user_input.trim().parse().expect("Input not an integer");
    println!("The {}th fibonacci number is {}", nth_fibonacci, fibonacci(nth_fibonacci));

    // --- STRUCTS ---
    println!("\n--- STRUCTS ---");
    debug_struct();
    debug_struct_2();

    // --- ENUMS ---
    println!("\n--- ENUMS ---");
    create_an_enum_with_data_types();
}
