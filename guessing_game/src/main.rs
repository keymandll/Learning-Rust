use std::io::{self, Write};
use std::cmp::Ordering;
use rand::Rng;

const COMMAND_QUIT: &str = "quit";

fn prompt_user() -> String {
    print!("Enter your guess: ");
    io::stdout().flush().expect("Failed to flush stdout.");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    guess.trim().to_string()
}

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut counter: u32 = 0;
    let rounds_played: u32 = loop {
        let input = prompt_user();
        if input == COMMAND_QUIT {
            break counter;
        }
        counter += 1;
        let guess: u32 = match input.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break counter;
            }
        }
    };
    println!("You have played {rounds_played} rounds.");
}
