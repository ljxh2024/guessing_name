// https://kaisery.github.io/trpl-zh-cn/ch02-00-guessing-game-tutorial.html

use rand::Rng;
use std::{cmp::Ordering, io, error::Error};

fn main() {
    println!("--- Guess the number ---");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let guess = match check_input() {
            Ok(n) => n,
            Err(e) => {
                println!("{e}");
                continue;
            }
        };

        if !(1..=100).contains(&guess) {
            println!("The secret number will be between 1 and 100.");
            continue;
        }

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn check_input() -> Result<i32, Box<dyn Error>> {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)?;
    let guess = guess.trim().parse::<i32>()?;
    Ok(guess)
}
