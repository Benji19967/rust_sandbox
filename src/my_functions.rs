pub fn print_hardcoded_string() {
    println!("Printing a hardcoded string.");
}

pub fn print_an_integer(x: i32) {
    println!("The value of x is {x}");
}

pub fn print_height(height: f32, unit: &str) {
    println!("The height is: {height}{unit}")
}

pub fn five() -> i32 {
    5
}

pub fn plus_one(x: i32) -> i32 {
    x + 1
}

pub fn fibonacci(mut n:  i32) -> i32 {
    // 0, 1, 1, 2, 3, 5, ...
    // F_0 = 0, F_1 = 1, F_2 = 1, F_3 = 2, ...
    let mut number = 0;
    let mut next = 1;
    while n > 0 {
        let temp = next;
        next = number + next;
        number = temp;
        n -= 1;
    }
    number
}
