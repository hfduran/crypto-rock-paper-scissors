use std::io::{self, Write};

use crate::model::{Mode, Play};

pub fn print_hello() {
    println!("WELCOME TO CRYPTO ROCK PAPER SCISSORS!");
}

pub fn get_play_input() -> Play {
    loop {
        print!("Please, make your play (r - Rock, p - Paper, s - Scissors: ");

        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let choice = input.trim().to_lowercase();

        match choice.as_str() {
            "r" => return Play::Rock,
            "p" => return Play::Paper,
            "s" => return Play::Scissors,
            _ => {
                println!("Invalid choice.");
            }
        }
    }
}

pub fn get_mode_input() -> Mode {
    loop {
        print!("Do you want to be the server or the client? (s - Server, c - Client): ");

        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let choice = input.trim().to_lowercase();

        match choice.as_str() {
            "s" => return Mode::Server,
            "c" => return Mode::Client,
            _ => {
                println!("Invalid choice.");
            }
        }
    }
}
