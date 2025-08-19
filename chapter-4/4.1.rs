// 4.1 => Ownership & Borrowing - Variable Scope

fn main() {
    // a & b are not valid here, they're not declared yet
    {
        let a = 5; // a is valid from this point forward
        let b = String::from("Hello Rust!"); // b is valid from this point forward
        // do stuff with a & b
    } // This scope is now over, and a & b are no longer valid
    // println!("{} {}", a, b);
}

/*
Notes:
------------------- Ownership Rules ---------------------
- Each value in Rust has a variable that's called it's owner
- There can only be one owner at a time
- When the owner goes out of scope, the value will be dropped
*/
