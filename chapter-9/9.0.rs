// 9.0 => - Error Handling - `panic!("msg")`

use std::panic;

fn main() {
    a();
}

fn a() {
    b();
}

fn b() {
    // c(22); // crashes the program if value is 22
    c(23);
}

fn c(num: i32) {
    if num == 22 {
        panic!("Don't pass in 22!");
    }
}

/*
Notes:
- Using `RUST_BACKTRACE=1 cargo run` can help trace error back to origin
*/
