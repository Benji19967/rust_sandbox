// The little book of Rust macros: https://veykril.github.io/tlborm/

// To see the expanded expressions: `cargo expand --bin <name_of_binary>`

// Empty match arm
macro_rules! four {
    () => { 1 + 3 };
}

// capture input as an expression under the metavariable $e
macro_rules! times_five {
    ($e:expr) => { 5 * $e };
}

// Multiple metavariables in a single matcher
macro_rules! multiply_add {
    ($a:expr, $b:expr, $c:expr) => { $a * ($b + $c) };
}

macro_rules! vec_strs {
    (
        // Start a repetition:
        $(
            // Each repeat must contain an expression...
            $element:expr
        )
        // ...separated by commas...
        ,
        // ...zero or more times.
        *
    ) => {
        // Enclose the expansion in a block so that we can use
        // multiple statements.
        {
            let mut v = Vec::new();

            // Start a repetition:
            $(
                // Each repeat will contain the following statement, with
                // $element replaced with the corresponding expression.
                v.push(format!("{}", $element));
            )*

            v
        }
    };
}


fn main() {
    println!("Four: {}", four!());
    println!("Four: {}", four![]);
    println!("Four: {}", four!{});

    println!("Times five: {}", times_five!(7));

    println!("Multiply add: {}", multiply_add!(2, 3, 4));
    println!("Multiply add: {}", multiply_add![2, 3, 4]);
    println!("Multiply add: {}", multiply_add!{2, 3, 4});

    let s = vec_strs![1, "a", true, 3.14159f32];
    println!("Vec strs: {:?}", s);
    let s: Vec<&str> = vec_strs![];
    println!("Vec strs: {:?}", s);
}
