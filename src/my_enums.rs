#[derive(Debug)]
enum Color {
    RED,
    GREEN,
    BLUE,
}

pub fn create_an_enum() {
    let my_color = Color::RED;
    let my_second_color = Color::GREEN;
    let my_third_color = Color::BLUE;
    println!("My color is {:?}", my_color);
    println!("My color is {:?}", my_second_color);
    println!("My color is {:?}", my_third_color);
}

pub fn create_an_enum_with_data_types() {
    // These could be 4 separate structs but they wouldn't be grouped together into
    // the `Message` type
    #[derive(Debug)]
    enum Message { 
        _Quit,
        _Move { x: i32, y: i32 }, // Struct
        Write(String), // Tuple struct
        _ChangeColor(i32, i32, i32), // Tuple struct
    }

    impl Message {
        fn call(&self) {
            println!("Self is: {:?}", &self);
        }
    }

    // Create an Enum of type `Message` with the value `Write`
    let m1 = Message::Write(String::from("hello"));
    m1.call();
    // let m2 = Message::Move { y: 5, x: 10 };
    // m2.call();
}
