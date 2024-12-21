use crate::grid::Grid;
//use std::collections::HashMap;
use std::{thread, time};

pub mod inputs {
    pub const REAL: &str = include_str!("real");
    pub const SAMPLE: &str = include_str!("sample");
}

fn parse(input: &str) -> Vec<String> {
    crate::helpers::to_lines(input)
}

pub fn part_1(input: &str) -> i32 {
    let codes = parse(input);
    //println!("{:?}",codes);
    let numpad = Grid::from_str("789\n456\n123\n#0A");
    //numpad.draw(false);
    let keypad = Grid::from_str("#^A\n<v>");
    //keypad.draw(false);

    let mut r1 = Robot { pad: numpad.clone(), cur: 'A' };
    let mut r2 = Robot { pad: keypad.clone(), cur: 'A' };
    let mut r3 = Robot { pad: keypad.clone(), cur: 'A' };

    let sleep = 0;

    let mut total_complexity = 0;

    //let code = codes.get(2).unwrap();
    for code in codes {
        let mut r1_keys: Vec<char> = Vec::new();
        let mut r2_keys: Vec<char> = Vec::new();
        let mut r3_keys: Vec<char> = Vec::new();
        let mut my_keys: Vec<char> = Vec::new();

        let num = code.get(0..3).unwrap().parse::<i32>().unwrap();

        //println!("==== {}", code);
        thread::sleep(time::Duration::from_millis(sleep));

        // for numeric code
        for c in code.chars() {
            // dirs that r1 needs to move to press it
            let dirs2 = get_dirs(&r1.pad, r1.cur, c);
            r1.cur = c;
            // chars that r2 needs to press to move r1
            let mut chars2: Vec<char> = dirs2.into_iter().map(|d| dir_to_char(d)).collect();
            // r2 needs to press A afterwards
            chars2.push('A');
            r1_keys.push(c);
            //println!("=== {}", c);
            //println!("chars2: {:?}",chars2);
            thread::sleep(time::Duration::from_millis(sleep));
            for c2 in chars2 {
                // dirs that r2 needs to move to press it
                let mut dirs3 = get_dirs(&r2.pad, r2.cur, c2);
                r2.cur = c2;
                // chars that r3 needs to press to move r2
                let mut chars3: Vec<char> = dirs3.into_iter().map(|d| dir_to_char(d)).collect();
                // r3 needs to press A afterwards
                chars3.push('A');
                r2_keys.push(c2);
                //println!("== {}", c2);
                //println!("chars3: {:?}",chars3);
                thread::sleep(time::Duration::from_millis(sleep));
                for c3 in chars3 {
                    // dirs that r3 needs to move to press it
                    let mut dirs4 = get_dirs(&r3.pad, r3.cur, c3);
                    r3.cur = c3;
                    // chars that I need to press to move r3
                    let mut chars4: Vec<char> = dirs4.into_iter().map(|d| dir_to_char(d)).collect();
                    // I need to press A afterwards
                    chars4.push('A');
                    r3_keys.push(c3);
                    //println!("= {}", c3);
                    //println!("chars4: {:?}",chars4);
                    thread::sleep(time::Duration::from_millis(sleep));
                    for c4 in chars4 {
                        my_keys.push(c4);
                        //print!("{}", c4);
                    }
                    //println!("");
                }
            }
        }
        //println!("---");
        //println!("{:?}", my_keys.len());
        //print_keys(&my_keys);
        //println!("{:?}", num);
        let complexity = num * (my_keys.len() as i32);
        total_complexity += complexity;
        //print_keys(&r3_keys);
        //print_keys(&r2_keys);
        //print_keys(&r1_keys);
    }

    total_complexity
}

fn print_keys(keys: &Vec<char>) {
    let string: String = keys.into_iter().collect();
    println!("{}", string);
}

fn dir_to_char(dir: Dir) -> char {
    match dir {
        Dir::U => '^',
        Dir::R => '>',
        Dir::D => 'v',
        Dir::L => '<'
    }
}

struct Robot {
    pad: Grid<char>,
    cur: char
}

#[derive(Clone,Debug)]
struct Node {
    c: char,
    x: i32,
    y: i32,
    cost: i32,
    offset: Option<(i32, i32)>
}

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

    // We need to order the dirs in a certain way to minimize key presses of parent,
    // but we also can't make the robot move over the empty spot.

    // check what the cell upwards is
    
    let mut done = false;

    // if U/D is the empty spot, so go L/R first
    let c = grid.get(x1, y1+dy).unwrap();
    if c == '#' {
        done = true;
        for _ in 0..left { dirs.push(Dir::L) }
        for _ in 0..right { dirs.push(Dir::R) }
        for _ in 0..up { dirs.push(Dir::U) }
        for _ in 0..down { dirs.push(Dir::D) }
    }

    // if L/R is the empty spot, so go U/D first
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
        // neither axis has the empty spot, prioritize this way
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

pub fn part_2(input: &str) -> i32 {
    let parsed = parse(input);
    0
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
