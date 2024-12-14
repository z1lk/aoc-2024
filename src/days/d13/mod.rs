use regex::Regex;
use std::str::FromStr;
use std::collections::HashMap;

pub mod inputs {
    pub const REAL: &str = include_str!("real");
    pub const SAMPLE: &str = include_str!("sample");
}

#[derive(Debug)]
struct Game {
    // button A
    ax: i64, ay: i64,
    // button B
    bx: i64, by: i64,
    // prize
    px: i64, py: i64,
}

fn parse(input: &str) -> Vec<Game> {
    let mut games: Vec<Game> = Vec::new();

    let a_re = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
    let b_re = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
    let p_re = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    let lines = crate::helpers::to_lines(input);

    let mut nums: HashMap<&str, i64> = HashMap::new();

    for line in lines {
        if a_re.is_match(&line) {
            let caps = a_re.captures(&line).unwrap();
            nums.insert( "ax", FromStr::from_str(caps.get(1).unwrap().as_str()).unwrap() );
            nums.insert( "ay", FromStr::from_str(caps.get(2).unwrap().as_str()).unwrap() );
        }
        if b_re.is_match(&line) {
            let caps = b_re.captures(&line).unwrap();
            nums.insert( "bx", FromStr::from_str(caps.get(1).unwrap().as_str()).unwrap() );
            nums.insert( "by", FromStr::from_str(caps.get(2).unwrap().as_str()).unwrap() );
        }
        if p_re.is_match(&line) {
            let caps = p_re.captures(&line).unwrap();
            nums.insert( "px", FromStr::from_str(caps.get(1).unwrap().as_str()).unwrap() );
            nums.insert( "py", FromStr::from_str(caps.get(2).unwrap().as_str()).unwrap() );
            games.push(Game {
                ax: *nums.get("ax").unwrap(), ay: *nums.get("ay").unwrap(),
                bx: *nums.get("bx").unwrap(), by: *nums.get("by").unwrap(),
                px: *nums.get("px").unwrap(), py: *nums.get("py").unwrap()
            });
        }
    }

    games
}

pub fn part_1(input: &str) -> i64 {
    let games = parse(input);
    let mut cost = 0_i64;
    for mut game in games {
        if let Some(game_cost) = cost_to_win(game) {
            cost += game_cost;
        }
    }
    cost
}

pub fn part_2(input: &str) -> i64 {
    let games = parse(input);
    let mut cost = 0;
    let p_inc = 10000000000000_i64;
    for mut game in games {
        game.px += p_inc;
        game.py += p_inc;
        if let Some(game_cost) = cost_to_win(game) {
            cost += game_cost;
        }
    }
    cost
}

/* Solving as system of equations by elimination.
 * This is where the formula to calculate B presses comes from:
 *
 * (a * m) + (b * n) = o // presses of A (a) and B (b) that move by AX (m) and BX (n) to get to PX (o)
 * (a * p) + (b * q) = r // presses of A (a) and B (b) that move by AY (p) and BY (q) to get to PY (r)
 *
 * p * (a * m) + p * (b * n) = p * o
 * m * (a * p) + m * (b * q) = m * r
 *
 * (p * a * m) + (p * b * n) = p * o
 * (m * a * p) + (m * b * q) = m * r
 *
 * (p * a * m) + (p * b * n)
 * - (m * a * p) - (m * b * q)
 * = (p * o) - (m * r)
 *
 * (p * b * n) - (m * b * q) = (p * o) - (m * r)
 * b * ((p * n) - (m * q)) = (p * o) - (m * r)
 * b = ((p * o) - (m * r)) / ((p * n) - (m * q))
 *
 * and then solve for A presses:
 *
 * (a * m) + (b * n) = o
 * (a * m) = o - (b * n)
 * a = (o - (b * n)) / m
 * */

fn cost_to_win(game: Game) -> Option<i64> {
    let m = game.ax as f64;
    let n = game.bx as f64;
    let o = game.px as f64;
    let p = game.ay as f64;
    let q = game.by as f64;
    let r = game.py as f64;

    let b = ((p * o) - (m * r)) / ((p * n) - (m * q));

    if b.fract() == 0.0 {
        let a = (o - (b * n)) / m;
        if a.fract() == 0.0 {
            let game_cost = 3_i64 * (a as i64) + (b as i64);
            return Some(game_cost);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(part_1(inputs::SAMPLE), 480);
    }

    #[test]
    fn part_1_real() {
        assert_eq!(part_1(inputs::REAL), 38714);
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(part_2(inputs::SAMPLE), 875318608908);
    }

    #[test]
    fn part_2_real() {
        assert_eq!(part_2(inputs::REAL), 74015623345775);
    }
}
