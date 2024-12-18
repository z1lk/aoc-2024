use std::{thread, time};

use crate::grid::Grid;

pub mod inputs {
    pub const REAL: &str = include_str!("real");
    pub const SAMPLE: &str = include_str!("sample");
}

#[derive(Copy,Clone,Debug,PartialEq)]
pub enum Dir { U, D, L, R }

pub enum Result { Exit, Loop }

fn parse(input: &str) -> Grid<char> {
    Grid::from_lines(crate::helpers::to_lines(input))
}

pub fn part_1(input: &str) -> i32 {
    let mut grid: Grid<char> = parse(input);
    let (res, trail) = walk(&mut grid, Dir::U);
    let mut cells: Vec<(i32, i32)> = Vec::new();
    for (cell, dir) in trail {
        if !cells.contains(&cell) { cells.push(cell) }
    }
    // plus last position before exit
    (cells.len() + 1).try_into().unwrap()
}

pub fn part_2(input: &str) -> i32 {
    let grid: Grid<char> = parse(input);
    let mut grid: Grid<char> = parse(input);
    let mut obstacles = grid.clone();

    // walk the grid once to get the trail
    let (res, trail) = walk(&mut grid.clone(), Dir::U);
    // for each step in the trail, place an obstacle in the next pos and walk.
    // if we loop, mark it in the `obstacles` grid clone
    for (i, (pos, dir)) in trail.iter().enumerate() {
        let mut grid2 = grid.clone();
        let next = next_pos(*pos, *dir);
        match grid2.get(next.0, next.1) {
            Some('.') | Some('X') => grid2.set('O', next.0, next.1),
            Some('^') | Some('#') | None => continue,
            Some(other) => panic!("uenxpected char in grid {}", other)
        }

        match walk(&mut grid2, Dir::U) {
            (Result::Exit, _) => (),
            (Result::Loop, _) => { obstacles.set('O', next.0, next.1) }
        }
    }

    //obstacles.draw();
    obstacles.count('O')
}

// wrapper around recursive _walk fn that initializes and returns the trail
pub fn walk(grid: &mut Grid<char>, dir: Dir) -> (Result, Vec<((i32, i32), Dir)>) {
    let mut trail: Vec<((i32, i32), Dir)> = Vec::new();
    let res = _walk(grid, dir, &mut trail);
    (res, trail)
}

fn next_pos(pos: (i32, i32), dir: Dir) -> (i32, i32) {
    let offset = match dir {
        Dir::U => (0, -1),
        Dir::R => (1, 0),
        Dir::D => (0, 1),
        Dir::L => (-1, 0)
    };
    (pos.0 + offset.0, pos.1 + offset.1)
}

fn next_dir(dir: Dir) -> Dir {
    match dir {
        Dir::U => Dir::R,
        Dir::R => Dir::D,
        Dir::D => Dir::L,
        Dir::L => Dir::U
    }
}

// recursive fn to walk the grid
pub fn _walk(grid: &mut Grid<char>, dir: Dir, trail: &mut Vec<((i32, i32), Dir)>) -> Result {
    //thread::sleep(time::Duration::from_millis(100));
    //grid.draw();
    let pos = grid.find('^').unwrap();
    let next = next_pos(pos, dir);
    match grid.get(next.0, next.1) {
        Some('.') | Some('X') => {
            let step = (pos, dir);
            if (trail.contains(&step)) { return Result::Loop }
            trail.push(step);
            //grid.set('X', pos.0, pos.1);
            grid.set('.', pos.0, pos.1);
            grid.set('^', next.0, next.1);
            return _walk(grid, dir, trail);
        },
        Some('#') | Some('O') => return _walk(grid, next_dir(dir), trail),
        None => return Result::Exit,
        Some(other) => panic!("unxpected char in grid {}", other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(part_1(inputs::SAMPLE), 41);
    }

    #[test]
    fn part_1_real() {
        assert_eq!(part_1(inputs::REAL), 4778);
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(part_2(inputs::SAMPLE), 6);
    }

    #[test]
    fn part_2_real() {
        // this takes a couple minutes on release build
        //assert_eq!(part_2(inputs::REAL), 1618);
    }
}
