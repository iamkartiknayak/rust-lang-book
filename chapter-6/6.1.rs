// 6.1 => Enums & Pattern Matching - Enums in Structs

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let local_host = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("192.123.1.1"),
    };

    println!(
        "Local Host\nKind: {:?}\nAddress: {}",
        local_host.kind, local_host.address
    )
}
