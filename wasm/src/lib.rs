use wasm_bindgen::prelude::*;
mod util;

#[wasm_bindgen]
pub fn gen(seed: i32) -> String {
    let input = util::gen(seed as u64, None, None);
    match input {
        Ok(input) => input.to_string(),
        Err(err) => err.to_string(),
    }
}

#[wasm_bindgen(getter_with_clone)]
pub struct Ret {
    pub score: i64,
    pub err: String,
    pub svg: String,
}

#[wasm_bindgen]
pub fn vis(_input: String, _output: String, turn: usize) -> Ret {
    let mut input = util::parse_input(&_input).unwrap();
    let mut vis_data_vec = vec![];
    let k = 10;
    let judge_result = util::judge(&_input, &_output, &mut vis_data_vec, &mut input, k);
    match judge_result {
        Ok(judge_result) => {
            let vis_data = vis_data_vec[turn].clone();
            let svg = util::draw_svg(&vis_data, &input, k).to_string();
            return Ret {
                score: judge_result.score as i64,
                err: "".to_string(),
                svg,
            };
        }
        Err(err) => {
            return Ret {
                score: 0,
                err: err.to_string(),
                svg: "".to_string(),
            };
        }
    }
}

#[wasm_bindgen]
pub fn get_max_turn(_input: String, _output: String) -> usize {
    let input = util::parse_input(&_input).unwrap();
    let output = util::parse_output(&_output, &input).unwrap();
    output.commented_ops.len()
}
