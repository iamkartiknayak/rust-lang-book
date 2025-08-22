// 8.1.1 => - Common Collections in Rust - String

fn main() {
    // Strings are stored as a collection of UTF-8 bytes

    let s1 = String::from("Hello, ");
    let s2 = String::from("Rust!");
    let s3 = s1 + &s2;
    // println!("{}", s1); // error => borrow of moved value: s1
    println!("1st: {}", s3);

    let s1 = String::from("Hello, ");
    let s2 = String::from("Rust!");
    let s3 = format!("{}{}", s1, s2);
    // println!("{}", s1); // error => borrow of moved value: s1
    println!("2nd: {}", s3);

    let s1 = String::from("Hello, ");
    let s2 = String::from("Rust!");
    let s3 = s1 + &s2;
    // println!("{}", s1); // error => borrow of moved value: s1
    println!("{}", s3);
}
