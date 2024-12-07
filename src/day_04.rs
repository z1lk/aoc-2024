use crate::grid::Grid;

fn read(input: &str) -> Vec<Vec<char>> {
    let lines = crate::helpers::read_input_to_lines(input);
    lines.iter().map(|line| line.chars().collect()).collect()
}

pub fn part_1(input: &str) -> i32 {
    let rows: Vec<Vec<char>> = read(input);
    let grid = Grid::new(rows);

    let mut xmas = 0;

    for (c, x, y) in grid.iter() {
        if c != Some('X') { continue; }
        for (c, xo, yo) in grid.neighbors(x, y).iter() {
            if (*c != Some('M')) { continue; }
            if (grid.get(x + xo * 2, y + yo * 2) != Some('A')) { continue; }
            if (grid.get(x + xo * 3, y + yo * 3) != Some('S')) { continue; }
            xmas += 1;
        }
    }

    xmas
}

pub fn part_2(input: &str) -> i32 {
    let rows: Vec<Vec<char>> = read(input);
    let grid = Grid::new(rows);

    let mut x_mas = 0;

    for (c, x, y) in grid.iter() {
        if c != Some('A') { continue; }

        let tl = grid.get(x-1, y-1);
        let tr = grid.get(x+1, y-1);
        let bl = grid.get(x-1, y+1);
        let br = grid.get(x+1, y+1);

        if (![tl, tr, bl, br].iter().all(|c| *c == Some('M') || *c == Some('S'))) {
            continue;
        }

        if (tl != br && tr != bl) {
            x_mas += 1;
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
