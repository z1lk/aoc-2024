use crate::grid::Grid;
use std::collections::HashMap;

pub mod inputs {
    pub const REAL: &str = include_str!("real");
    pub const SAMPLE: &str = include_str!("sample");
}

fn parse(input: &str) -> Grid<char> {
    Grid::from_lines(crate::helpers::to_lines(input))
}

pub fn part_1(input: &str) -> i32 {
    let mut grid = parse(input);
    let path = pathfind(&grid);

    let cheat_counts = count_cheats(path, 2);

    let mut gteq_100 = 0;
    for (key, val) in cheat_counts.iter() {
        if *key >= 100 { gteq_100 += val; }
    }
    gteq_100
}

pub fn part_2(input: &str) -> i32 {
    let mut grid = parse(input);
    let path = pathfind(&grid);

    let cheat_counts = count_cheats(path, 20);

    let mut gteq_100 = 0;
    for (key, val) in cheat_counts.iter() {
        if *key >= 100 { gteq_100 += val; }
    }
    gteq_100
}

// Count the possible cheats on the path, returning a map of time saved => number of cheats
fn count_cheats(path: Vec<(i32, i32)>, cheat_len: i32) -> HashMap<i32, i32> {
    let mut cheat_counts: HashMap<i32, i32> = HashMap::new();

    for i in 0..path.len() {
        for j in 0..path.len() {
            if j == i { continue } // same
            if j > i { continue } // already checked when i > j
            let dist = (i - j) as i32; // num steps between the points
            if dist <= cheat_len { continue } // not worth cheating
            let m = path.get(i).unwrap();
            let n = path.get(j).unwrap();
            // taxicab distance between points, i.e. the time to cheat
            let cheat_dist = (m.0 - n.0).abs() + (m.1 - n.1).abs();
            if (cheat_dist <= cheat_len) {
                let time_saved = dist - cheat_dist;
                let count = cheat_counts.entry(time_saved).or_insert(0);
                *count += 1;
            }
        }
    }

    cheat_counts
}

#[derive(Clone,Debug)]
struct Node {
    c: char,
    x: i32,
    y: i32,
    cost: i32,
    prev: Option<(i32, i32)>
}

// Dijkstra's algorithm to find the path on the track
fn pathfind(grid: &Grid<char>) -> Vec<(i32, i32)> {
    // build a list of unvisited nodes from the grid
    let mut unvisited: Vec<Node> = Vec::new();
    for (oc, x, y) in grid.iter() {
        let c = oc.unwrap();
        match c {
            '.' | 'E' => {
                unvisited.push(Node {
                    c, x, y,
                    cost: i32::MAX,
                    prev: None
                });
            }
            'S' => {
                unvisited.push(Node {
                    c, x, y,
                    cost: 0,
                    prev: None
                });
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
                let node = unvisited.get_mut(i).unwrap();
                let cost = cur.cost + 1;
                if cost < node.cost {
                    node.cost = cost;
                    node.prev = Some((cur.x,cur.y));
                }
            }
        }

        visited.push(cur);

        unvisited.sort_by(|m, n| m.cost.cmp(&n.cost));
        cur = unvisited.remove(0); // swap_remove?

        if cur.c == 'E' {
            // unwind the nodes' prev values to get the best path
            let mut path = vec![(cur.x,cur.y)];
            let mut node = cur.clone();
            let mut coords = cur.prev;
            while coords != None {
                let co = coords.unwrap();
                path.push(coords.unwrap());
                node = visited.iter().find(|node| node.x == co.0 && node.y == co.1).unwrap().clone();
                coords = node.prev;
            }
            return path;
        }
        if unvisited.len() == 0 { panic!("visited all nodes!") }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /*#[test]
    fn part_1_sample() {
        assert_eq!(part_1(inputs::SAMPLE), 0);
    }*/

    #[test]
    fn part_1_real() {
        assert_eq!(part_1(inputs::REAL), 1321);
    }

    /*#[test]
    fn part_2_sample() {
        assert_eq!(part_2(inputs::SAMPLE), 0);
    }*/

    #[test]
    fn part_2_real() {
        assert_eq!(part_2(inputs::REAL), 971737);
    }
}
