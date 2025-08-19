// 5.0 => Structs - Defining/Using Structs

struct User {
    user_name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        email: String::from("user@mail.com"),
        user_name: String::from("Kartik"),
        sign_in_count: 2,
        active: true,
    };

    let name = user1.user_name;
    println!("User name: {}", name);
    user1.user_name = String::from("Shiva"); // needs mutable struct
    println!("User name: {}", user1.user_name);
}
