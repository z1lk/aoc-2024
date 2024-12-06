fn read(input: &str) -> Vec<Vec<char>> {
    let lines = crate::helpers::read_input_to_lines(input);
    lines.iter().map(|line| line.chars().collect()).collect()
}

pub fn part_1(input: &str) -> i32 {
    let grid: Vec<Vec<char>> = read(input);
    let offsets: [i32; 3] = [-1, 0, 1];
    let mut xmas = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell != 'X' { continue; }

            for xo in offsets {
                for yo in offsets {
                    if (xo == 0 && yo == 0) { continue; }
                    if (get(&grid, x, y, xo  , yo  ) != 'M') { continue; }
                    if (get(&grid, x, y, xo*2, yo*2) != 'A') { continue; }
                    if (get(&grid, x, y, xo*3, yo*3) != 'S') { continue; }

                    xmas += 1;
                }
            }
        }
    }

    xmas
}

fn get(grid: &Vec<Vec<char>>, x: usize, y: usize, xo: i32, yo: i32) -> char {
    let x2 = (x as i32) + xo;
    let y2 = (y as i32) + yo;

    // if out of bounds, return '!' which won't match
    if x2 < 0 { return '!'; }
    if y2 < 0 { return '!'; }
    if x2 > grid[0].len() as i32 - 1 { return '!'; }
    if y2 > grid.len() as i32 - 1 { return '!'; }

    grid[y2 as usize][x2 as usize]
}

pub fn part_2(input: &str) -> i32 {
    let grid: Vec<Vec<char>> = read(input);

    let mut x_mas = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell != 'A' { continue; }

            let tl = get(&grid, x, y, -1, -1);
            let tr = get(&grid, x, y, 1, -1);
            let bl = get(&grid, x, y, -1, 1);
            let br = get(&grid, x, y, 1, 1);

            if (![tl, tr, bl, br].iter().all(|c| *c == 'M' || *c == 'S')) {
                continue;
            }

            if (tl != br && tr != bl) {
                x_mas += 1;
            }
        }
    }

    x_mas
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_1() {
        assert_eq!(part_1("04_sample"), 18);
    }

    #[test]
    fn answer_1() {
        assert_eq!(part_1("04"), 2618);
    }

    #[test]
    fn sample_2() {
        assert_eq!(part_2("04_sample"), 9);
    }

    #[test]
    fn answer_2() {
        assert_eq!(part_2("04"), 2011);
    }
}
