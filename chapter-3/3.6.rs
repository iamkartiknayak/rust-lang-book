// 3.6 => Common concepts - `for`

use std::vec;

fn main() {
    // looping on an array
    let numbers = [1, 3, 5, 7, 9];

    for number in numbers {} // uses `.into_iter()` behind the scenes; elements are Copy, so array is not moved
    for number in numbers.iter() {} // explicitly borrowing
    println!("{:?}", numbers); // valid because `i32` is Copy

    for number in numbers.into_iter() {} // explicitly calls IntoIterator::into_iter(), still works due to Copy
    println!("{:?}", numbers); // valid => `i32` is Copy

    let cats = [
        String::from("Whsikers"),
        String::from("Toby"),
        String::from("Luna"),
    ];

    // ❌ this would move the array and its elements
    // for _ in cats {} // moves each String (not Copy), so `cats` is moved
    // println!("{:?}", cats); // error: use of moved value

    for _ in cats.iter() {} // borrows elements immutably
    println!("{:?}", cats); // valid: `.iter()` only borrows

    let dogs = ["Charlie", "Bella", "Max"]; // array of &str (string literals)

    for _ in dogs.iter() {} // borrowing references
    println!("{:?}", dogs); // still usable

    for _ in dogs.into_iter() {} // `&str` is Copy, so although `into_iter()` yields value, values are copied, not moved & array remains valid
    println!("{:?}", dogs); // `&str` is Copy

    // looping on a vector
    let numbers = vec![1, 3, 5, 7, 9];

    for _ in numbers.iter() {} // borrows elements
    println!("{:?}", numbers); // valid: `.iter()` only borrows

    // for _ in numbers {} // uses `.into_iter()` and moves the vector and its contents
    // println!("{:?}", numbers); // error => moved value

    for _ in numbers.into_iter() {} // moves the vector and its elements
    // println!("{:?}", numbers); // error => use of moved value

    // `for` with `range`
    for number in 1..=5 {
        print!("{} ", number) // standard range loop
    }
}

/*
Notes:
- `.iter()` creates a reference to each element (never moves).
- `.into_iter()` moves the collection and its elements (unless elements are Copy).
- `for` uses:
    - `.into_iter()` for arrays by default (regardless of type), but:
        - If elements are Copy (e.g. integers, `&str`), it looks like a borrow (but actually copied).
        - If elements are not Copy (e.g. `String`), it moves them.
    - `.into_iter()` always moves a `Vec<T>`, regardless of T being Copy or not.
    - `.iter()` should be used when you want to borrow and keep using the collection after the loop.
*/
