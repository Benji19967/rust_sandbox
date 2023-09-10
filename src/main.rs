const CENTIMETERS_IN_METERS: i32 = 100;

fn main() {
    println!("Hello World!");
    print_hardcoded_string();
    print_an_integer(7);

    // Example of casting an i32 to a f32
    print_height(1.88 * CENTIMETERS_IN_METERS as f32, "cm");

    println!("{}", five());
    println!("5 + 1 is: {}", plus_one(5));
}

fn print_hardcoded_string() {
    println!("Printing a hardcoded string.");
}

fn print_an_integer(x: i32) {
    println!("The value of x is {x}");
}

fn print_height(height: f32, unit: &str) {
    println!("The height is: {height}{unit}")
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
