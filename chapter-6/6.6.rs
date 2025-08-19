// 6.6 => Enums & Pattern Matching - `if let` Syntax

fn main() {
    let some_value = Some(3);
    match some_value {
        Some(3) => println!("three"),
        _ => println!("Not three"),
    }

    if let Some(3) = some_value {
        println!("three");
    }
}

/*
Notes:
- Match expression needs to be exhaustive
- `if let` only care about matching one pattern
*/
