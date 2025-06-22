use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let correct = rand::rng().random_range(1..=10);
    println!("Correct: {correct}");
    println!("Guess a number");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Error with parse: {e}");
                continue;
            }
        };

        match guess.cmp(&correct) {
            Ordering::Greater => println!("You guessed too high"),
            Ordering::Less => println!("You guessed too low"),
            Ordering::Equal => {
                println!("You guessed correct!");
                break;
            }
        };
    }
}
