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
    io::{self, BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
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

fn game_start(mut stream: TcpStream) -> Result<()> {
    let read_stream = stream.try_clone()?;

    thread::spawn(move || {
        let buf_reader = BufReader::new(&read_stream);
        for line in buf_reader.lines() {
            match line {
                Ok(msg) => {
                    println!("\nThem: {}", msg);
                    let message = Message::from_json_str(&msg).unwrap();
                    println!("{message:?}");
                    io::stdout().flush().unwrap();
                }
                Err(_) => {
                    println!("\nConnection closed");
                    break;
                }
            }
        }
    });

    loop {
        let play = get_play_input();
        let (nonce, hashed_play) = commit_to_play(&play);
        let message = Message::HashedPlay(hashed_play);
        let message_json_str = format!("{}\n", message.to_json_str()?);

        if stream.write_all(message_json_str.as_bytes()).is_err() {
            println!("Failed to send message");
            break;
        }
    }

    Ok(())
}

fn commit_to_play(play: &Play) -> (String, String) {
    let nonce = create_nonce();
    let hashed_play = hash_play(play, &nonce);
    return (nonce, hashed_play);
}
