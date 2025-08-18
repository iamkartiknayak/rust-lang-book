// 2.0 => Guessing game

use std::{
    cmp::Ordering,
    io::{self, Write},
};

use colored::Colorize;
use rand::Rng;

fn main() {
    println!("Guessing game...");

    let secret_number = rand::rng().random_range(0..=100);
    let mut attempts = 7;

    loop {
        print!("Enter your guess: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            _ => {
                println!("Invalid input!\n");
                continue;
            }
        };

        attempts -= 1;
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too low".yellow()),
            Ordering::Greater => println!("{}", "Too high".bright_red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                println!("Number of attempts left: {}", attempts);
                break;
            }
        }

        if attempts == 0 {
            println!("{}", "You Lost!".red());
        }
    }
}

/*
[dependencies]
rand = "0.9.2"
colored = "3.0.0"
*/
