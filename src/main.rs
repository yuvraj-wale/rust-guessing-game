use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    println!("Guess the Number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {num},
            Err(_) => {
                println!("Invalid Number! Try again.");
                continue},
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!!!");
                break;
            }
            Ordering::Greater => println!("Too big!")
        }
    }
}
