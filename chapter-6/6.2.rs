// 6.2 => Enums & Pattern Matching - Values inside Enums

enum IpAddrKind {
    V4(u8, u8, u8, u8), // called a variant
    V6(String),
}

// example enum to demo the types, one enum can have
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let local_host = IpAddrKind::V4(127, 0, 0, 1);
}
