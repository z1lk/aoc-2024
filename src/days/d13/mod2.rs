// I was getting the wrong answer and so tried using a decimal lib in case
// float arithmetic was the cause. Ended up not being the cause of the bug.
// Using floats is fine, although this solution using the decimal lib works too.

use regex::Regex;
use std::str::FromStr;
use std::collections::HashMap;
use rust_decimal::prelude::*;

pub mod inputs {
    pub const REAL: &str = include_str!("real");
    pub const SAMPLE: &str = include_str!("sample");
}

#[derive(Debug)]
struct Game {
    // button A
    ax: i32, ay: i32,
    // button B
    bx: i32, by: i32,
    // prize
    px: i32, py: i32,
}

fn parse(input: &str) -> Vec<Game> {
    let mut games: Vec<Game> = Vec::new();

    let a_re = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
    let b_re = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
    let p_re = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    let lines = crate::helpers::to_lines(input);

    let mut nums: HashMap<&str, i32> = HashMap::new();

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


/* solving as sys of eq by elimination. this is where the equation to calc b comes from
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
 * and then solve for a
 *
 * (a * m) + (b * n) = o
 * (a * m) = o - (b * n)
 * a = (o - (b * n)) / m
 * */

pub fn part_1(input: &str) -> i32 {
    let games = parse(input);

    let mut cost = 0;

    for game in games {
        println!("{:?}", game);

        let m = game.ax as f32;
        let n = game.bx as f32;
        let o = game.px as f32;
        let p = game.ay as f32;
        let q = game.by as f32;
        let r = game.py as f32;

        let b = ((p * o) - (m * r)) / ((p * n) - (m * q));

        println!("{:?}", b);

        if b.fract() == 0.0 {
            let a = (o - (b * n)) / m;
            //println!("solution: a={:?} b={:?}", a, b);
            let game_cost = 3 * (a as i32) + (b as i32);
            println!("game cost: {:?}", game_cost);
            cost += game_cost;
        }
    }
    cost
}

pub fn part_2(input: &str) -> i64 {
    let games = parse(input);

    let mut cost = 0_i64;

    for game in games {
        println!("\n{:?}", game);

        let p_inc = 10000000000000_i64;

        let m = Decimal::from_i32(game.ax).unwrap();
        let n = Decimal::from_i32(game.bx).unwrap();
        let o = Decimal::from_i64(game.px as i64 + p_inc).unwrap();
        let p = Decimal::from_i32(game.ay).unwrap();
        let q = Decimal::from_i32(game.by).unwrap();
        let r = Decimal::from_i64(game.py as i64 + p_inc).unwrap();

        let mut game_cost = i64::MAX;

        // These bits were to check if we could beat the game by only pressing A or B,
        // which might be cheaper. Not the case for the given inputs.
        /*
        // can we beat game only pressing A
        let a_press_x = o / m;
        let a_press_y = r / p;
        if a_press_x.fract() == 0.0 {
            println!("MAYBE CAN WIN WITH A ({:?}/{:?})", a_press_x, a_press_y);
            if a_press_x == a_press_y {
                println!("CAN WIN WITH A");
                let a_press_cost = (a_press_x as i64) * 3;
                if a_press_cost < game_cost {
                    game_cost = a_press_cost;
                }
            }
        }

        // can we beat game only pressing B
        let b_press_x = o / n;
        let b_press_y = r / q;
        if b_press_x.fract() == 0.0 {
            println!("MAYBE CAN WIN WITH B ({:?}/{:?})", b_press_x, b_press_y);
            if b_press_x == b_press_y {
                println!("CAN WIN WITH B");
                let b_press_cost = b_press_x as i64;
                if b_press_cost < game_cost {
                    game_cost = b_press_cost;
                }
            }
        }
        */

        // let b = ((p * o) - (m * r)) / ((p * n) - (m * q));
        let b =
            (
                p.checked_mul(o).unwrap().checked_sub( m.checked_mul(r).unwrap() ).unwrap()
            ).checked_div(
                p.checked_mul(n).unwrap().checked_sub( m.checked_mul(q).unwrap() ).unwrap()
            ).unwrap();
        //println!("{:?}", b);

        if b.fract().is_zero() {
            //let a = (o - (b * n)) / m;
            let a = o.checked_sub(
                b.checked_mul(n).unwrap()
            ).unwrap().checked_div(m).unwrap();
            println!("solution: a={:?} b={:?}", a, b);
            if a.fract().is_zero() {
                let ab_press_cost = 3 * a.to_i64().unwrap() + b.to_i64().unwrap();
            //println!("game cost: {:?}", game_cost);
            //if ab_press_cost < game_cost {
                game_cost = ab_press_cost;
            //}
            // not 75758977198101
            }
        }

        if game_cost != i64::MAX {
            cost += game_cost;
        }
    }
    cost
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(part_1(inputs::SAMPLE), 0);
    }

    #[test]
    fn part_1_real() {
        assert_eq!(part_1(inputs::REAL), 0);
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(part_2(inputs::SAMPLE), 0);
    }

    #[test]
    fn part_2_real() {
        assert_eq!(part_2(inputs::REAL), 0);
    }
}
