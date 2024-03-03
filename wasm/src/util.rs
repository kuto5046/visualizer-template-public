#![allow(non_snake_case, unused_macros)]

use itertools::Itertools;
use proconio::{input, marker::Chars};
use rand::prelude::*;
use std::ops::RangeBounds;
// use svg::node::element::{Rectangle, Text};
use svg::node::element::Text as SvgText;
use svg::node::Text as TextContent;
use svg::Document;
use svg::node::element::Path as SvgPath;
use svg::node::element::path::Data as SvgData;
// circle
use svg::node::element::Circle;

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
    pub ty: u64,
    pub n: usize,
    pub a: Vec<Vec<i32>>,
    pub vs: Vec<Vec<char>>,
    pub hs: Vec<Vec<char>>,
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {}", self.ty, self.n)?;
        for i in 0..self.n {
            writeln!(f, "{}", self.vs[i].iter().collect::<String>())?;
        }
        for i in 0..self.n - 1 {
            writeln!(f, "{}", self.hs[i].iter().collect::<String>())?;
        }
        for i in 0..self.n {
            writeln!(f, "{}", self.a[i].iter().join(" "))?;
        }
        Ok(())
    }
}

pub fn parse_input(f: &str) -> Input {
    let f = proconio::source::once::OnceSource::from(f);
    input! {
        from f,
        ty: u64, n: usize,
        vs: [Chars; n],
        hs: [Chars; n - 1],
        a: [[i32; n]; n],
    }
    Input { ty, n, a, vs, hs }
}

pub fn parse_input_fixed(f: &str) -> Input {
    let f = proconio::source::once::OnceSource::from(f);
    input! {
        from f,
        ty: u64, n: usize,
        vs: [Chars; n],
        hs: [Chars; n - 1],
    }
    for i in 0..n {
        assert_eq!(vs[i].len(), n - 1);
    }
    for i in 0..n - 1 {
        assert_eq!(hs[i].len(), n);
    }
    Input {
        ty,
        n,
        a: vec![],
        vs,
        hs,
    }
}

pub fn read<T: Copy + PartialOrd + std::fmt::Display + std::str::FromStr, R: RangeBounds<T>>(
    token: Option<&str>,
    range: R,
) -> Result<T, String> {
    if let Some(v) = token {
        if let Ok(v) = v.parse::<T>() {
            if !range.contains(&v) {
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

const DIRS: [char; 4] = ['U', 'D', 'L', 'R'];
const DIJ: [(usize, usize); 4] = [(!0, 0), (1, 0), (0, !0), (0, 1)];

pub struct Output {
    pub start: (usize, usize, usize, usize),
    pub out: Vec<(bool, usize, usize)>,
}

pub fn parse_output(input: &Input, f: &str) -> Result<Output, String> {
    let mut out = vec![];
    let mut ss = f.split_whitespace();
    let start = (
        read(ss.next(), 0..input.n)?,
        read(ss.next(), 0..input.n)?,
        read(ss.next(), 0..input.n)?,
        read(ss.next(), 0..input.n)?,
    );
    while let Some(mv) = ss.next() {
        let do_swap = if mv == "1" {
            true
        } else if mv != "0" {
            return Err(format!("Invalid action: {}", mv));
        } else {
            false
        };
        let dir1 = read(ss.next(), '.'..='Z')?;
        let dir2 = read(ss.next(), '.'..='Z')?;
        let dir1 = if dir1 == '.' {
            !0
        } else if let Some(dir1) = DIRS.iter().position(|&d| d == dir1) {
            dir1
        } else {
            return Err(format!("Invalid direction: {}", dir1));
        };
        let dir2 = if dir2 == '.' {
            !0
        } else if let Some(dir2) = DIRS.iter().position(|&d| d == dir2) {
            dir2
        } else {
            return Err(format!("Invalid direction: {}", dir2));
        };
        out.push((do_swap, dir1, dir2));
    }
    if out.len() > 4 * input.n * input.n {
        return Err("Too many actions".to_owned());
    }
    Ok(Output { start, out })
}

const FIXED: [&'static str; 20] = [
    include_str!("../in_fixed/0.txt"),
    include_str!("../in_fixed/1.txt"),
    include_str!("../in_fixed/2.txt"),
    include_str!("../in_fixed/3.txt"),
    include_str!("../in_fixed/4.txt"),
    include_str!("../in_fixed/5.txt"),
    include_str!("../in_fixed/6.txt"),
    include_str!("../in_fixed/7.txt"),
    include_str!("../in_fixed/8.txt"),
    include_str!("../in_fixed/9.txt"),
    include_str!("../in_fixed/10.txt"),
    include_str!("../in_fixed/11.txt"),
    include_str!("../in_fixed/12.txt"),
    include_str!("../in_fixed/13.txt"),
    include_str!("../in_fixed/14.txt"),
    include_str!("../in_fixed/15.txt"),
    include_str!("../in_fixed/16.txt"),
    include_str!("../in_fixed/17.txt"),
    include_str!("../in_fixed/18.txt"),
    include_str!("../in_fixed/19.txt"),
];

pub fn gen(seed: u64) -> Input {
    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(seed);
    let ty = seed % 20;
    let mut input = parse_input_fixed(FIXED[ty as usize]);
    let mut nums = (1..=input.n * input.n).collect_vec();
    nums.shuffle(&mut rng);
    input.a = mat![0; input.n; input.n];
    for i in 0..input.n {
        for j in 0..input.n {
            input.a[i][j] = nums[i * input.n + j] as i32;
        }
    }
    input
}

fn can_move(N: usize, h: &Vec<Vec<char>>, v: &Vec<Vec<char>>, i: usize, j: usize, dir: usize) -> bool {
    let (di, dj) = DIJ[dir];
    let i2 = i + di;
    let j2 = j + dj;
    if i2 >= N || j2 >= N {
        return false;
    }
    if di == 0 {
        v[i][j.min(j2)] == '0'
    } else {
        h[i.min(i2)][j] == '0'
    }
}

pub fn compute_score(input: &Input, out: &Output) -> (i64, String) {
    let (mut score, err, _) = compute_score_details(input, out.start, &out.out);
    if err.len() > 0 {
        score = 0;
    }
    (score, err)
}

fn compute_diff(input: &Input, a: &Vec<Vec<i32>>) -> i64 {
    let mut diff = 0;
    for i in 0..input.n {
        for j in 0..input.n {
            for dir in 1..=2 {
                if can_move(input.n, &input.hs, &input.vs, i, j, dir) {
                    let d = (a[i][j] - a[i + DIJ[dir].0][j + DIJ[dir].1]) as i64;
                    diff += d * d;
                }
            }
        }
    }
    diff
}

pub fn compute_score_details(
    input: &Input,
    start: (usize, usize, usize, usize),
    out: &[(bool, usize, usize)],
) -> (i64, String, (Vec<Vec<i32>>, (usize, usize), (usize, usize))) {
    let mut a = input.a.clone();
    let mut p1 = (start.0, start.1);
    let mut p2 = (start.2, start.3);
    let before = compute_diff(&input, &a);
    for &(do_swap, dir1, dir2) in out {
        if do_swap {
            let tmp = a[p1.0][p1.1];
            a[p1.0][p1.1] = a[p2.0][p2.1];
            a[p2.0][p2.1] = tmp;
        }
        if dir1 != !0 {
            if !can_move(input.n, &input.hs, &input.vs, p1.0, p1.1, dir1) {
                return (0, format!("Invalid move: {}", DIRS[dir1]), (a, p1, p2));
            }
            p1.0 += DIJ[dir1].0;
            p1.1 += DIJ[dir1].1;
        }
        if dir2 != !0 {
            if !can_move(input.n, &input.hs, &input.vs, p2.0, p2.1, dir2) {
                return (0, format!("Invalid move: {}", DIRS[dir2]), (a, p1, p2));
            }
            p2.0 += DIJ[dir2].0;
            p2.1 += DIJ[dir2].1;
        }
    }
    let after = compute_diff(&input, &a);
    let score = ((1e6 * (f64::log2(before as f64) - f64::log2(after as f64))).round() as i64).max(1);
    (score, String::new(), (a, p1, p2))
}


const MARGIN: f32 = 10.0;
const BOX_SIZE: f32 = 40.0;
const TAKA_COLOR: &str = "Red";
const AOKI_COLOR: &str = "Blue";
const CIRCLE_SIZE: f32 = 10.0;

pub fn vis(input: &Input, output: &Output, turn: usize) -> (i64, String, String) {
    let N = BOX_SIZE * input.n as f32;
    let out = &output.out[..turn];
    let (score, err, (arrange, p_taka, p_aoki)) = compute_score_details(input, output.start, &out);

    // arranegの隣接する数字の誤差平均を計算
    let mut arrange_score_map = mat![0; input.n; input.n];
    for i in 0..input.n {
        for j in 0..input.n {
            // 隣接する4マスを見る
            for dir in 0..4 {
                let (di, dj) = DIJ[dir];
                let i2 = i as i32 + di as i32;
                let j2 = j as i32 + dj as i32;
                // 範囲外の場合はスキップ
                if i2 < 0 || i2 >= input.n as i32 || j2 < 0 || j2 >= input.n as i32 {
                    continue;
                }
                let loss = (arrange[i][j] - arrange[i2 as usize][j2 as usize]).pow(2);
                arrange_score_map[i][j] += loss;
            }
        }
    }
    // arrange_scoreを0~1に正規化
    let max_score = arrange_score_map.iter().map(|v| v.iter().max().unwrap()).max().unwrap();
    let min_score = arrange_score_map.iter().map(|v| v.iter().min().unwrap()).min().unwrap();
    let mut arrange_score = mat![0.0; input.n; input.n];
    for i in 0..input.n {
        for j in 0..input.n {
            arrange_score[i][j] = (arrange_score_map[i][j] - min_score) as f64 / (max_score - min_score) as f64;
        }
    }
    let mut doc = Document::new()
    .set("ViewBox", (0, 0, (MARGIN * 2.0 + BOX_SIZE * N) as i64, (MARGIN * 2.0 + BOX_SIZE * N) as i64))
    .set("id", "util")
    .set("width", (MARGIN * 2.0 + BOX_SIZE * N) as i64)
    .set("height", (MARGIN * 2.0 + BOX_SIZE * N) as i64);

    for i in 0..input.n {
        for j in 0..input.n {
            let score = arrange_score[i][j];
            let color = "Green";
            let data = 
                SvgData::new()
                // 正方形(相対移動とそこを基準とした線の描画)
                .move_to((MARGIN + BOX_SIZE * j as f32, MARGIN + BOX_SIZE * i as f32))
                .line_by((BOX_SIZE, 0))
                .line_by((0, BOX_SIZE))
                .line_by((-BOX_SIZE, 0))
                .line_by((0, -BOX_SIZE));
            let path = SvgPath::new()
                .set("d", data)
                .set("fill", color)
                .set("fill-opacity", score)
                .set("stroke", "black")
                .set("stroke-opacity", 0.7)
                .set("stroke-width", 1);
            doc = doc.add(path);

            // 数字を表示
            let number = arrange[i][j];
            let text = SvgText::new()
                .set("x", MARGIN + BOX_SIZE * (j as f32 + 0.5))
                .set("y", MARGIN + BOX_SIZE * (i as f32 + 0.5))
                .set("font-size", "medium")
                .set("fill", "black")
                .set("fill-opacity", 0.7)
                .set("text-anchor", "middle")
                .set("dominant-baseline", "central")
                .add(TextContent::new(format!("{}", number)));
            doc = doc.add(text);

            // 高橋くんと青木くんの位置を描く
            if (i, j) == (p_taka.0, p_taka.1) {
                let circle = Circle::new()
                    .set("cx", MARGIN + BOX_SIZE * (j as f32 + 0.5))
                    .set("cy", MARGIN + BOX_SIZE * (i as f32 + 0.5))
                    .set("r", CIRCLE_SIZE)
                    .set("fill", TAKA_COLOR)
                    .set("fill-opacity", 0.6)
                    .set("stroke", "black");
                doc = doc.add(circle);
            }
            if (i, j) == (p_aoki.0, p_aoki.1) {
                let circle = Circle::new()
                    .set("cx", MARGIN + BOX_SIZE * (j as f32 + 0.5))
                    .set("cy", MARGIN + BOX_SIZE * (i as f32 + 0.5))
                    .set("r", CIRCLE_SIZE)
                    .set("fill", AOKI_COLOR)
                    .set("fill-opacity", 0.6)
                    .set("stroke", "black");
                doc = doc.add(circle);
            }
            }
        }

    // 壁を描く    
    // 一番下の線を引くためにH+1まで描く
    for i in 0..input.n + 1 {
        for j in 0..input.n {
            // 上下の壁をhで描く
            // i=0とi=Hの時は無条件で描く(大枠) hはH-1まで
            if !(1 <= i && i < input.n) || input.hs[i - 1][j] == '1' {
                let (color, width) = ("black", 3);
                // 
                let data = 
                    SvgData::new()
                    // 
                    .move_to((MARGIN + BOX_SIZE * j as f32, MARGIN + BOX_SIZE * i as f32))
                    .line_by((BOX_SIZE * 1.0, 0));
                let p = SvgPath::new()
                    .set("d", data)
                    .set("stroke", color)
                    .set("stroke-opacity", 1)
                    .set("stroke-width", width);
                doc = doc.add(p);
            }
        }
    }

    for j in 0..input.n + 1 {
        for i in 0..input.n {
            // 左右の壁をvで描く
            // j=0とj=Wの時は無条件で描く(大枠)
            // 初期位置の場合は描かない
            if !(1 <= j && j < input.n) || input.vs[i][j - 1] == '1' {
                let (color, width) = ("black", 3);
                let data = 
                    SvgData::new()
                    .move_to((MARGIN + BOX_SIZE * j as f32, MARGIN + BOX_SIZE * i as f32))
                    .line_by((0, BOX_SIZE * 1.0));
                let p = SvgPath::new()
                    .set("d", data)
                    .set("stroke", color)
                    .set("stroke-opacity", 1)
                    .set("stroke-width", width);
                doc = doc.add(p);
            }
        }
    }

    (score, err.to_string(), doc.to_string())
}
