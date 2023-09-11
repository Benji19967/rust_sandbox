pub struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


pub fn create_user() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    println!("{}", user1.active);
    println!("{}", user1.username);
    println!("{}", user1.email);
    println!("{}", user1.sign_in_count);
}

pub fn create_and_update_user() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("other@example.com");
}

pub fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username: username,
        email, // Field init shorthand, same as `email: email`
        sign_in_count: 1,
    }
}

pub fn from_other_user(user: User) -> User {
    User {
        active: false,
        ..user
    }
    // user no longer valid, data gets moved.
    // If all data types of the struct implemented `Copy` then `user` would still
    // be valid
}

pub fn tuple_structs() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    // Different types, eventhough all elements of the tuples have the same types
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
}

pub fn unit_like_structs() {
    struct AlwaysEqual;

    let _subject = AlwaysEqual;
}

pub fn debug_struct() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 width: {}", rect1.width);
    println!("rect1 height: {}", rect1.height);
    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);
}

pub fn debug_struct_2() {
    // Prints file and line number of debug statements!
    // `dbg!` returns the owenership of the expression's value
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1); // Reference, so `dbg!` does not take the owenership of `rect1`
}
