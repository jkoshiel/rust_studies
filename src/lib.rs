// example module. pubic so that it's callable from elsewhere
pub mod hello_world {
    // function must also be public to be callable
    pub fn hello() {
        println!("Hello from the crate library!")
    }
}

// second public module in the same library
// semicolon tells rust to look for the contents in a file with the same name as ths module
pub mod guessing_game;
