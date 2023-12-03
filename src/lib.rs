pub mod hello_world {
    pub fn hello() {
        println!("Hello from the crate library!")
    }
}

pub mod guessing_game {
    use rand::Rng;
    use std::cmp::Ordering;
    use std::io;

    pub fn guessing() {
        println!("Guess a number between 1 and 100!");

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
}
