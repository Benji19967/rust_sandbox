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
