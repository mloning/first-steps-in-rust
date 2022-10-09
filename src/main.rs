use core::num;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Hi! Let's play a guessing game.");
    let number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {number}");

    println!("Guess the number between 0 and 100!");
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };
        println!("You guessed: {guess}");

        match guess.cmp(&number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
