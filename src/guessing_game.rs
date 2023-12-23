// calls to external dependencies. rand must be defined in cargo.toml to be used here
use rand::Rng;
//multiple items can be brought into scope using nested paths
use std::{cmp::Ordering, io};

struct UserInput {
    value: String,
}

impl UserInput {
    fn new_from_stdin() -> Self {
        let mut value = String::new();
        io::stdin()
            .read_line(&mut value)
            .expect("Failed to read line");
        UserInput { value }
    }
}

fn get_max() -> u32 {
    println!("Enter your desired difficulty level:");
    println!("(1) easy, (2) normal, (3) hard");
    println!("Invalid selection will result in a normal game.");

    let selection = UserInput::new_from_stdin();

    let selection: u32 = selection.value.trim().parse().unwrap_or(2);

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

        let guess = UserInput::new_from_stdin();

        let guess: u32 = match guess.value.trim().parse() {
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
