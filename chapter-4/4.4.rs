// 4.4 => Ownership & Borrowing - References & Borrowing

fn main() {
    let a = String::from("Rust!");
    let length = calculate_length(&a);
    println!("Length of `{}` is {}", a, length);

    let mut b = String::from("Rust");
    change_string(&mut b);
    println!("The message is `{}`", b);

    let mut c = String::from("Hello Rust!");

    let r1 = &c;
    let r2 = &c;
    let r3 = &c;
    println!("{} {}", r1, r2);

    let r4 = &mut c;
    // let r5 = &c; // error => cannot borrow c as immutable because it is also borrowed as mutable
    println!("{}", r4);

    let r6 = &mut c;
    // let r7 = &mut c; // error => cannot borrow c as mutable more than once at a time
    println!("{}", r6);

    let r8 = &c;
    let r9 = &mut c;
}

fn calculate_length(some_string: &String) -> usize {
    some_string.len()
}

fn change_string(some_string: &mut String) {
    some_string.push_str(" is awesome");
}

// function’s return type contains a borrowed value, but there is no value for it to be borrowed from
//error => missing lifetime specifier
// fn dangle() -> &String {
//     let some_string = String::from("Hello Rust!");
//     &some_string
// }

/*
Notes:
- Any number of immutable (&T) references allowed at the same time.
- At most one mutable (&mut T) reference allowed at a time.
- Immutable and mutable references cannot overlap.
    - Exception: they can both exist if the immutable borrow’s lifetime ends before the mutable one starts.
- Multiple mutable borrows are fine if their lifetimes don’t overlap.
- Lifetimes end at last use, not at scope end.
- Copy types (like i32, bool, &str) are duplicated by value (bitwise copy) → not affected by borrow rules.
- No dangling references: a reference must always be valid (Rust enforces this at compile time).
*/
