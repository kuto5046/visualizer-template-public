use wasm_bindgen::prelude::*;
mod util;

#[wasm_bindgen]
pub fn gen(seed: i32) -> String {
    util::gen(seed as u64).to_string()
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
    // _outputが空文字の場合"0"にする
    let _output = if _output.is_empty() { "0" } else { &_output };
    let output = util::parse_output(&input, &_output);
    let (score, err, svg) = match output {
        Ok(output) => util::vis(&input, &output, turn),
        Err(err) => (0, err, "".to_string()),
    };
    Ret {
        score,
        err,
        svg,
    }
}

#[wasm_bindgen]
pub fn get_max_turn(_input: String, _output: String) -> usize {
    let input = util::parse_input(&_input);
    input.T
}
