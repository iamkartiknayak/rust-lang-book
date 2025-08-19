// 5.3 => Structs - Example Use Case

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of rectangle is {} sq pixels", area(&rect));
    println!("{:#?}", rect);
}

fn area(dimensions: &Rectangle) -> u32 {
    dimensions.width * dimensions.height
}

/*
Notes:
- Primitive types like Integers, Flaoting, Boolean & Character implements the Display trait
- Custom types like struct needs custom implementation
*/
