// 4.2 => Ownership & Borrowing - Memory Allocation

fn main() {
    let x = 5;
    let y = x; // bitwise-copy
    println!("{} {}", x, y);

    let s1 = String::from("Rust is awesome!");
    let s2 = s1; // move => value moved to s1

    // println!("{}", s1); // error => borrow of moved value: s1
    println!("{}", s2);

    let s3 = s2.clone(); // deep-copy
    println!("{} {}", s2, s3); // s2 is still valid
}
