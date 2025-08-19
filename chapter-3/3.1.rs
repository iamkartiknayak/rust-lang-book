// 3.1 => Common concepts - Scalar Data Types

fn main() {
    // Integers
    let a = 98_222; // Decimal
    let b = 0xff; // Hex
    let c = 0o77; // Octal
    let d = 0b1111_000; // Binary
    let e = b'A'; // Byte

    let e: u8 = 255; // from 0 to upto 255 is valid
    // let f: u8 = 256; // error => integer overflow

    // Floating-point numbers
    let f = 24.98;
    let g = 231237366388.23;

    // Bolleans
    let h = true;
    let i = false;

    // Character
    let j = 'J';
    let k = '🪁';

    // println!("{}", 12 + 5.2); // error => mismatched type
    println!("{}", 12 as f64 * 5.2); // explicit casting
}

/*
Notes:
- Integer (default: i32)
    - in debug mode overflow panics
    - in release mode overflow result in 2's complement wrapping:
        256 = 0
        257 = 1
- Float (default: f64)
- There's no concept of coersion (implicit casting)
*/
