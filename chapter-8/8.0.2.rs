// 8.0.2 => - Common Collections in Rust - Vectors

use std::vec;

fn main() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("Indigo")),
        SpreadsheetCell::Float(10.12),
    ];

    match &row[0] {
        SpreadsheetCell::Int(i) => println!("{}", i),
        _ => println!("Not a integer"),
    }
}
