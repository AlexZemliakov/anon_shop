use backend::AppState;
use std::sync::{Arc, RwLock};
use std::io;

#[tokio::main]
async fn main() {
    let state = AppState {
        message: Arc::new(RwLock::new("Hello World".to_string())),
    };

    loop {
        println!("Current message: {}", state.message.read().unwrap());
        println!("Enter new message (or 'quit' to exit):");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "quit" {
            break;
        }

        *state.message.write().unwrap() = input.to_string();
        println!("Message updated!");
    }
}