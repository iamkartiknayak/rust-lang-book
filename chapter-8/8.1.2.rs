// 8.1.2 => - Common Collections in Rust - String

use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let hello = String::from("Здравствуйте");
    // let first_char = hello[0]; // the type str cannot be indexed by {integer}

    let namaste = String::from("नमस्ते");

    // Bytes
    // [224, 164, 168, 224, 174, 224, 164, 184, 224, .....]
    for byte in namaste.bytes() {
        print!("{}, ", byte);
    }
    println!("");

    // Scalar values
    // ['न', 'म', 'स्', 'त', ' े']
    for char in namaste.chars() {
        print!("{}, ", char);
    }

    // Grapheme clusters
    //  ["न", "म", "स्", "ते"]
    for grapheme in namaste.graphemes(true) {
        print!("{}, ", grapheme);
    }
}

/*
[dependencies]
unicode-segmentation = "1.12.0"
*/
