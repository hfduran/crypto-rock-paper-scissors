mod cli;
mod crypto;
mod model;

use crate::{
    cli::{get_mode_input, get_play_input, print_hello},
    crypto::{create_nonce, hash_play},
    model::{FromJson, Message, Mode, Play, ToJson},
};
use anyhow::Result;
use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::mpsc,
    thread,
};

const ADDR: &'static str = "127.0.0.1:8080";

fn main() -> Result<()> {
    print_hello();

    let mode = get_mode_input();

    match mode {
        Mode::Server => run_server()?,
        Mode::Client => run_client()?,
    }

    Ok(())
}

fn run_server() -> Result<()> {
    let listener = TcpListener::bind(ADDR)?;

    let (stream, _) = listener.accept()?;
    game_start(stream)?;

    Ok(())
}

fn run_client() -> Result<()> {
    let stream = TcpStream::connect(ADDR)?;
    println!("Connected to server!");

    game_start(stream)?;

    Ok(())
}

fn for_each_message_read<F>(stream: &TcpStream, handler: F)
where
    F: Fn(String) -> bool,
{
    let buf_reader = BufReader::new(stream);
    for line in buf_reader.lines() {
        match line {
            Ok(msg) => {
                if handler(msg) {
                    break;
                }
            }
            Err(_) => {
                println!("\nConnection closed");
                break;
            }
        }
    }
}

fn game_start(mut stream: TcpStream) -> Result<()> {
    let read_stream = stream.try_clone()?;

    let (tx, rx) = mpsc::channel::<Message>();

    let read_thread_handle = thread::spawn(move || {
        // PHASE 1 - HASH EXCHANGE
        for_each_message_read(&read_stream, |msg| {
            let message = Message::from_json_str(&msg).unwrap();
            match message {
                Message::HashedPlay(_) => {
                    tx.send(message).unwrap();
                    true
                }
                _ => false,
            }
        });

        // PHASE 2 - EXPLANATION EXCHANGE
        for_each_message_read(&read_stream, |msg| {
            let message = Message::from_json_str(&msg).unwrap();
            match message {
                Message::Explanation{play:_, nonce:_} => {
                    tx.send(message).unwrap();
                    true
                }
                _ => false,
            }
        });
    });

    // PHASE 1 - HASH EXCHANGE
    let play = get_play_input();
    let (nonce, hashed_play) = commit_to_play(&play);
    let message = Message::HashedPlay(hashed_play);
    let message_json_str = message.to_json_str()?;
    stream.write_all(message_json_str.as_bytes())?;

    let opponent_hashed_play_message = rx.recv().unwrap();
    let opponent_hashed_play: String;
    if let Message::HashedPlay(val) = opponent_hashed_play_message {
        opponent_hashed_play = val;
    } else {
        panic!("Invalid message from opponent!");
    }

    // PHASE 2 - EXPLANATION EXCHANGE
    let explanation_message = Message::Explanation { play: play, nonce: nonce };
    let message_json_str = explanation_message.to_json_str()?;
    stream.write_all(message_json_str.as_bytes())?;
    
    let opponent_explanation_message = rx.recv().unwrap();
    let opponent_play;
    let opponent_nonce;
    if let Message::Explanation { play, nonce } = opponent_explanation_message {
        opponent_play = play;
        opponent_nonce = nonce;
    } else {
        panic!("Invalid message from opponent!");
    }

    // PHASE 3 - VALIDATION
    let opponent_validation_hash = hash_play(&opponent_play, &opponent_nonce);
    if opponent_validation_hash != opponent_hashed_play {
        println!("Opponent cheated!");
        return Ok(());
    }

    println!("");
    println!("RESULTS:");
    println!("Your play: {}", play.get_value());
    println!("Opponent play: {}", opponent_play.get_value());
    println!("Result: {}", get_result(&play, &opponent_play));

    read_thread_handle.join().unwrap();
    Ok(())
}

fn commit_to_play(play: &Play) -> (String, String) {
    let nonce = create_nonce();
    let hashed_play = hash_play(play, &nonce);
    return (nonce, hashed_play);
}

fn get_result(play: &Play, opponent_play: &Play) -> String {
    let str_winner = match play {
        Play::Rock => {
            match opponent_play {
                Play::Rock => "Draw",
                Play::Paper => "Loss",
                Play::Scissors => "Win",
            }
        },
        Play::Paper => {
            match opponent_play {
                Play::Rock => "Win",
                Play::Paper => "Draw",
                Play::Scissors => "Loss",
            }
        },
        Play::Scissors => {
            match opponent_play {
                Play::Rock => "Loss",
                Play::Paper => "Win",
                Play::Scissors => "Draw",
            }
        },
    };
    String::from(str_winner)
}