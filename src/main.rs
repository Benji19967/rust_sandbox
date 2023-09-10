fn main() {
    // Hello World!
    // println!("Hello World!");


    print_hardcoded_string();
    print_an_integer(7);
    print_height(188, "cm");
}

fn print_hardcoded_string() {
    println!("Printing a hardcoded string.");
}

fn print_an_integer(x: i32) {
    println!("The value of x is {x}");
}

fn print_height(height: i32, unit: &str) {
    println!("The height is: {height}{unit}")
}
