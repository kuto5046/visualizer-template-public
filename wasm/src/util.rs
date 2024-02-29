#![allow(non_snake_case, unused_macros)]

use itertools::Itertools;
use proconio::input;
use rand::prelude::*;
use svg::Document;
use svg::node::Text as TextContent;
use svg::node::{
    element::{Group, Rectangle, Style, Title, Text},
};

pub trait SetMinMax {
    fn setmin(&mut self, v: Self) -> bool;
    fn setmax(&mut self, v: Self) -> bool;
}
impl<T> SetMinMax for T
where
    T: PartialOrd,
{
    fn setmin(&mut self, v: T) -> bool {
        *self > v && {
            *self = v;
            true
        }
    }
    fn setmax(&mut self, v: T) -> bool {
        *self < v && {
            *self = v;
            true
        }
    }
}

#[macro_export]
macro_rules! mat {
	($($e:expr),*) => { Vec::from(vec![$($e),*]) };
	($($e:expr,)*) => { Vec::from(vec![$($e),*]) };
	($e:expr; $d:expr) => { Vec::from(vec![$e; $d]) };
	($e:expr; $d:expr $(; $ds:expr)+) => { Vec::from(vec![mat![$e $(; $ds)*]; $d]) };
}

#[derive(Clone, Debug)]
pub struct Input {
    pub n: usize,
    pub m: usize,
    pub bs: Vec<Vec<usize>>,
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {}", self.n, self.m)?;
        for i in 0..self.m {
            writeln!(f, "{}", self.bs[i].iter().join(" "))?;
        }
        Ok(())
    }
}

pub fn parse_input(f: &str) -> Input {
    let f = proconio::source::once::OnceSource::from(f);
    input! {
        from f,
        n: usize, m: usize,
        bs: [[usize; n / m]; m]
    }
    Input { n, m, bs }
}

pub fn read<T: Copy + PartialOrd + std::fmt::Display + std::str::FromStr>(
    token: Option<&str>,
    lb: T,
    ub: T,
) -> Result<T, String> {
    if let Some(v) = token {
        if let Ok(v) = v.parse::<T>() {
            if v < lb || ub < v {
                Err(format!("Out of range: {}", v))
            } else {
                Ok(v)
            }
        } else {
            Err(format!("Parse error: {}", v))
        }
    } else {
        Err("Unexpected EOF".to_owned())
    }
}

pub struct Output {
    pub out: Vec<(usize, usize)>,
}

pub fn parse_output(input: &Input, f: &str) -> Result<Output, String> {
    let mut out = vec![];
    let mut tokens = f.split_whitespace().peekable();
    while tokens.peek().is_some() {
        out.push((read(tokens.next(), 1, input.n)?, read(tokens.next(), 0, input.m)?));
    }
    if out.len() > 5000 {
        return Err("Too many output".to_owned());
    }
    Ok(Output { out })
}

pub fn gen(seed: u64) -> Input {
    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(seed);
    let n = 200;
    let m = 10;
    let mut is = (1..=n).collect_vec();
    is.shuffle(&mut rng);
    let mut bs = vec![vec![]; m];
    for i in 0..n {
        bs[i % m].push(is[i]);
    }
    Input { n, m, bs }
}

pub fn compute_score(input: &Input, out: &Output) -> (i64, String) {
    let (mut score, err, _) = compute_score_details(input, &out.out);
    if err.len() > 0 {
        score = 0;
    }
    (score, err)
}

pub fn compute_score_details(input: &Input, out: &[(usize, usize)]) -> (i64, String, Vec<Vec<usize>>) {
    let mut bs = input.bs.clone();
    let mut cost = 0;
    let mut t = 0;
    for &(v, mut to) in out {
        let (i, j) = 'ij: {
            for i in 0..bs.len() {
                for j in 0..bs[i].len() {
                    if bs[i][j] == v {
                        break 'ij (i, j);
                    }
                }
            }
            return (0, format!("Box {v} has already been taken out."), bs);
        };
        if to == 0 {
            if j + 1 != bs[i].len() {
                return (0, format!("Box {v} is not at the top of the stack."), bs);
            } else if v != t + 1 {
                return (
                    0,
                    format!("Before carrying out box {v}, all boxes less than {v} must be carried out."),
                    bs,
                );
            }
            bs[i].pop();
            t += 1;
        } else {
            cost += bs[i].len() - j + 1;
            to -= 1;
            if i != to {
                for k in j..bs[i].len() {
                    let b = bs[i][k];
                    bs[to].push(b);
                }
                bs[i].truncate(j);
            }
        }
    }
    let score = (10000 - cost as i64).max(1);
    let err = if t < input.n {
        format!("Not finished ({} / {})", t, input.n)
    } else {
        String::new()
    };
    (score, err, bs)
}

pub fn txt(x: usize, y: usize, height: usize, text: &str) -> Text {
    Text::new()
        .add(TextContent::new(text))
        .set("x", x + BOX_WIDTH / 2)
        .set("y", y + height / 2)
        .set("fill", "black")
        .set("font-size", 15)
        .set("text-anchor", "middle")
        .set("dominant-baseline", "central")
}

pub fn rect(x: usize, y: usize, height: usize, color: &str, opacity: f64) -> Rectangle {
    Rectangle::new()
        .set("x", x)
        .set("y", y)
        .set("width", BOX_WIDTH)
        .set("height", height)
        .set("fill", "Green")
        .set("fill-opacity", opacity)
        .set("stroke", "black")
}


pub fn cardboard_box(x: usize, y: usize, height: usize, color: &str, opacity: f64, text: &str) -> Group {
    Group::new().add(rect(x, y, height, color, opacity)).add(txt(x, y, height, text))
}

const SVG_SIZE: usize = 800;
const MARGIN: isize = 10;
const BOX_WIDTH : usize = 80;

pub fn color(mut val: f64) -> String {
    assert!(0.0 <= val && val <= 1.0);
    val = val.min(1.0);
    val = val.max(0.0);
    let (r, g, b) = if val < 0.5 {
        let x = val * 2.0;
        (
            30. * (1.0 - x) + 144. * x,
            144. * (1.0 - x) + 255. * x,
            255. * (1.0 - x) + 30. * x,
        )
    } else {
        let x = val * 2.0 - 1.0;
        (
            144. * (1.0 - x) + 255. * x,
            255. * (1.0 - x) + 30. * x,
            30. * (1.0 - x) + 70. * x,
        )
    };
    format!(
        "#{:02x}{:02x}{:02x}",
        r.round() as i32,
        g.round() as i32,
        b.round() as i32
    )
}

pub fn color_with_alpha(mut val: f64, alpha: f64) -> String {
    assert!(0.0 <= alpha && alpha <= 1.0);
    let alpha: u8 = (alpha * 255.0) as u8;
    let c = color(val);
    // 16進数の形式にフォーマット（RGBA形式）
    format!("{}{:02X}", c, alpha)
}


pub fn vis(input: &Input, output: &Output, turn: usize) -> (i64, String, String) {
    let out = &output.out[..turn];
    let mut doc = Document::new()
        .set("viewBox", (-MARGIN, -MARGIN, SVG_SIZE + 2 * MARGIN as usize, SVG_SIZE + 2 * MARGIN as usize))
        .set("width", SVG_SIZE + MARGIN as usize)
        .set("height", SVG_SIZE + MARGIN as usize)
        .set("style", "background-color:#F2F3F5"); // 通常不要

    let (score, err, bs) = compute_score_details(input, out);
    let max_h = (2 * input.n / input.m).max(bs.iter().map(|b| b.len()).max().unwrap());
    let box_height: usize = SVG_SIZE / max_h as usize;
    // mこの山に長方形を割り振る。長方形には番号を振る
    for i in 0..input.m {
        let x: usize = (i as f64 * SVG_SIZE as f64 / input.m as f64) as usize;
        for j in 0..bs[i].len() {
            let y = (SVG_SIZE - MARGIN as usize) - (j * box_height) as usize;
            let box_idx = bs[i][j];
            let color = color(box_idx as f64 / input.n as f64);
            let opacity = box_idx as f64 / input.n as f64;
            let gr = cardboard_box(x as usize, y as usize, box_height, &color, opacity, &format!("{}", box_idx));
            doc = doc.add(gr);
        }
    }
    // let (score, err) = compute_score(input, output);
    (score, err, doc.to_string())
}
