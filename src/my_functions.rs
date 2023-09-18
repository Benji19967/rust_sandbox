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

pub fn fibonacci(mut n:  u128) -> u128 {
    // 0, 1, 1, 2, 3, 5, ...
    // F_0 = 0, F_1 = 1, F_2 = 1, F_3 = 2, ...
    
    // n: i32  --> max is 45th  fibonacci number before overflowing
    // n: u32  --> max is 46th  fibonacci number before overflowing
    // n: i64  --> max is 91st  fibonacci number before overflowing 
    // n: u64  --> max is 92nd  fibonacci number before overflowing 
    // n: i128 --> max is 183rd fibonacci number before overflowing 
    // n: u128 --> max is 185th fibonacci number before overflowing 
    // 185th takes ~50-70 microseconds to compute on MacBook Pro M1 
    // 185th takes ~18-20 microseconds to compute on MacBook Pro M1 when using 
    // `cargo ... --release`
    // 1hr later, re-running now only takes 375-500 nanoseconds with `--release`
    // and 2.5 to 3.5 microseconds without it. I wonder why it's faster now...
    // Python takes similar amount of time: ~30 microseconds
     
    use std::time::Instant;
    let now = Instant::now();

    let mut number = 0;
    let mut next = 1;
    while n > 0 {
        let temp = next;
        next = number + next;
        number = temp;
        n -= 1;
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn five_is_five() {
        assert_eq!(five(), 5);
    }
}
