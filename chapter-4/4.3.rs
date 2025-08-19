// 4.3 => Ownership & Borrowing - Ownership & Functions

fn main() {
    let a = String::from("Rust!");
    takes_ownership(a); // value moved from a to function
    // println!("{}", user_name) // error => borrow of moved value: user_name

    let b = 12;
    let c = "Hello Rust!";
    makes_copy(b, c); // values got bitwise-copied, (i32 & &str implements Copy)
    println!("{}, {}", b, c); // valid => both gets bitwise copied

    let d = gives_ownership(); // value returned and given ownership to d
    println!("{}", d);

    let e = String::from("COSMIC DE");
    let f = takes_and_gives_back(e); // move from e to function then return to f
    // println!("{}", e); // error => value moved to f
    println!("{}", f);
}

fn takes_ownership(some_string: String) {
    println!("User name is {}", some_string)
}

fn makes_copy(some_integer: i32, some_str: &str) {
    println!("{}, {}", some_integer, some_str);
}

fn gives_ownership() -> String {
    let some_string = String::from("Rust is awesome!");
    some_string
}

fn takes_and_gives_back(some_string: String) -> String {
    some_string
}
