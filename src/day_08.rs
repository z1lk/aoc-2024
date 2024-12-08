use crate::grid::Grid;

fn read(input: &str) -> Grid {
    Grid::from_lines(crate::helpers::read_input_to_lines(input))
}

pub fn part_1(input: &str) -> i32 {
    let grid = read(input);
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
    let grid = read(input);
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
    fn sample_1() {
        assert_eq!(part_1("08_sample"), 14);
    }

    #[test]
    fn answer_1() {
        assert_eq!(part_1("08"), 240);
    }

    #[test]
    fn sample_2() {
        assert_eq!(part_2("08_sample"), 34);
    }

    #[test]
    fn answer_2() {
        assert_eq!(part_2("08"), 955);
    }
}
