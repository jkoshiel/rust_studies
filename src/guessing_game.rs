// calls to external dependencies. rand must be defined in cargo.toml to be used here
use rand::Rng;
//multiple items can be brought into scope using nested paths
use std::{cmp::Ordering, io};

fn game_loop() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess:");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess is too low!"),
            Ordering::Greater => println!("Your guess is too high!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

pub fn guessing() {
    println!("Guess a number between 1 and 100!");

    game_loop();
}
