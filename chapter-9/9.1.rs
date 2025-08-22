// 9.1 => - Error Handling - Result

use std::{fs::File, io::ErrorKind};

fn main() {
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Error creating the file {:?}", error),
            },
            other_error => {
                panic!("Problem opening the file {:?}", other_error)
            }
        },
    };

    let f1 = File::open("hello_world.txt").expect("Failed to open hello_world.txt");
}

/*
Notes:
- Always consider using Result over panic!
- Only use `panic!` in exceptional cases where recovering is not possible
*/
