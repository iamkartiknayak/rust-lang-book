// 6.5.1 => Enums & Pattern Matching - Match Expression (1)

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
}

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Arizona));
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}
