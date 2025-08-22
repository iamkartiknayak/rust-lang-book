// 8.0.1 => - Common Collections in Rust - Vectors

fn main() {
    let mut v3 = vec![1, 3, 5, 7, 9];
    let second = &v3[1];
    // v3.push(6); // error => cannot borrow v3 as mutable because it is also borrowed as immutable
    println!("The second element is {}", second); // the error in above line only triggers if this line is un-commented

    for i in &mut v3 {
        *i += 50;
    }
    println!("{:#?}", v3);
}

/*
Notes:
- `*i` => de-reference operator
*/
