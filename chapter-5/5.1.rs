// 5.1 => Structs - Function Constructors

struct User {
    user_name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mike = build_user(String::from("mikeross@mail.com"), String::from("Mike Ross"));

    let louis = User {
        email: String::from("louis@litt.up"),
        user_name: String::from("Louis Litt"),
        ..mike // reusing instance data => other 2 properties of mike is copied
    };
}

fn build_user(email: String, user_name: String) -> User {
    User {
        email, // field init shorthand syntax
        user_name,
        sign_in_count: 0,
        active: false,
    }
}
