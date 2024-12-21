use crate::grid::Grid;
use std::collections::HashMap;
use std::{thread, time};

pub mod inputs {
    pub const REAL: &str = include_str!("real");
    pub const SAMPLE: &str = include_str!("sample");
}

fn parse(input: &str) -> Vec<String> {
    crate::helpers::to_lines(input)
}

/*
pub fn part_1_original(input: &str) -> i32 {
    let codes = parse(input);

    let mut robot = Robot::new(2);

    let mut total_complexity = 0;

    for code in codes {
        for c in code.chars() {
            robot.press(c);
        }

        let num = code.get(0..3).unwrap().parse::<i32>().unwrap();
        let presses = robot.get_presses();

        let complexity = num * presses.len() as i32;
        total_complexity += complexity;
        robot.clear_presses();
    }

    total_complexity
}
*/

pub fn part_1(input: &str) -> i64 {
    let codes = parse(input);
    let mut robot = Robot::new(4);
    let mut total_complexity = 0;

    for code in codes {
        let mut presses = 0;
        for c in code.chars() {
            presses += robot.press_and_get(c);
        }
        let num = code.get(0..3).unwrap().parse::<i64>().unwrap();
        let complexity = num * presses;
        total_complexity += complexity;
    }

    total_complexity
}

pub fn part_2(input: &str) -> i64 {
    let codes = parse(input);
    let mut robot = Robot::new(27);
    let mut total_complexity = 0;

    for code in codes {
        let mut presses = 0;
        for c in code.chars() {
            presses += robot.press_and_get(c);
        }
        let num = code.get(0..3).unwrap().parse::<i64>().unwrap();
        let complexity = num * presses;
        total_complexity += complexity;
    }

    total_complexity
}

/*
fn print_keys(keys: &Vec<char>) {
    let string: String = keys.into_iter().collect();
    println!("{}", string);
}
*/

fn dir_to_char(dir: Dir) -> char {
    match dir {
        Dir::U => '^',
        Dir::R => '>',
        Dir::D => 'v',
        Dir::L => '<'
    }
}

struct Robot {
    id: i32,
    pad: Grid<char>,
    cur: char,
    presses: Vec<char>,
    presses_cache: HashMap::<String, i64>,
    controller: Option<Box<Robot>>
}

impl Robot {
    pub fn new(chain_len: i32) -> Self {
        let id = 1;
        let numpad = Grid::from_str("789\n456\n123\n#0A");
        let mut r = Robot {
            id,
            pad: numpad,
            cur: 'A',
            presses: Vec::<char>::new(),
            presses_cache: HashMap::new(),
            controller: None
        };
        if id < chain_len {
            let controller = Robot::new_controller(id + 1, chain_len);
            r.controller = Some(Box::new(controller));
        }
        r
    }

    pub fn new_controller(id: i32, chain_len: i32) -> Self {
        let keypad = Grid::from_str("#^A\n<v>");
        let mut r = Robot {
            id,
            pad: keypad,
            cur: 'A',
            presses: Vec::<char>::new(),
            presses_cache: HashMap::new(),
            controller: None
        };
        if id < chain_len {
            let controller = Robot::new_controller(id + 1, chain_len);
            r.controller = Some(Box::new(controller));
        }
        r
    }

    // This is for the original solution for part 1.
    // It does not do any caching, and stores all keypresses
    // which can be later be grabbed and reset.
    /*
    fn get_presses(&mut self) -> Vec<char> {
        match &mut self.controller {
            None => return self.presses.clone(),
            Some(r) => {
                return r.get_presses();
            }
        }
    }

    fn clear_presses(&mut self) {
        self.presses = Vec::new();
        match &mut self.controller {
            None => (),
            Some(r) => { r.clear_presses() }
        }
    }

    fn press(&mut self, c: char) {
        self.presses.push(c);
        // dirs self needs to move to push c
        let dirs = get_dirs(&self.pad, self.cur, c);
        self.cur = c;
        // chars that controller needs to press to move self
        let mut chars: Vec<char> = dirs.into_iter().map(|d| dir_to_char(d)).collect();
        // controller needs to press A afterwards
        chars.push('A');
        match &mut self.controller {
            None => (),
            Some(r) => {
                for c2 in chars {
                    r.press(c2);
                }
            }
        }
    }
    */

    fn press_and_get(&mut self, c: char) -> i64 {
        let cache_key = format!("{}:{}", self.cur, c);
        if let Some(v) = self.presses_cache.get(&cache_key) {
            // Very important that we set the current key as if we moved,
            // even though we are returning a cached value. The cost of future
            // presses depends on which key we are starting from.
            self.cur = c;
            return *v
        }

        self.presses.push(c);
        // dirs self needs to move to push c
        let dirs = get_dirs(&self.pad, self.cur, c);
        self.cur = c;
        // chars that controller needs to press to move self
        let mut chars: Vec<char> = dirs.into_iter().map(|d| dir_to_char(d)).collect();
        // controller needs to press A afterwards
        chars.push('A');
        let num = match &mut self.controller {
            None => 1,
            Some(r) => {
                let mut sum = 0_i64;
                for c2 in chars {
                    let x = r.press_and_get(c2);
                    sum += x;
                }
                sum
            }
        };

        self.presses_cache.insert(cache_key.clone(), num.into());

        num
    }
}

#[derive(Clone,Debug)]
struct Node {
    c: char,
    x: i32,
    y: i32,
    cost: i32,
    offset: Option<(i32, i32)>
}

// get a list of the directions to move from key `start` to key `end` on keypad `grid`
fn get_dirs(grid: &Grid<char>, start: char, end: char) -> Vec<Dir> {
    let mut dirs = Vec::<Dir>::new();

    if start == end { return dirs }

    let (x1, y1) = grid.find(start).unwrap();
    let (x2, y2) = grid.find(end).unwrap();

    let dy = y2 - y1;
    let dx = x2 - x1;

    let mut up = 0;
    let mut right = 0;
    let mut down = 0;
    let mut left = 0;

    if dy < 0 { up = dy.abs() }
    if dy > 0 { down = dy }
    if dx < 0 { left = dx.abs() }
    if dx > 0 { right = dx }

    // We need to order the dirs to minimize key presses of parent,
    // but we also can't make the robot move over the empty spot.

    let mut done = false;

    // if U/D is the empty spot, go L/R first
    let c = grid.get(x1, y1+dy).unwrap();
    if c == '#' {
        done = true;
        for _ in 0..left { dirs.push(Dir::L) }
        for _ in 0..right { dirs.push(Dir::R) }
        for _ in 0..up { dirs.push(Dir::U) }
        for _ in 0..down { dirs.push(Dir::D) }
    }

    // if L/R is the empty spot, go U/D first
    if !done {
        let c = grid.get(x1+dx, y1).unwrap();
        if c == '#' {
            done = true;
            for _ in 0..up { dirs.push(Dir::U) }
            for _ in 0..down { dirs.push(Dir::D) }
            for _ in 0..left { dirs.push(Dir::L) }
            for _ in 0..right { dirs.push(Dir::R) }
        }
    }

    if !done {
        // Neither move hits the empty spot.
        // Going L is expensive because it is the left-most key from A,
        // where we always return. So we want to get L presses out of
        // the way at the beginning, and then remaining presses
        // will require R/U/D which are cheaper.
        //
        // This is hard to get an intuitive sense of, but you can prove it by thinking
        // about the moves required in a chain length greater than 3.
        // https://www.reddit.com/r/adventofcode/comments/1hj7f89/comment/m34erhg
        for _ in 0..left { dirs.push(Dir::L) }
        for _ in 0..up { dirs.push(Dir::U) }
        for _ in 0..down { dirs.push(Dir::D) }
        for _ in 0..right { dirs.push(Dir::R) }
    }

    dirs
}

#[derive(Debug)]
pub enum Dir { U, D, L, R }

fn offset_to_dir(dx: i32, dy: i32) -> Dir {
    match (dx,dy) {
        (0, -1) => Dir::U,
        (1, 0) => Dir::R,
        (0, 1) => Dir::D,
        (-1, 0) => Dir::L,
        _ => panic!("unexpected offsets {:?},{:?}", dx, dy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(part_1(inputs::SAMPLE), 126384);
    }

    #[test]
    fn part_1_real() {
        assert_eq!(part_1(inputs::REAL), 184180);
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(part_2(inputs::SAMPLE), 154115708116294);
    }

    #[test]
    fn part_2_real() {
        assert_eq!(part_2(inputs::REAL), 231309103124520);
    }
}
