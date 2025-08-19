// 4.5 => Ownership & Borrowing - Slicing

fn main() {
    let mut some_string = String::from("Rust is awesome!");
    let first_word = get_first_word(&some_string);
    // some_string.clear(); error => cannot borrowed as mutable cuz it already is borrowed as immutable
    println!("First word is {}", first_word);

    let mut hello_world = String::from("Hello World!");
    let world = &mut hello_world[6..]; // mutable referance to the String in heap
    world.make_ascii_uppercase();

    println!("{}", hello_world);

    let numbers = [1, 3, 5, 7, 9];
    let slice = &numbers[2..4];
    println!("{:?}", slice);
}

fn get_first_word(some_string: &String) -> &str {
    let bytes = some_string.as_bytes();

    for (i, &char) in bytes.iter().enumerate() {
        if char == b' ' {
            // &some_string[..i] // invalid => only works if the last line of a fn
            return &some_string[..i];
        }
    }
    &some_string[..]
}

/*
Notes:
- `&str` is called a fat pointer,
    - it has 2 components:
        - pointer to the start of the slice, based on where data is created it could live in
            - Heap → if it comes from a String (e.g. let s = String::from("Hi"); let slice: &str = &s;)
            - Static binary segment (.rodata) → if it’s a string literal (e.g. let slice: &str = "Hi";)
            - Stack → only if you slice into a stack-allocated array of bytes/chars (rare, but possible)
        - length of the slice
- String has 3 components
    - A pointer to heap buffer.
    - Length.
    - Capacity.
*/
