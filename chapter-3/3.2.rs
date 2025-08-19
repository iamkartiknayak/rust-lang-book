// 3.2 => Common concepts - Compound Data Types

fn main() {
    // Tuples
    let tup = ("Rust is awesome!", 100_000);
    let (string, mut number) = tup;
    println!("{}, {}", string, number);

    let string = tup.0;
    number = tup.1;
    // let something = tup.3; // error (compile-time) => unknown field

    // Arrays
    let error_code = [200, 404, 500];
    let not_found = error_code[1];
    let x = error_code[3]; // error (runtime) => out of bounds exception

    let byte = [0; 8]; // array filled with 8 `0`s
    print!("{:?}", byte);
}

/*
Notes:
- Arrays => collection of homogeneous elements
    - is iterable
- Tuples => collection of possibly heterogeneous elements
    - is not iterable
- Both types have fixed size known at compile time
*/
