// 8.0.0 => - Common Collections in Rust - Vectors

fn main() {
    let v = vec![1, 2, 3]; // Rust auto infers the type

    // let v1 = Vec::new(); // error => immutable & not type annotated
    // let mut v2= Vec::new(); // error => mutable & type not annotated or possibility of inference as var `v2` isn't used later
    let mut v3 = Vec::new(); // can skip type annotation as it was made clear in next statement

    v3.push(1);
    v3.push(2);
    v3.push(3);

    let third = v3[2];
    println!("Third value is {}", third);
    // let value = v[20]; // runtime error => out of bounds

    let value = v.get(20); // returns `Option` type
    // println!("{:?}", value);
    let index = 2;
    match v.get(index) {
        Some(value) => println!("Value at index {} is {}", index, value),
        None => println!("{} is a invalid index", index),
    };
}

/*
Notes:
- Immutable empty inits must be annotated, since type cannot change later.
- Mutable empty inits can skip annotation only if later usage makes the type unambiguous.
- `out of bounds` error triggers
    - compile time error for arrays
    - runtime error for vectors
*/
