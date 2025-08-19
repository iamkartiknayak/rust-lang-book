// 3.0 => Common concepts - Mutability & Shadowing

fn main() {
    let a = 10;
    println!("Value of a is {}", a);
    // a = 20; // error => cannot mutate immutable variable

    let mut b = 20;
    println!("Value of b is {}", b);
    b = 30; // valid => mutable var
    println!("Value assigned to `mut` b is {}", b);

    const PI_VALUE: f32 = 3.14159;

    let c = 32;
    println!("Value of c is {}", c);
    let c = "Thirty Two"; // shadowing
    println!("Value of c is {c}"); // also valid print
}

/*
Notes:
- All variables are immutable by default
- Only `mut` variables can be mutated
- `const` must be always type annotated
- `const` type can never be `mut`
- Shadowing => used to change type & value of a var while maintaining immutability
*/
