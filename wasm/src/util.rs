#![allow(non_snake_case, unused_imports)]
use svg::node::{element::{Line, Circle, Group, Title, Rectangle}, Text};
use proconio::input;
use rand::prelude::*;


pub const DXY: [(i32, i32); 8] = [
	(1, 0),
	(1, 1),
	(0, 1),
	(-1, 1),
	(-1, 0),
	(-1, -1),
	(0, -1),
	(1, -1)
];

pub type P = (i32, i32);
pub type Output = Vec<[P; 4]>;


#[macro_export]
macro_rules! mat {
	($($e:expr),*) => { Vec::from(vec![$($e),*]) };
	($($e:expr,)*) => { Vec::from(vec![$($e),*]) };
	($e:expr; $d:expr) => { Vec::from(vec![$e; $d]) };
	($e:expr; $d:expr $(; $ds:expr)+) => { Vec::from(vec![mat![$e $(; $ds)*]; $d]) };
}


#[derive(Clone, Debug)]
pub struct Input {
    pub N: usize,  // 盤面の大きさ
    pub ps: Vec<P>, // 初期状態での点の位置
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Input構造体をto_stringする fmt::Displayトレイトを実装している必要がある
        writeln!(f, "{} {}", self.N, self.ps.len())?;
        for p in &self.ps {
            writeln!(f, "{} {}", p.0, p.1)?;
        }
        Ok(())
    }
}

pub fn parse_input(f: &str) -> Input {
    // 指定したpathのファイルを読み込む
	let f = proconio::source::once::OnceSource::from(f);
	input! {
		from f,
		N: usize, M: usize,
		ps: [(i32, i32); M],
	}
	Input { N, ps }
}


fn read<T: Copy + PartialOrd + std::fmt::Display + std::str::FromStr>(token: Option<&str>, lb: T, ub: T) -> Result<T, String> {
    // ジェネリック型Tを持つ汎用的な入力読み取り関数
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


pub fn parse_output(input: &Input, f: &str) -> Result<Output, String> {
    // 指定したpathのファイルを読み込む
	let mut out = vec![];
	let mut tokens = f.split_whitespace();
	let M = read(tokens.next(), 0, 1000000000)?;
	for _ in 0..M {
		let mut rect = [(0, 0); 4];
		for i in 0..4 {
			rect[i] = (read(tokens.next(), 0, input.N as i32 - 1)?, read(tokens.next(), 0, input.N as i32 - 1)?);
		}
		out.push(rect);
	}
	Ok(out)
}

pub fn gen(seed: u64, n: usize, m: usize) -> Input {
    // 問題文にある生成方法に従ってN, Mを生成する
    // n, mが0以上の場合はそれを使う

    // seed固定してランダムな値を生成する
	let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(seed ^ 3808);
    // rand(15,30) * 2 + 1
	let mut N = rng.gen_range(15i32, 31i32) as usize * 2 + 1;
    if n > 0 {
        N = n;
    }
    // rand(N, N**2/12)
    let mut M = rng.gen_range(N as i32, (N*N/12 + 1).max(N+1) as i32) as usize;
    if m > 0 {
        M = m;
    }

    // 初期状態での点の位置を生成する
    let mut ps = vec![];
    for x in N/4..=3*N/4 {
        for y in N/4..=3*N/4 {
            ps.push((x as i32, y as i32));
        }
    }
    ps.shuffle(&mut rng);
    ps.truncate(M);
    Input { N, ps }
}

fn rect(x: i32, y: i32, w: i32, h: i32, fill: &str) -> Rectangle {
    Rectangle::new()
        .set("x", x)
        .set("y", y)
        .set("width", w)
        .set("height", h)
        .set("fill", fill)
}

#[derive(Clone, Debug)]
pub struct State {
    pub has_point: Vec<Vec<bool>>,  // 頂点が存在するか(N*Nの2次元配列)
    pub used: Vec<Vec<[bool; 8]>>,  // 辺が使われているか(隣接する8つの頂点との辺)
}

impl State {
    pub fn new(input: &Input) -> Self {
        let mut has_point = mat![false; input.N; input.N];
        let used = mat![[false; 8]; input.N; input.N];
        for i in 0..input.ps.len() {
            has_point[input.ps[i].0 as usize][input.ps[i].1 as usize] = true;
        }
        Self {
            has_point,
            used,
        }
    }

    // pub fn check_move(&self, rect: [P; 4]) -> String {
    //     // 与えられた四角形が配置可能かを判定する
    //     // 四角形の4頂点のうちどれかが頂点を持たない場合はerrorを返す
    //     if let Some(i) = (1..4).find(|&i| !self.has_point[rect[i].0 as usize][rect[i].1 as usize]) {
	// 		return format!("({}, {}) does not contain a dot", rect[i].0, rect[i].1);
    //     //
    //     } else if self.has_point[rect[0].0 as usize][rect[0].1 as usize] {
    //         return format!("({}, {}) already contains as dot", rect[0].0, rect[0].1);
    //     } else {
    //         let dx01 = rect[1].0 - rect[0].0;
    //         let dy01 = rect[1].1 - rect[0].1;
    //         let dx03 = rect[3].0 - rect[0].0;
    //         let dy03 = rect[3].1 - rect[0].1;

    //         if dx01 * dx03 + dy01 * dy03 != 0 {
    //             return "Illegal rectangle".to_owned();
    //         }
    //     }
    // }

    // pub fn apply_move(&mut self, rect: [P; 4]) {
    //     // 与えられた四角形を配置する
    //     self.has_point[rect[0].0 as usize][rect[0].1 as usize] = true;
    //     for i in 0..4 {
    //         let (mut x, mut y) = rect[i];
    //         //
    //         let (tx, ty) = rect[(i + 1) % 4];
    //         let dx = (tx - x).signum();
    //         let dy = (ty - y).signum();
    //         // dx, dyからdir(0~8の数字)を得る
    //         let dir = (0..8).find(|&dir| DXY[dir]==(dx,dy)).unwrap();
    //         while (x, y) != (tx, ty) {
    //             self.used[x as usize][y as usize][dir] = true;
    //             x += dx;
    //             y += dy;
    //             // 反対側の辺もusedを記録する
    //             self.used[x as usize][y as usize][dir ^ 4] = true;
    //         }
    //     }
    // }
}

pub fn weight((x, y): P, N: usize) -> i32 {
	let dx = x - N as i32 / 2;
	let dy = y - N as i32 / 2;
	dx * dx + dy * dy + 1
}


pub fn vis(input: &Input, out: &[[P; 4]], turn: usize) -> (i64, String, String) {
    // turnまでの状態を描画する
    let out = &out[..turn];
    let D = 800 / input.N;
    let H = input.N * D;
    let W = input.N * D;
    // 描画エリアを作成
    let mut doc = svg::Document::new().set("id", "vis").set("viewBox", (0, 0, W, H)).set("width", W).set("height", H);
    // 背景を白に設定
    doc = doc.add(rect(0, 0, W as i32, H as i32, "white"));

    // 初期状態での点の位置を!1としてidの2次元配列に記録する
    // id[x][y]: x,Yの位置を選択したturn
    let mut id = mat![!0; input.N; input.N];
    for p in &input.ps {
        id[p.0 as usize][p.1 as usize] = !1;
    }
    // 回答の四角形をidの2次元配列に記録する iはturnを表す
    for i in 0..out.len() {
        id[out[i][0].0 as usize][out[i][0].1 as usize] = i;
    }

    // N*NのgridをCircleで描画する
    for x in 0..input.N {
        for y in 0..input.N {
            if id[x][y] == !0 {
                doc = doc.add(Circle::new().set("cx", D/2+x * D).set("cy", H - D / 2 - y * D).set("r", 2).set("fill", "gray"));
            }
        }
    }
    // 回答の四角形の辺を順に描画する
    for rect in out {
        // 四角形の4辺をLineで描画する
        for i in 0..4 {
            let (x1, y1) = rect[i];
            let (x2, y2) = rect[(i + 1) % 4];
			doc = doc.add(Line::new().set("x1", D / 2 + x1 as usize * D).set("y1", H - D / 2 - y1 as usize * D).set("x2", D / 2 + x2 as usize * D).set("y2", H - D / 2 - y2 as usize * D).set("stroke", "black").set("stroke-width", 2));
        }
    }

    for x in 0..input.N {
        for y in 0..input.N {
            // 何も記録されていない頂点であればskip
            let c = if id[x][y] == !0 {
                continue;
            // 初期状態での点であれば黒い点を描画する
            } else if id[x][y] == !1 {
                Circle::new().set("cx", D / 2 + x * D).set("cy", H-D/2 - y * D).set("r", (D/4).max(5)).set("fill", "black")
            // 最後のターンでなければ黒い点を描画する
            } else if id[x][y] != out.len() - 1 {
                Circle::new().set("cx", D / 2 + x * D).set("cy", H-D/2 - y * D).set("r", (D/4).max(5)).set("fill", "black")
            // 最後のターンであれば赤い点を描画する
            } else {
                Circle::new().set("cx", D / 2 + x * D).set("cy", H-D/2 - y * D).set("r", (D/4).max(5)).set("fill", "red")
            };
            doc = doc.add(c);
        }
    }

    for x in 0..input.N {
        for y in 0..input.N {
            let w = weight((x as i32, y as i32), input.N);
            doc = doc.add(Group::new()
                     .add(Title::new().add(Text::new(format!("({}, {})\nw={}", x, y, w))))
                     .add(Circle::new().set("cx", D / 2 + x * D).set("cy", H - D / 2 - y * D).set("r", D / 2 - 1).set("fill", "#00000000")
            ));
        }
    }

    let score = 0;
    let error = "".to_string();
    (score, error, doc.to_string())
}


