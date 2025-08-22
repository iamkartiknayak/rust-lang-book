// 8.2.1 => - Common Collections in Rust - Hashmaps

use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 40); // overwrites the value

    scores.entry(String::from("Yellow")).or_insert(30); // adds value only if key didn't exist
    scores.entry(String::from("Yellow")).or_insert(40);

    println!("Blue: {:?}", scores.get("Blue"));
    println!("Yellow: {:?}", scores.get("Yellow"));
}
