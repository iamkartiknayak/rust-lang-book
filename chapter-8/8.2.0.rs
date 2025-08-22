// 8.2.0 => - Common Collections in Rust - Hashmaps

use std::collections::HashMap;

fn main() {
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();

    scores.insert(blue, 10);
    scores.insert(yellow, 40);

    // println!("{}", blue); // error => borrow of moved value: blue

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("{:?}", score);

    for (key, value) in scores {
        println!("{}: {}", key, value);
    }
}

/*
Notes:
- A HashMap<K, V> is a unordered collection of key–value pairs where
    - Each key is unique.
    - Values are looked up by their key using hashing function
    - Keys and values can be any type that implements the required traits (Eq and Hash for keys).

- Hashmaps are fast, unordered and ownership aware (inserts move keys/values) unless borrowed types are used
*/
