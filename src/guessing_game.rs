// calls to external dependencies. rand must be defined in cargo.toml to be used here
use rand::Rng;
//multiple items can be brought into scope using nested paths
use std::{cmp::Ordering, io};

fn get_max() -> u32 {
    println!("Choose your difficulty level. Enter the corresponding number:");
    println!("(1) easy, (2) normal, (3) hard");
    println!("Invalid selection will result in a normal game.");

    let mut selection: String = String::new();

    io::stdin()
        .read_line(&mut selection)
        .expect("Failed to read line");

    let selection: u32 = selection.trim().parse().unwrap_or(2);

    let max: u32 = match &selection {
        1 => 10,
        3 => 1000,
        _ => 100,
    };

    max
}

fn game_loop(secret: u32, max: u32) {
    loop {
        println!("Guess a number between 1 and {}!", max);

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if !(1..=max).contains(&guess) {
            println!("Guess value must be between 1 and {}, got {}", max, guess);
            continue;
        }

        println!("You guessed: {}", guess);

        match guess.cmp(&secret) {
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
    let max: u32 = get_max();
    let secret_number: u32 = rand::thread_rng().gen_range(1..=max);

    game_loop(secret_number, max);
}
