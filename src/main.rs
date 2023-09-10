fn main() {
    println!("Hello World!");
    print_hardcoded_string();
    print_an_integer(7);
    print_height(188, "cm");
    println!("{}", five());
    println!("5 + 1 is: {}", plus_one(5));
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

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
