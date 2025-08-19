// 6.5.2 => Enums & Pattern Matching - Match Expression (2)

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    println!("{:?}", six);

    let none = plus_one(None);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

/*
Notes:
- In this scenario of match we only have 2 possibilities i.e `Some()` & `None`
- If there are several possibilities than `_ =>` can be used to accomodate rest after required match hands
- `_` is called wild-card pattern (catch-all)
*/
