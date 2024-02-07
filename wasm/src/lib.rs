use wasm_bindgen::prelude::*;
mod util;

#[wasm_bindgen]
pub fn gen(seed: i32) -> String {
    // Input構造体をto_stringする fmt::Displayトレイトを実装している必要がある
    util::gen(seed as u64, 0, 0).to_string()
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
    let output = util::parse_output(&input, &_output);
	let (score, err, svg) = match output {
		Ok(output) => util::vis(&input, &output, turn),
		Err(err) => (0, err, String::new()),
	};
    Ret {
        score,
        err: err.to_string(),
        svg: svg.to_string(),
    }
}

#[wasm_bindgen]
pub fn get_max_turn(_input: String, _output: String) -> usize {
    let input = util::parse_input(&_input);
    let output = util::parse_output(&input, &_output);
    match output {
        Ok(output) => output.len() as usize,
        Err(_) => 0 as usize,
    }
}
