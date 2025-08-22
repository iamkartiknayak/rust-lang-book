// 8.1.0 => - Common Collections in Rust - String

fn main() {
    // Strings are stored as a collection of UTF-8 bytes

    let s1 = String::new();
    let s2 = "initial contents";
    let s3 = s2.to_string();
    let s4 = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');
    println!("{}", s);
}

/*
Notes:
- `push_str` used to concatenate a string
- `push` is used to concatenate a single character
*/
