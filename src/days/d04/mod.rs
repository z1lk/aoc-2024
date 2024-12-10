use crate::grid::Grid;

pub mod inputs {
    pub const REAL: &str = include_str!("real");
    pub const SAMPLE: &str = include_str!("sample");
}

fn parse(input: &str) -> Grid {
    Grid::from_lines(crate::helpers::to_lines(input))
}

pub fn part_1(input: &str) -> i32 {
    let grid: Grid = parse(input);

    let mut xmas = 0;

    for (c, x, y) in grid.iter() {
        if c != Some('X') { continue; }
        for (c, dx, dy) in grid.neighbors(x, y).iter() {
            if (*c != Some('M')) { continue; }
            if (grid.get(x + dx * 2, y + dy * 2) != Some('A')) { continue; }
            if (grid.get(x + dx * 3, y + dy * 3) != Some('S')) { continue; }
            xmas += 1;
        }
    }

    xmas
}

pub fn part_2(input: &str) -> i32 {
    let grid: Grid = parse(input);

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
    fn part_1_sample() {
        assert_eq!(part_1(inputs::SAMPLE), 18);
    }

    #[test]
    fn part_1_real() {
        assert_eq!(part_1(inputs::REAL), 2618);
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(part_2(inputs::SAMPLE), 9);
    }

    #[test]
    fn part_2_real() {
        assert_eq!(part_2(inputs::REAL), 2011);
    }
}
