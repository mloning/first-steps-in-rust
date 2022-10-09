use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // intro message
    println!("Hi! Let's play a guessing game.");

    // generate secret number
    let min: u32 = 1;
    let max: u32 = 100;
    let number: u32 = rand::thread_rng().gen_range(min..=max);
    println!("The secret number is: {number}");

    // repeat until game is over
    println!("Guess the number between {min} and {max}!");
    loop {
        // take user input
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // handle invalid input
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        // check input against secret number
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
