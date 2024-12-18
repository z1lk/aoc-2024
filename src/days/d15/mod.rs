use crate::grid::Grid;

pub mod inputs {
    pub const REAL: &str = include_str!("real");
    pub const SAMPLE: &str = include_str!("sample");
    pub const SAMPLE2: &str = include_str!("sample2");
}

fn parse(input: &str) -> (Grid<char>, Vec<Dir>) {
    let lines = crate::helpers::to_lines(input);
    let mut grid_lines: Vec<String> = Vec::new();
    let mut moves: Vec<Dir> = Vec::new();
    let mut done_grid = false;

    for line in &lines {
        if !done_grid {
            if line == "" {
                done_grid = true;
            } else {
                grid_lines.push(line.clone());
            }
            continue;
        }

        for c in line.chars() {
            let dir = match c {
                '^' => Dir::U,
                '>' => Dir::R,
                'v' => Dir::D,
                '<' => Dir::L,
                _ => panic!("unexpected char for dir {}", c)
            };
            moves.push(dir);
        }
    }

    (Grid::from_lines(grid_lines), moves)
}

#[derive(Debug)]
pub enum Dir { U, D, L, R }

fn next_pos(pos: (i32, i32), dir: &Dir) -> (i32, i32) {
    let offset = match dir {
        Dir::U => (0, -1),
        Dir::R => (1, 0),
        Dir::D => (0, 1),
        Dir::L => (-1, 0)
    };
    (pos.0 + offset.0, pos.1 + offset.1)
}

pub fn part_1(input: &str) -> i32 {
    let (mut grid, moves) = parse(input);

    for dir in moves {
        let pos = grid.find('@').unwrap();
        grid = move_dir(grid.clone(), pos, &dir);
    }

    let mut gps_sum = 0;
    for (oc, x, y) in grid.iter() {
        if oc.unwrap() == 'O' {
            gps_sum += 100 * y + x;
        }
    }
    gps_sum
}

pub fn move_dir(mut grid: Grid<char>, pos: (i32, i32), dir: &Dir) -> Grid<char> {
    let p = next_pos(pos, dir);
    let mut c = grid.get(p.0, p.1).unwrap();
    match c {
        '#' => { return grid },
        '@' => panic!("trying to move into robot"),
        'O' => {
            // try to move the box
            grid = move_dir(grid, p, dir);
            // we'll move below, if possible
        }
        '.' => {
            // we'll move below
        }
        _ => panic!("unexpected char {}", c),
    }

    // try to move, might not be able to if there was a box and it couldn't move
    c = grid.get(p.0, p.1).unwrap();
    if c == '.' {
        let s = grid.get(pos.0, pos.1).unwrap();
        grid.set('.', pos.0, pos.1);
        grid.set(s, p.0, p.1);
    }
    return grid;
}

pub fn part_2(input: &str) -> i32 {
    let (mut grid0, moves) = parse(input);

    // build a copy of the grid that is twice as wide
    let mut grid = Grid::fresh('.', grid0.get_width() * 2, grid0.get_height());
    for (oc, x, y) in grid0.iter() {
        match oc.unwrap() {
            '#' => {
                grid.set('#', x*2, y);
                grid.set('#', x*2+1, y);
            }
            '@' => {
                grid.set('@', x*2, y);
                grid.set('.', x*2+1, y);
            }
            'O' => {
                grid.set('[', x*2, y);
                grid.set(']', x*2+1, y);
            }
            '.' => {
                grid.set('.', x*2, y);
                grid.set('.', x*2+1, y);
            }
            c => panic!("unexpected char {}", c)
        }
    }

    for dir in moves {
        let pos = grid.find('@').unwrap();
        grid = move_dir_2(grid.clone(), pos, &dir);
    }

    let mut gps_sum = 0;
    for (oc, x, y) in grid.iter() {
        if oc.unwrap() == '[' {
            gps_sum += 100 * y + x;
        }
    }
    gps_sum
}

// recursive fn to check if object can move in direction
pub fn can_move_2(grid: &Grid<char>, pos: (i32, i32), dir: &Dir) -> bool {
    let p = next_pos(pos, dir);
    let c = grid.get(p.0, p.1).unwrap();
    match c {
        '#' => return false,
        '@' => panic!("trying to move into robot"),
        '[' => {
            return match dir {
                // when moving boxes U/D, have to check if the other piece can move U/D too.
                Dir::U | Dir::D => can_move_2(grid, p, dir) && can_move_2(grid, (p.0+1, p.1), dir),
                // moving away from other piece; it can move into this spot if this piece can move.
                Dir::L => can_move_2(grid, p, dir),
                // moving into other piece; just check the other piece as if it were another object.
                Dir::R => can_move_2(grid, (p.0+1, p.1), dir)
            }
        },
        ']' => {
            return match dir {
                Dir::U | Dir::D => can_move_2(grid, (p.0-1, p.1), dir) && can_move_2(grid, p, dir),
                Dir::L => can_move_2(grid, (p.0-1, p.1), dir),
                Dir::R => can_move_2(grid, p, dir)
            }
        },
        '.' => return true,
        _ => panic!("unexpected char {}", c),
    }
}
pub fn move_dir_2(mut grid: Grid<char>, pos: (i32, i32), dir: &Dir) -> Grid<char> {
    // first check self and anything in the way can move.
    // it's resursive, so we only need to check once, on the initial move of the robot.
    // for the boxes, we will have already checked.
    let s = grid.get(pos.0, pos.1).unwrap();
    if s == '@' {
        if !can_move_2(&grid, pos, dir) {
            return grid;
        }
    }

    // move anything that is in the way
    let p = next_pos(pos, dir);
    let c = grid.get(p.0, p.1).unwrap();
    match c {
        '[' | ']' => {
            match dir {
                Dir::U | Dir::D => {
                    let o = if c == '[' { (p.0+1,p.1) } else { (p.0-1,p.1) };
                    grid = move_dir_2(grid, o, dir);
                    grid = move_dir_2(grid, p, dir);
                }
                _ => {
                    grid = move_dir_2(grid, p, dir);
                }
            }
        }
        '.' => { /* nothing to move */ }
        _ => panic!("trying to move into {}", c),
    }

    // move self
    grid.set('.', pos.0, pos.1);
    grid.set(s, p.0, p.1);
    return grid;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(part_1(inputs::SAMPLE), 2028);
    }

    #[test]
    fn part_1_sample2() {
        assert_eq!(part_1(inputs::SAMPLE2), 10092);
    }

    #[test]
    fn part_1_real() {
        assert_eq!(part_1(inputs::REAL), 1495147);
    }

    #[test]
    fn part_2_sample2() {
        assert_eq!(part_2(inputs::SAMPLE2), 9021);
    }

    #[test]
    fn part_2_real() {
        assert_eq!(part_2(inputs::REAL), 1524905);
    }
}
