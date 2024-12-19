use crate::grid::Grid;

pub mod inputs {
    pub const REAL: &str = include_str!("real");
    pub const SAMPLE: &str = include_str!("sample");
}

fn parse(input: &str) -> (Grid<char>, Vec<(i32,i32)>) {
    let lines = crate::helpers::to_lines(input);

    let mut points: Vec<(i32, i32)> = Vec::new();

    for line in lines {
        let nums: Vec<i32> = line.split(',').map(|s| s.parse::<i32>().unwrap()).collect();
        points.push(
            ( *nums.get(0).unwrap(), *nums.get(1).unwrap() )
        );
    }

    let size = if input == inputs::SAMPLE { 7 } else { 71 };
    let mut grid = Grid::fresh('.', size, size);
    grid.set('S', 0, 0);
    grid.set('E', size-1, size-1);

    (grid, points)
}

#[derive(Debug)]
struct Node {
    c: char,
    x: i32,
    y: i32,
    cost: i32
}

pub fn part_1(input: &str) -> i32 {
    let (mut grid, points) = parse(input);

    let num_to_fall = if input == inputs::SAMPLE { 12 } else { 1024 };

    for i in 0..num_to_fall {
        let (x,y) = points.get(i).unwrap();
        grid.set('#', *x, *y);
    }

    exit_cost(&grid).unwrap()
}

pub fn part_2(input: &str) -> String {
    let (mut grid, points) = parse(input);

    // we know from part 1 that there is an exit with these bytes
    let mut num_to_fall = if input == inputs::SAMPLE { 12 } else { 1024 };

    // so go ahead and set them on the grid
    for i in 0..num_to_fall {
        let (x,y) = points.get(i).unwrap();
        grid.set('#', *x, *y);
    }

    // Loop on: drop the next byte, check if we can exit.
    // If not then return the coords of that byte.
    loop {
        println!("{:?}", num_to_fall);
        let (x,y) = points.get(num_to_fall).unwrap();
        num_to_fall += 1;
        grid.set('#', *x, *y);
        match exit_cost(&grid) {
            Some(_) => continue,
            None => return format!("{:?},{:?}", x, y)
        }
    }
}

// Pathfind on the grid from S->E, returning the number of steps.
// Pulled from day 16, implementation of Dijkstra's algorithm, minus the turn cost.
pub fn exit_cost(grid: &Grid<char>) -> Option<i32> {
    // build a list of unvisited nodes from the grid
    let mut unvisited: Vec<Node> = Vec::new();
    for (oc, x, y) in grid.iter() {
        let c = oc.unwrap();
        match c {
            '.' | 'E' => {
                unvisited.push(Node { c, x, y, cost: i32::MAX });
            }
            'S' => {
                unvisited.push(Node { c, x, y, cost: 0 });
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
                let cost = cur.cost + 1;
                let mut node = unvisited.remove(i);
                if cost < node.cost {
                    node.cost = cost;
                }
                unvisited.push(node);
            }
        }

        if unvisited.len() == 0 { return None }

        unvisited.sort_by(|m, n| m.cost.cmp(&n.cost));
        cur = unvisited.remove(0);

        // only remaining nodes are unreachable
        if cur.cost == i32::MAX { return None }

        if cur.c == 'E' { return Some(cur.cost) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(part_1(inputs::SAMPLE), 22);
    }

    #[test]
    fn part_1_real() {
        assert_eq!(part_1(inputs::REAL), 356);
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(part_2(inputs::SAMPLE), "6,1");
    }

    #[test]
    fn part_2_real() {
        // takes a minute
        //assert_eq!(part_2(inputs::REAL), "22,33");
    }
}
