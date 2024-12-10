use crate::grid::Grid;
use std::collections::HashMap;

pub mod inputs {
    pub const REAL: &str = include_str!("real");
    pub const SAMPLE: &str = include_str!("sample");
    pub const SAMPLE2: &str = include_str!("sample2");
    pub const SAMPLE3: &str = include_str!("sample3");
    pub const SAMPLE4: &str = include_str!("sample4");
    pub const SAMPLE_LARGE: &str = include_str!("sample_large");
    pub const P2_SAMPLE: &str = include_str!("p2_sample");
    pub const P2_SAMPLE2: &str = include_str!("p2_sample2");
    pub const P2_SAMPLE3: &str = include_str!("p2_sample3");
}

fn parse(input: &str) -> Grid {
    Grid::from_lines(crate::helpers::to_lines(input))
}

pub fn part_1(input: &str) -> i32 {
    let grid: Grid = parse(input);
    let mut trails = HashMap::new();
    for (c, x, y) in grid.iter() {
        if c != Some('0') { continue; }
        trails.insert((x,y), Vec::<(i32, i32)>::new());
        let peaks: &mut Vec<(i32, i32)> = trails.get_mut(&(x,y)).unwrap();
        push_peaks(x, y, &grid, peaks);
    }
    let mut score = 0;
    for (head, peaks) in trails {
        score += peaks.len() as i32;
    }
    score
}

fn next_height(height: char) -> char {
    match height {
        '0' => '1',
        '1' => '2',
        '2' => '3',
        '3' => '4',
        '4' => '5',
        '5' => '6',
        '6' => '7',
        '7' => '8',
        '8' => '9',
        _ => { panic!("unknown next_height for {:?}", height) }
    }
}

// recursive fn that looks at a grid cells neighbors, steps up until it reaches height 9,
// and pushes that peak onto the peaks vec, if it hasn't been found yet.
fn push_peaks(x: i32, y: i32, grid: &Grid, peaks: &mut Vec<(i32, i32)>) {
    let height = grid.get(x, y).unwrap();
    let next = next_height(height);
    for (c, dx, dy) in grid.neighbors(x, y).iter() {
        // neighbors() returns diagonals, but we only want UDLR, in which case one of dx/dy will be 0
        if (*dx != 0 && *dy != 0) { continue } 
        if let Some(h) = c {
            if (*h != next) { continue }
            let x2 = x + dx;
            let y2 = y + dy;
            if next == '9' {
                if !peaks.contains(&(x2, y2)) {
                    peaks.push((x2, y2));
                }
            } else {
                push_peaks(x2, y2, grid, peaks);
            }
        }
    }
}

// same as part 1, except that instead of tracking unique peaks, we track unique paths to peaks
pub fn part_2(input: &str) -> i32 {
    let grid: Grid = parse(input);
    let mut trails = HashMap::new();
    for (c, x, y) in grid.iter() {
        if c != Some('0') { continue; }
        trails.insert((x,y), Vec::<Vec<(i32, i32)>>::new());
        let paths: &mut Vec<Vec<(i32, i32)>> = trails.get_mut(&(x,y)).unwrap();
        let path: Vec<(i32, i32)> = Vec::new();
        push_paths(x, y, &grid, path.clone(), paths);
    }
    let mut score = 0;
    for (head, paths) in trails {
        score += paths.len() as i32;
    }
    score
}

fn push_paths(x: i32, y: i32, grid: &Grid, mut path: Vec<(i32, i32)>, paths: &mut Vec<Vec<(i32, i32)>>) {
    let height = grid.get(x, y).unwrap();
    let next = next_height(height);
    for (c, dx, dy) in grid.neighbors(x, y).iter() {
        if (*dx != 0 && *dy != 0) { continue } 
        if let Some(h) = c {
            if (*h != next) { continue }
            let x2 = x + dx;
            let y2 = y + dy;
            path.push((x2, y2));
            if next == '9' {
                if !paths.contains(&path) {
                    paths.push(path.clone());
                }
            } else {
                push_paths(x2, y2, grid, path.clone(), paths);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(part_1(inputs::SAMPLE), 1);
    }

    #[test]
    fn part_1_sample2() {
        assert_eq!(part_1(inputs::SAMPLE2), 2);
    }

    #[test]
    fn part_1_sample3() {
        assert_eq!(part_1(inputs::SAMPLE3), 4);
    }

    #[test]
    fn part_1_sample4() {
        assert_eq!(part_1(inputs::SAMPLE4), 3);
    }

    #[test]
    fn part_1_sample_large() {
        assert_eq!(part_1(inputs::SAMPLE_LARGE), 36);
    }

    #[test]
    fn part_1_real() {
        assert_eq!(part_1(inputs::REAL), 517);
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(part_2(inputs::P2_SAMPLE), 3);
    }

    #[test]
    fn part_2_sample2() {
        assert_eq!(part_2(inputs::P2_SAMPLE2), 13);
    }

    #[test]
    fn part_2_sample3() {
        assert_eq!(part_2(inputs::P2_SAMPLE3), 227);
    }

    #[test]
    fn part_2_sample_large() {
        assert_eq!(part_2(inputs::SAMPLE_LARGE), 81);
    }

    #[test]
    fn part_2_real() {
        assert_eq!(part_2(inputs::REAL), 1116);
    }

}
