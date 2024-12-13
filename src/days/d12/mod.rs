use crate::grid::Grid;

pub mod inputs {
    pub const REAL: &str = include_str!("real");
    pub const SAMPLE: &str = include_str!("sample");
    pub const SAMPLE2: &str = include_str!("sample2");
    pub const SAMPLE3: &str = include_str!("sample3");
    pub const SAMPLE4: &str = include_str!("sample4");
    pub const SAMPLE5: &str = include_str!("sample5");
}

fn parse(input: &str) -> (Grid, Vec<Vec<(i32,i32)>>) {
    let grid = Grid::from_lines(crate::helpers::to_lines(input));

    // identify plots by character on grid. the solutions operate on them
    // idependently, so we don't need to record the char.
    let mut plots: Vec<Vec<(i32, i32)>> = Vec::new();
    for (c, x, y) in grid.iter() {
        let mut seen = false;
        for plot in &plots {
            if plot.contains(&(x,y)) {
                seen = true;
                break;
            }
        }
        if (seen) { continue }
        let mut plot: Vec<(i32, i32)> = Vec::new();
        plot.push((x,y));
        build_plot(c.unwrap(), x, y, &grid, &mut plot);
        plots.push(plot);
    }

    (grid, plots)
}

fn build_plot(letter: char, x: i32, y: i32, grid: &Grid, plot: &mut Vec<(i32, i32)>) {
    for (c, dx, dy) in grid.neighbors(x, y, false).iter() {
        if (Some(letter) == *c) {
            let x2 = x + dx;
            let y2 = y + dy;
            if plot.contains(&(x2,y2)) { continue }
            plot.push((x2, y2));
            build_plot(letter, x2, y2, grid, plot);
        }
    }
}

pub fn part_1(input: &str) -> i32 {
    let (grid, mut plots) = parse(input);

    let mut price = 0;

    // calculate the price of a plot
    for plot in &plots {
        let area = plot.len();
        // build a new grid
        let mut plot_grid = Grid::fresh('.', grid.get_width(), grid.get_height());
        // mark the plot cells
        for (x, y) in plot {
            plot_grid.set('x', *x, *y);
        }
        let mut perim = 0;
        // for every plot cell, inc perim by its non-plot neighbors
        for (c, x, y) in plot_grid.iter() {
            if c != Some('x') { continue }
            for (d, dx, dy) in plot_grid.neighbors(x, y, false).iter() {
                if *d == Some('x') { continue }
                perim += 1;
            }
        }
        price += (area * perim) as i32;
    }

    price
}

// Similar to part 1, except have to collapse adjacent pieces of perimeter as a single "side".
// The way we count these sides is by gathering all the edges, and then only counting the first in
// a series, which are identifiable by whether they have a piece above (in the case of vertical
// edges) or to the left (in the case of horizontal edges). There are some edge cases that cause
// improper counting, which are resolved by performing the operation for up/down/left/right edges
// independent of each other.
pub fn part_2(input: &str) -> i32 {
    let (grid, mut plots) = parse(input);

    let mut price = 0;

    for plot in &plots {
        let area = plot.len();
        let mut plot_grid = Grid::fresh('.', grid.get_width(), grid.get_height());
        for (x, y) in plot {
            plot_grid.set('x', *x, *y);
        }

        let mut u_edges: Vec<(i32, i32)> = Vec::new();
        let mut d_edges: Vec<(i32, i32)> = Vec::new();
        let mut l_edges: Vec<(i32, i32)> = Vec::new();
        let mut r_edges: Vec<(i32, i32)> = Vec::new();

        for (c, x, y) in plot_grid.iter() {
            if c != Some('x') { continue }
            for (d, dx, dy) in plot_grid.neighbors(x, y, false).iter() {
                if *d == Some('x') { continue }
                let x2 = x + dx;
                let y2 = y + dy;
                if *dy == -1 { u_edges.push((x2,y2)) }
                if *dy == 1 { d_edges.push((x2,y2)) }
                if *dx == -1 { l_edges.push((x2,y2)) }
                if *dx == 1 { r_edges.push((x2,y2)) }
            }
        }

        let mut sides = 0;

        for edges in [l_edges, r_edges] {
            for (x,y) in &edges {
                let mut adj = false;
                for (x2,y2) in &edges {
                    if (*x==*x2 && *y==*y2+1) { adj = true; break; }
                }
                if (!adj) { sides += 1; }
            }
        }

        for edges in [u_edges, d_edges] {
            for (x,y) in &edges {
                let mut adj = false;
                for (x2,y2) in &edges {
                    if (*x==*x2+1 && *y==*y2) { adj = true; break; }
                }
                if (!adj) { sides += 1; }
            }
        }

        price += (area * sides) as i32;
    }

    price
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(part_1(inputs::SAMPLE), 140);
    }

    #[test]
    fn part_1_sample2() {
        assert_eq!(part_1(inputs::SAMPLE2), 772);
    }

    #[test]
    fn part_1_sample3() {
        assert_eq!(part_1(inputs::SAMPLE3), 1930);
    }

    #[test]
    fn part_1_real() {
        assert_eq!(part_1(inputs::REAL), 1550156);
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(part_2(inputs::SAMPLE), 80);
    }

    #[test]
    fn part_2_sample2() {
        assert_eq!(part_2(inputs::SAMPLE2), 436);
    }

    #[test]
    fn part_2_sample3() {
        assert_eq!(part_2(inputs::SAMPLE3), 1206);
    }

    #[test]
    fn part_2_sample4() {
        assert_eq!(part_2(inputs::SAMPLE4), 236);
    }

    #[test]
    fn part_2_sample5() {
        assert_eq!(part_2(inputs::SAMPLE5), 368);
    }

    #[test]
    fn part_2_real() {
        assert_eq!(part_2(inputs::REAL), 946084);
    }
}
