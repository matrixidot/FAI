use std::cmp::Ordering;
use std::io;
use std::io::Write;
use rand::prelude::*;

fn main() {
    println!("Guessing game");

    let num = rand::rng().random_range(1..=100);
    let mut guesses = 1;
    loop {
        print!("Guess #{guesses}\nPlease input your guess: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        guess = guess.trim().to_string();

        let guess: u32 = match guess.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{guess} Is not a valid integer.");
                continue;
            }
        };

        println!("You guessed: {guess}!");

        match guess.cmp(&num) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Large"),
            Ordering::Equal => {
                println!("You Won in {guesses} guesses!");
                break;
            },
        }
        guesses += 1;
    }
}
