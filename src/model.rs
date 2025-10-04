use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Play {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
pub enum Mode {
    Server,
    Client,
}

const ROCK_PLAY_VALUE: &str = "r";
const PAPER_PLAY_VALUE: &str = "p";
const SCISSORS_PLAY_VALUE: &str = "s";

impl Play {
    pub fn get_value(&self) -> &str {
        match self {
            Self::Rock => ROCK_PLAY_VALUE,
            Self::Paper => PAPER_PLAY_VALUE,
            Self::Scissors => SCISSORS_PLAY_VALUE,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Message {
    Play { play: Play },
    Ack(String),
}

pub trait ToJson: Serialize {
    fn to_json_str(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

pub trait FromJson: for<'a> Deserialize<'a> {
    fn from_json_str(json: &str) -> Result<Self, serde_json::Error>
    where
        Self: Sized,
    {
        serde_json::from_str(json)
    }
}

impl ToJson for Message {}
impl FromJson for Message {}
impl ToJson for Play {}
impl FromJson for Play {}
