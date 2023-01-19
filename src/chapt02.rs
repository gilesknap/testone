// Chapter 2: Guessing Game

use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn guessing_game() {
    let mut guesses: u32 = 10;

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess. {} guesses left", guesses);

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please enter a number");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        guesses = guesses - 1;
        if guesses == 0 {
            println!("You lose!");
            break;
        }
    }
}
