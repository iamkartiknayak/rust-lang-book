// 6.0 => Enums & Pattern Matching - Defining Enums

enum IpAddrKind {
    V4, // called a variant
    V6,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
}

/*
Notes:
- Variants are namespaced under their identifier, this `::`
*/
