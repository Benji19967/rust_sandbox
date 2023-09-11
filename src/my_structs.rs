struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

pub fn create_user() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
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
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

pub fn unit_like_structs() {
    struct AlwaysEqual;

    let subject = AlwaysEqual;
}
