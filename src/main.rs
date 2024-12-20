use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("--- Guess the number ---");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let Ok(guess) = guess.trim().parse::<i32>() else {
            println!("Failed to parse number");
            continue;
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