use crate::grid::Grid;

pub mod inputs {
    pub const REAL: &str = include_str!("real");
    pub const SAMPLE: &str = include_str!("sample");
}

fn parse(input: &str) -> Grid<char> {
    Grid::from_lines(crate::helpers::to_lines(input))
}

pub fn part_1(input: &str) -> i32 {
    let grid = parse(input);
    let mut antinodes = Grid::fresh('.', grid.get_width(), grid.get_height());
    for (c, x, y) in grid.iter() {
        if (c == Some('.')) { continue }
        for (c2, x2, y2) in grid.iter() {
            if (c != c2) { continue }
            if (x == x2 && y == y2) { continue }
            let ox = x2 - x;
            let oy = y2 - y;
            let ax = x - ox;
            let ay = y - oy;
            if (ax >= 0 && ax <= (antinodes.get_width() - 1) && ay >= 0 && ay <= (antinodes.get_height() - 1)) {
                antinodes.set('#', ax, ay);
            }
        }
    }
    antinodes.count('#')
}

pub fn part_2(input: &str) -> i32 {
    let grid = parse(input);
    let mut antinodes = Grid::fresh('.', grid.get_width(), grid.get_height());
    for (c, x, y) in grid.iter() {
        if (c == Some('.')) { continue }
        for (c2, x2, y2) in grid.iter() {
            if (c != c2) { continue }
            if (x == x2 && y == y2) { continue }
            antinodes.set('#', x, y);
            antinodes.set('#', x2, y2);
            let mut ox = x2 - x;
            let mut oy = y2 - y;
            let mut ax = x - ox;
            let mut ay = y - oy;
            while (ax >= 0 && ax <= (antinodes.get_width() - 1) && ay >= 0 && ay <= (antinodes.get_height() - 1)) {
                antinodes.set('#', ax, ay);
                ax = ax - ox;
                ay = ay - oy;
            }
        }
    }
    antinodes.count('#')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(part_1(inputs::SAMPLE), 14);
    }

    #[test]
    fn part_1_real() {
        assert_eq!(part_1(inputs::REAL), 240);
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(part_2(inputs::SAMPLE), 34);
    }

    #[test]
    fn part_2_real() {
        assert_eq!(part_2(inputs::REAL), 955);
    }
}
