use rand::prelude::*;

pub fn gen(seed: u64) -> String {
    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(seed);
    "".to_string()
}

pub struct Input {
}

pub struct Output {
}

pub fn vis(input: &Input, output: &Output, turn: usize) -> (i64, String, String) {
    (0, "".to_string(), "".to_string())
}