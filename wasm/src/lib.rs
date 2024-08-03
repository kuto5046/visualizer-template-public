use wasm_bindgen::prelude::*;
mod util;

const PROBLEM_ID: char = 'C';

#[wasm_bindgen]
pub fn gen(seed: i32) -> String {
    util::gen(seed as u64, PROBLEM_ID).to_string()
}

#[wasm_bindgen(getter_with_clone)]
pub struct Ret {
    pub score: i64,
    pub err: String,
    pub svg: String,
}

#[wasm_bindgen]
pub fn vis(_input: String, _output: String, turn: usize) -> Ret {
    let input = util::parse_input(&_input);
    let output = util::parse_output(&input, &_output).unwrap();
    let (score, err, svg) = util::vis(&input, &output, turn);
    Ret {
        score,
        err: err.to_string(),
        svg: svg.to_string(),
    }
}

#[wasm_bindgen]
pub fn get_max_turn(_input: String, _output: String) -> usize {
    let input = util::parse_input(&_input);
    let output = util::parse_output(&input, &_output).unwrap();
    output.out.len() - 1
}