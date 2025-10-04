use rand::Rng;
use sha2::{Digest, Sha256};

use crate::model::Play;

pub fn create_nonce() -> String {
    let mut rng = rand::rng();
    let mut nonce = [0u8; 32]; // 256 bits
    rng.fill(&mut nonce);
    hex::encode(nonce)
}

pub fn hash_play(play: &Play, nonce: &str) -> String {
    let play_value = play.get_value();

    let mut hasher = Sha256::new();
    hasher.update(play_value.as_bytes());
    hasher.update(nonce.as_bytes());
    let result = hasher.finalize();

    format!("{:x}", result)
}
