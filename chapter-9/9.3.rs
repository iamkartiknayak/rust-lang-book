// 9.3 => - Error Handling - Error Propagation

use std::{error::Error, fs::File};

// Doesn't work
// fn main() {
//     let f = File::open("hello.txt")?;
// }

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}

/*
Notes:
- `fn main()` is a special, it has retrictions on what types can it return
*/
