#![allow(non_snake_case, unused_imports)]
use proconio::input;
use svg::node::element::{Group, Rectangle, Style, Title, Line};
use svg::node::Text;

/*
#####################
便利関数
#####################
*/
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
pub enum Query {
    Survey(Vec<(usize, usize)>),
    Mining((usize, usize)),
    Ans(Vec<(usize, usize)>),
}

// シミュレーションの状態
#[derive(Clone, Debug)]
pub struct Sim {
    pub eps: f64, // エラーパラメータ
    // 真値
    pub oil_amount: Vec<Vec<i32>>, // 各マスの油田の量(実際は不明)
    pub errors: Vec<f64>,          // 誤差
    // 行動履歴
    pub query: Vec<Query>,      // クエリの履歴
    pub resp: Vec<i32>,         // クエリ結果の履歴
    pub cost: f64,              // 累積コスト
    pub count: usize,           // 油田の数
    pub mined: Vec<Vec<usize>>, // 採掘結果の履歴(そのマスを採掘したターン数が入る)
    pub finished: bool,         // 終了フラグ
    pub costs: Vec<f64>,        // コストの履歴
}

impl Sim {
    pub fn new(input: &Input) -> Self {
        let mut count = 0;
        for i in 0..input.map_size {
            for j in 0..input.map_size {
                if input.oil_amount[i][j] > 0 {
                    count += 1;
                }
            }
        }
        Self {
            eps: input.eps,
            oil_amount: input.oil_amount.clone(),
            errors: input.errors.clone(),
            query: vec![],
            resp: vec![],
            cost: 0.0,
            count,
            mined: mat![100000; input.map_size; input.map_size],
            finished: false,
            costs: vec![0.0],
        }
    }
    fn query(&mut self, q: Query) -> i32 {
        let resp = match q {
            Query::Mining((x, y)) => {
                self.cost += 1.0;
                self.mined[x][y].setmin(self.resp.len());
                self.oil_amount[x][y]
            }
            Query::Survey(ref ps) => {
                self.cost += 1.0 / (ps.len() as f64).sqrt();
                let mut sum = 0;
                for &(x, y) in ps {
                    sum += self.oil_amount[x][y];
                }
                let k = ps.len() as f64;
                let mu = (k - sum as f64) * self.eps + sum as f64 * (1.0 - self.eps);
                let sigma = (k * self.eps * (1.0 - self.eps)).sqrt();
                ((mu + self.errors[self.resp.len()] * sigma).round() as i32).max(0)
            }
            Query::Ans(ref ps) => {
                if ps.len() == self.count && ps.iter().all(|&(x, y)| self.oil_amount[x][y] > 0) {
                    self.finished = true;
                    1
                } else {
                    self.cost += 1.0;
                    0
                }
            }
        };
        self.query.push(q);
        self.resp.push(resp);
        self.costs.push(self.cost);
        resp
    }
}

/* 
#####################
入出力パート
#####################
*/

#[derive(Clone, Debug)]
pub struct Input {
    pub map_size: usize,                      // 島の大きさ 10 <= N <= 20
    pub oil_field_cnt: usize,                    // 油田の数  2 <= M <= 20
    pub eps: f64,                                // エラーパラメータ 0.01 <= eps <= 0.2
    pub oil_positions: Vec<Vec<(usize, usize)>>, // 油田ごとの相対座標集合(長方形の左上を0,0とする先頭のdは型が違うため含まれない)
    // 以降は真値なのでsubmissionの場合は含まれない
    pub ps: Vec<(usize, usize)>, // 油田クラスタを含む長方形の左上の座標(これが基準点になる)これを探索してoil_positionsとの整合性が保てればOK
    pub oil_amount: Vec<Vec<i32>>, // 各マスの油田量
    pub errors: Vec<f64>,        // 発生する誤差の真値
}

pub fn parse_input(f: &str) -> Input {
    let f = proconio::source::once::OnceSource::from(f);
    input! {
        from f,
        map_size: usize, oil_field_cnt: usize, eps: f64,
        oil_positions: [[(usize, usize)]; oil_field_cnt],
        ps: [(usize, usize); oil_field_cnt],
        oil_amount: [[i32; map_size]; map_size],
        errors: [f64; map_size * map_size * 2],
    }
    Input {
        map_size,
        oil_field_cnt,
        eps,
        oil_positions,
        ps,
        oil_amount,
        errors,
    }
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
    pub sim: Sim,
    pub comments: Vec<String>,
}

pub fn parse_output(input: &Input, f: &str) -> Result<Output, String> {
    let mut lines = f.lines();
    let mut sim = Sim::new(input);
    let mut comments = vec![];
    let mut comment = String::new();
    while sim.resp.len() < 2 * input.map_size * input.map_size {
        let Some(line) = lines.next() else {
            break;
        };
        let line = line.trim();
        if line.starts_with("#") {
            let line = line.trim_start_matches('#').trim();
            comment += line;
            comment.push('\n');
            continue;
        } else if line.is_empty() {
            continue;
        }
        comments.push(comment);
        comment = String::new();
        let mut ss = line.split_whitespace();
        let Some(ty) = ss.next() else {
            return Err(format!("Invalid query format: {}", line));
        };
        let num = read(ss.next(), 1, input.map_size * input.map_size)?;
        if ty == "a" {
            let mut ps = vec![];
            for _ in 0..num {
                let x = read(ss.next(), 0, input.map_size)?;
                let y = read(ss.next(), 0, input.map_size)?;
                ps.push((x, y));
            }
            ps.sort();
            ps.dedup();
            if ps.len() != num {
                return Err("Query contains the same square multiple times.".to_owned());
            }
            let resp = sim.query(Query::Ans(ps));
            if ss.next().is_some() {
                return Err(format!("Invalid query format: {}", line));
            }
            if resp == 1 {
                break;
            }
        } else if ty == "q" {
            let _resp = if num == 1 {
                sim.query(Query::Mining((read(ss.next(), 0, input.map_size)?, read(ss.next(), 0, input.map_size)?)))
            } else {
                let mut ps = vec![];
                for _ in 0..num {
                    let x = read(ss.next(), 0, input.map_size)?;
                    let y = read(ss.next(), 0, input.map_size)?;
                    ps.push((x, y));
                }
                ps.sort();
                ps.dedup();
                if ps.len() != num {
                    return Err("Query contains the same square multiple times.".to_owned());
                }
                sim.query(Query::Survey(ps))
            };
            if ss.next().is_some() {
                return Err(format!("Invalid query format: {}", line));
            }
        } else {
            return Err(format!("Invalid query format: {}", line));
        }
    }
    Ok(Output { sim, comments })
}


pub fn gen(seed: u64, map_size: usize, oil_field_cnt: usize, eps: f64) {
    // データを生成する
}

#[derive(Clone, Debug)]
pub struct State {

}
/* 
#####################
可視化パート 
#####################
*/
pub fn rect(x: usize, y: usize, w: usize, h: usize, fill: &str) -> Rectangle {
    Rectangle::new()
        .set("x", x)
        .set("y", y)
        .set("width", w)
        .set("height", h)
        .set("fill", fill)
}

pub fn group(title: String) -> Group {
    Group::new().add(Title::new().add(Text::new(title)))
}

pub fn vis(input: &Input, out: &Output, turn: usize) -> (i64, String, String) {
    // 可視化の実装
    let D = 500 / input.map_size;   // 1マスの大きさ(どのmap_sizeでも最大の大きさが600になるように調整)
    let H = D * input.map_size;
    let W = D * input.map_size;

    let mut doc = svg::Document::new()
        .set("id", "vis")
        .set("viewBox", (-10, -10, W + 10, H + 10))
        .set("width", W + 10)
        .set("height", H + 10)
        .set("style", "background-color:white");
    doc = doc.add(Style::new(format!(
        "text {{text-anchor: middle;dominant-baseline: central;}}"
    )));
    // whiteで構成されたH*Wの行列
    let mut color = mat!["white"; input.map_size; input.map_size];

    if turn > 0 {
        let turn = turn - 1;
        //行動に応じてcolorを変える
        match &out.sim.query[turn] {
            Query::Mining((y,x)) => {
                color[*y][*x] = "Lightcoral";
            }
            Query::Survey(ps) => {
                for (y,x) in ps {
                    color[*y][*x] = "Lightblue";
                }
            }
            Query::Ans(ps) => {
                for (y,x) in ps {
                    color[*y][*x] = "Lightgreen";
                }
            }
        }
    }

    // 正方形を1つずつ作成
    for i in 0..input.map_size {
        for j in 0..input.map_size {
            let mut group = group(format!("({}, {})", i, j)).add(rect(j * D, i * D, D, D, color[i][j]));
            // 過去の採掘ずみマス
            if out.sim.mined[i][j] < turn {
                // テキストを表示
                group = group.add(
                    svg::node::element::Text::new()
                    .add(Text::new((format!("{}", out.sim.oil_amount[i][j]))))  // 採掘によって得た油田量を表示
                    .set("x", j * D + D / 2)
                    .set("y", i * D + D / 2)
                    .set("font-size", D/3)
                    .set("fill", "black")
                );
            // 現在の採掘マス
            } else if out.sim.oil_amount[i][j] > 0 {
                // テキストを表示
                group = group.add(
                    svg::node::element::Text::new()
                    .add(Text::new((format!("{}", out.sim.oil_amount[i][j]))))  // 採掘によって得た油田量を表示
                    .set("x", j * D + D / 2)
                    .set("y", i * D + D / 2)
                    .set("font-size", D/3)
                    .set("fill", "darkgray")
                );
            }

            if out.sim.oil_amount[i][j] > 0 {
                // oil_amountの枠を囲むようにlineで表示
                group = group.add(
                    Line::new()
                    .set("x1", j * D)
                    .set("y1", i * D)
                    .set("x2", j * D + D)
                    .set("y2", i * D)
                    .set("stroke", "green")
                    .set("stroke-width", 5)
                );
                group = group.add(
                    Line::new()
                    .set("x1", j * D + D)
                    .set("y1", i * D)
                    .set("x2", j * D + D)
                    .set("y2", i * D + D)
                    .set("stroke", "green")
                    .set("stroke-width", 5)
                );
                group = group.add(
                    Line::new()
                    .set("x1", j * D + D)
                    .set("y1", i * D + D)
                    .set("x2", j * D)
                    .set("y2", i * D + D)
                    .set("stroke", "green")
                    .set("stroke-width", 5)
                );
                group = group.add(
                    Line::new()
                    .set("x1", j * D)
                    .set("y1", i * D + D)
                    .set("x2", j * D)
                    .set("y2", i * D)
                    .set("stroke", "green")
                    .set("stroke-width", 5)
                );
            }

            doc = doc.add(group);
        }
    }

    for i in 0..=input.map_size {
        // 縦線
        doc = doc.add(
            Line::new()
            .set("x1", 0)
            .set("y1", i * D)
            .set("x2", W)
            .set("y2", i * D)
            .set("stroke", "gray")
            .set("stroke-width", 2)
        );
        // 横線
        doc = doc.add(
            Line::new()
            .set("x1", i * D)
            .set("y1", 0)
            .set("x2", i * D)
            .set("y2", H)
            .set("stroke", "black")
            .set("stroke-width", 2)
        );
    }
    (0, "".to_string(), doc.to_string())
}


fn main() {
    todo!()
}