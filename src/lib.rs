// example module. pubic so that it's callable from elsewhere
pub mod hello_world {
    // function must also be public to be callable
    pub fn hello() {
        println!("Hello from the crate library!")
    }
}

// second public module in the same library
pub mod guessing_game {
    // calls to external dependencies. rand must be defined in cargo.toml to be used here
    use rand::Rng;
    use std::cmp::Ordering;
    use std::io;

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
}