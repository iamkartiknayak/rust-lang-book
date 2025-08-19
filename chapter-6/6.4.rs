// 6.4 => Enums & Pattern Matching - Option Enum

fn main() {
    let some_number = Some(5);
    let some_string = Some("Hello Rust!");

    // let absent_number = None; // error => type annotations needed for Option<_> (rustc E0282)
    let absent_number: Option<i32> = None;

    let a = Some(2);
    let b = 8;
    let c: Option<i32> = None;
    // let sum = a + b; // error => cannot add {integer} to Option<{integer}>
    let sum = a.unwrap_or(0) + b + c.unwrap_or(0); // now valid
    println!("The sum is {}", sum);
}

/*
Notes:
- In Rust there's no concept of null, instead it has `Option` enum & it has 2 variants
    - Some(T)
    - None
*/
