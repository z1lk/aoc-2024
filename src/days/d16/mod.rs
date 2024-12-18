use crate::grid::Grid;
use std::{thread, time};

pub mod inputs {
    pub const REAL: &str = include_str!("real");
    pub const SAMPLE: &str = include_str!("sample");
    pub const SAMPLE2: &str = include_str!("sample2");
}

fn parse(input: &str) -> Grid<char> {
    Grid::from_lines(crate::helpers::to_lines(input))
}


#[derive(Clone,Debug,PartialEq)]
pub enum Dir { U, D, L, R, Z } // Z == unknown

#[derive(Debug)]
struct Node {
    c: char,
    x: i32,
    y: i32,
    cost: i32,
    dir: Dir
}

// solved with Dijkstra's algorithm
pub fn part_1(input: &str) -> i32 {
    let mut grid = parse(input);

    // build a list of unvisited nodes from the grid
    let mut unvisited: Vec<Node> = Vec::new();
    for (oc, x, y) in grid.iter() {
        let c = oc.unwrap();
        match c {
            '.' | 'E' => {
                unvisited.push(Node { c, x, y, cost: i32::MAX, dir: Dir::Z });
            }
            'S' => {
                unvisited.push(Node { c, x, y, cost: 0, dir: Dir::R });
            }
            _ => ()
        }
    }

    let mut visited: Vec<Node> = Vec::new();

    // sort unvisited to move Start to the front
    unvisited.sort_by(|m, n| m.cost.cmp(&n.cost));
    let mut cur = unvisited.remove(0);

    // loop until we find End
    loop {
        for (oc, dx, dy) in grid.neighbors(cur.x, cur.y, false) {
            let x = cur.x + dx;
            let y = cur.y + dy;
            if let Some(i) = unvisited.iter().position(|n| n.x == x && n.y == y) {
                let dir = match (dx,dy) {
                    (0, -1) => Dir::U,
                    (1, 0) => Dir::R,
                    (0, 1) => Dir::D,
                    (-1, 0) => Dir::L,
                    _ => panic!("unexpected offsets {:?},{:?}", dx, dy)
                };
                let mut cost = cur.cost + 1;
                if dir != cur.dir { cost += 1000; }
                let mut node = unvisited.remove(i);
                if cost < node.cost {
                    node.cost = cost;
                    node.dir = dir;
                }
                unvisited.push(node);
            }
        }

        unvisited.sort_by(|m, n| m.cost.cmp(&n.cost));
        cur = unvisited.remove(0);
        if cur.c == 'E' { break }
        if unvisited.len() == 0 { panic!("visited all nodes!") }
    }

    cur.cost
}

pub fn part_2(input: &str) -> i32 {
    // use our part_1 solution to get the best path
    let best_cost = part_1(input);
    let mut grid = parse(input);

    // grid for maintaining best costs for each cell
    // `i32::MAX - 1001` gives us room to to add 1001 to the values, the cost of 1 step and turn
    let mut costs_grid: Grid<i32> = Grid::fresh(i32::MAX - 1001, grid.get_width(), grid.get_height());

    let mut todo_paths: Vec<Path> = Vec::new();
    let mut path = Path {
        steps: Vec::new(),
        cost: 0,
        dir: Dir::R
    };

    path.steps.push(grid.find('S').unwrap());
    todo_paths.push(path);
    let mut done_paths: Vec<Path> = Vec::new();

    let mut i = 0;
    while todo_paths.len() > 0 {
        i += 1;

        // shift off the first path to work on it
        let path = todo_paths.swap_remove(0);

        /*if i % 10000 == 0 {
            let mut grid2 = grid.clone();
            for (a,b) in path.steps.clone() {
                grid2.set('*', a, b);
            }
            grid2.draw(true);
            thread::sleep(time::Duration::from_millis(100));
        }*/

        // this ensures any "done" paths will necessarily be equal to best
        if path.cost > best_cost { continue }

        // get the last step
        let last = path.steps.last().unwrap();

        // skip if cost of the last step is more that a step and turn than
        // the observed best for this cell, so far
        let last_cur_best = costs_grid.get(last.0, last.1).unwrap();
        if path.cost > last_cur_best + 1001 { continue }

        // mark cost on this cell if better than what we have observed
        if path.cost < last_cur_best {
            costs_grid.set(path.cost, last.0, last.1);
        }

        pathfind(&grid, &mut todo_paths, &mut done_paths, path);
    }

    // gather all the unique cells on all best paths
    let mut unique: Vec<(i32, i32)> = Vec::new();
    for done_path in done_paths {
        for step in done_path.steps {
            if !unique.contains(&step) {
                unique.push(step);
            }
        }
    }

    unique.len() as i32
}

#[derive(Clone,Debug)]
struct Path {
    pub steps: Vec<(i32, i32)>,
    pub cost: i32,
    pub dir: Dir
}

// Take a path, and for each of its potential next steps, construct a new path and push it onto
// `todo_paths`. If it reaches the end, push it onto `done_paths`.
fn pathfind(grid: &Grid<char>, todo_paths: &mut Vec<Path>, done_paths: &mut Vec<Path>, mut path: Path) {
    let (x,y) = path.steps.last().unwrap();

    let mut neighbors = grid.neighbors(*x, *y, false);

    for (oc, dx, dy) in neighbors {
        let p = (*x + dx, *y + dy);
        if path.steps.contains(&p) { continue }
        let c = oc.unwrap();

        match c {
            'S' | '#'  => (),
            'E' | '.'  => {
                let mut path = path.clone();
                path.steps.push(p);
                path.cost += 1;
                let dir = match (dx, dy) {
                    (-1,0) => Dir::L, (1,0) => Dir::R, (0,-1) => Dir::U, (0,1) => Dir::D,
                    _ => panic!("unexpected offset {:?},{:?}", dx, dy)
                };
                if dir != path.dir {
                    path.cost += 1000;
                    path.dir = dir;
                }

                match c {
                    '.' => {
                        todo_paths.push(path);
                        todo_paths.sort_by(|a, b| {
                            a.cost.cmp(&b.cost)
                        });
                    },
                    'E' => {
                        // println!("END");
                        done_paths.push(path);
                        done_paths.sort_by(|a, b| {
                            a.cost.cmp(&b.cost)
                        });
                    }
                    _ => panic!("unexpected char {:?}", oc)
                }
            },
            _ => panic!("unexpected char {:?}", oc)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(part_1(inputs::SAMPLE), 7036);
    }

    #[test]
    fn part_1_sample2() {
        assert_eq!(part_1(inputs::SAMPLE2), 11048);
    }

    #[test]
    fn part_1_real() {
        assert_eq!(part_1(inputs::REAL), 122492);
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(part_2(inputs::SAMPLE), 45);
    }

    #[test]
    fn part_2_sample2() {
        assert_eq!(part_2(inputs::SAMPLE2), 64);
    }

    #[test]
    fn part_2_real() {
        assert_eq!(part_2(inputs::REAL), 520);
    }
}
