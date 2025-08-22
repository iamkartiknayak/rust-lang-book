// 8.2.2 => - Common Collections in Rust - Hashmaps

use std::collections::HashMap;

fn main() {
    let text = "Hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

/*
Notes:
 - `or_insert()` returns a mutable referance to the value
*/
