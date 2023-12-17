// calls to external dependencies. rand must be defined in cargo.toml to be used here
use rand::Rng;
//multiple items can be brought into scope using nested paths
use std::{cmp::Ordering, io};

// pub struct Guess {
//     value: u32,
// }

// impl Guess {
//     pub fn check(value: u32) -> Guess {
//         if !(1..=100).contains(&value) {
//             println!("Guess value must be between 1 and 100, got {}", value);
//         }

//         Guess { value }
//     }

//     pub fn value(&self) -> u32 {
//         self.value
//     }
// }

struct Max {
    value: u32,
}

impl Max {
    pub fn choose() {
        todo!();
    }
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

        if !(1..=100).contains(&guess) {
            println!("Guess value must be between 1 and 100, got {}", guess);
            continue;
        }

        // Guess::check(guess);

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
    let upper_bound: u32 = 100;

    let secret_number: u32 = rand::thread_rng().gen_range(1..=upper_bound);

    game_loop(secret_number, upper_bound);
}
