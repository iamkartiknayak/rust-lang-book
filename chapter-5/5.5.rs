// 5.5 => Structs - Associative Functions

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// first impl
impl Rectangle {
    // other impl....
}

// second impl
impl Rectangle {
    fn build() -> Rectangle {
        Rectangle {
            width: 30,
            height: 12,
        }
    }
}

fn main() {
    let rect = Rectangle::build();
    println!("{:#?}", rect);
}

/*
Notes:
- Structs allow multiple `impl` blocks
- Inside inside `impl` block if fn takes
    - `&self` as first param, then it gets called on instance (eg: rect.area())
    - `&self` not as first param, then it gets called on Type (eg: Rectangle::build())
*/
