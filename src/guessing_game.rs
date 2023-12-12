// calls to external dependencies. rand must be defined in cargo.toml to be used here
use rand::Rng;
//multiple items can be brought into scope using nested paths
use std::{cmp::Ordering, io};

fn game_loop(secret: u32, lower: u32, upper: u32) {
    loop {
        println!("Guess a number between {} and {}!", lower, upper);

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

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
    let lower_bound: u32 = 1;
    let upper_bound: u32 = 100;

    assert_eq!(
        upper_bound.cmp(&lower_bound),
        Ordering::Greater,
        "The upper bound must be higher than the lower bound!"
    );

    let secret_number: u32 = rand::thread_rng().gen_range(lower_bound..=upper_bound);

    game_loop(secret_number, lower_bound, upper_bound);
}
