// 5.4 => Structs - Method Syntax

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let a = Rectangle {
        width: 30,
        height: 18,
    };

    let b = Rectangle {
        width: 88,
        height: 59,
    };

    println!("The area of rectangle is {} sq pixels", a.area());
    println!("{:#?}", a);

    println!("a can hold b: {}", a.can_hold(&b));
    println!("b can hold a: {}", b.can_hold(&a));
}

/*
Notes:
- Primitive types like Integers, Flaoting, Boolean & Character implements the Display trait
- Custom types like struct needs custom implementation
- `impl` methods are tied to instance of `struct` but not associative functions
*/
