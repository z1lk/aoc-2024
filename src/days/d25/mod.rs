use crate::grid::Grid;

pub mod inputs {
    pub const REAL: &str = include_str!("real");
    pub const SAMPLE: &str = include_str!("sample");
}

#[derive(Debug)]
struct Schema {
    heights: Vec<i32>,
    grid: Grid<char>
}

fn parse(input: &str) -> (Vec<Schema>, Vec<Schema>) {
    let mut lines = crate::helpers::to_lines(input);
    lines.push("".to_string());

    let mut grid_lines: Vec<String> = Vec::new();

    let mut locks: Vec<Schema> = Vec::new();
    let mut keys: Vec<Schema> = Vec::new();

    for line in &lines {
        if line == "" {
            let grid = Grid::from_lines(grid_lines.clone());
            let is_lock = grid.get(0,0).unwrap() == '#';

            let mut heights: Vec<i32> = Vec::new();

            for x in (0..grid.get_width()) {
                let top = grid.get(x,0).unwrap();
                for y in (1..grid.get_height()) {
                    if top != grid.get(x,y).unwrap() {
                        if is_lock {
                            heights.push(y - 1);
                        } else {
                            heights.push(grid.get_height() - 1 - y);
                        }
                        break;
                    }
                }
            }

            let schema = Schema { grid, heights };

            if is_lock {
                locks.push(schema);
            } else {
                keys.push(schema);
            }

            grid_lines.clear();
        } else {
            grid_lines.push(line.clone());
        }
    }

    (locks, keys)
}

pub fn part_1(input: &str) -> i32 {
    let (locks, keys) = parse(input);

    /*
    for lock in &locks {
        println!("\n--- lock");
        println!("heights: {:?}", lock.heights);
        lock.grid.draw(false);
    }

    for key in &keys {
        println!("\n--- key");
        println!("heights: {:?}", key.heights);
        key.grid.draw(false);
    }
    */

    let mut fit = 0;

    for (l, lock) in locks.iter().enumerate() {
        'key: for (k, key) in keys.iter().enumerate() {
            for i in 0..5 {
                let lock_h = lock.heights.get(i).unwrap();
                let key_h = key.heights.get(i).unwrap();
                if (*key_h + *lock_h) > 5 {
                    continue 'key;
                }
            }
            fit += 1
        }
    }

    fit
}

pub fn part_2(input: &str) -> i32 {
    let (locks, keys) = parse(input);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(part_1(inputs::SAMPLE), 3);
    }

    #[test]
    fn part_1_real() {
        assert_eq!(part_1(inputs::REAL), 2993);
    }
/*
    #[test]
    fn part_2_sample() {
        assert_eq!(part_2(inputs::SAMPLE), 0);
    }

    #[test]
    fn part_2_real() {
        assert_eq!(part_2(inputs::REAL), 0);
    }
*/
}
