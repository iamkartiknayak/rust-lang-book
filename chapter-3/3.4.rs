// 3.4 => Common concepts - Control Flow

fn main() {
    let number = 12;

    if number < 10 {
        println!("{} is less than 10", number);
    } else if number < 22 {
        println!("{} is in-between 10 & 22", number);
    } else {
        println!("{} is greater than 22", number);
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("{}", number)
}
