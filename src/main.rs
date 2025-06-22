use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let mut how_many = String::new();
    println!("How many numbers do you want to guess?");
    io::stdin()
        .read_line(&mut how_many)
        .expect("Error reading input");

    let num_guesses: u8 = how_many.trim().parse().expect("Error ");

    let mut correct = Vec::new();

    for _ in 0..num_guesses {
        correct.push(rand::rng().random_range(1..=10));
    }

    println!("{correct:?}");

    let mut guesses_made = 0;

    println!("Guess a number");

    while guesses_made < num_guesses {
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

        match guess.cmp(&correct[guesses_made as usize]) {
            Ordering::Greater => println!("You guessed too high"),
            Ordering::Less => println!("You guessed too low"),
            Ordering::Equal => {
                println!("You guessed correct!");
                guesses_made += 1;
                if guesses_made < num_guesses {
                    println!("Guess the next number");
                }
            }
        };
    }
    println!("Thanks for playing! The correct numbers were:");
    for item in correct {
        println!("{item}");
    }
}
