use regex::Regex;
use std::str::FromStr;
use crate::grid::Grid;
use std::{thread, time};

pub mod inputs {
    pub const REAL: &str = include_str!("real");
    pub const SAMPLE: &str = include_str!("sample");
}

#[derive(Debug)]
struct Robot {
    pub x: i32, pub y: i32,
    pub vx: i32, pub vy: i32
}

fn parse(input: &str) -> Vec<Robot> {
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    let lines = crate::helpers::to_lines(input);
    let mut robots: Vec<Robot> = Vec::new();
    for line in lines {
        let caps = re.captures(&line).unwrap();
        robots.push(Robot {
            x: FromStr::from_str(caps.get(1).unwrap().as_str()).unwrap(),
            y: FromStr::from_str(caps.get(2).unwrap().as_str()).unwrap(),
            vx: FromStr::from_str(caps.get(3).unwrap().as_str()).unwrap(),
            vy: FromStr::from_str(caps.get(4).unwrap().as_str()).unwrap(),
        });
    }
    robots
}

pub fn part_1(input: &str) -> i32 {
    let mut robots = parse(input);
    // the width/height are given separately
    let (width, height) =
        if input == inputs::SAMPLE {
            (11, 7)
        } else {
            (101, 103)
        };
    //draw(&robots, width, height);

    for _ in (0..100) {
        for robot in &mut robots {
            step(robot, width, height);
        }
    }

    // count robots in each quadrant
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;
    // w/h are guaranteed to be odd
    let half_width = (width - 1) / 2;
    let half_height = (height - 1) / 2;

    for robot in robots {
        if robot.x < half_width && robot.y < half_height {
            q1 += 1;
        } else if robot.x > half_width && robot.y < half_height {
            q2 += 1;
        } else if robot.x > half_width && robot.y > half_height {
            q3 += 1;
        } else if robot.x< half_width && robot.y > half_height {
            q4 += 1;
        } // else not counted
    }

    q1 * q2 * q3 * q4
}

pub fn part_2(input: &str) -> i32 {
    let mut robots = parse(input);
    // the width/height are given separately
    let (width, height) =
        if input == inputs::SAMPLE {
            (11, 7)
        } else {
            (101, 103)
        };

    let mut i = 0;
    loop {
        // some key cells to look for--the border around the xmas tree
        // remove them as we see them. when vec is empty then easter egg is present.
        let mut border = vec![ (41,20), (41,21), (41,22), (42,20), (43,20) ];

        i += 1;
        for robot in &mut robots {
            step(robot, width, height);
            if let Some(i) = border.iter().position(|&(x,y)| x==robot.x && y==robot.y) {
                border.remove(i);
            }
        }

        if border.is_empty() {
            // ensure it's the full image
            //draw(&robots, width, height);
            break;
        }

        // first spotted tree around this time. paused to get some key cells to look for.
        /*if i >= 28150 && i % 5 == 0 {
            draw(&robots, width, height);
            println!("\n");
            println!("i={:?}", i);
            return 0;
        }*/
    }
    i
}

fn step(robot: &mut Robot, width: i32, height: i32) {
    robot.x += robot.vx;
    robot.y += robot.vy;
    if robot.x > width - 1 {
        robot.x -= width;
    } else if robot.x < 0 {
        robot.x += width;
    }
    if robot.y > height - 1 {
        robot.y -= height;
    } else if robot.y < 0 {
        robot.y += height;
    }
}

fn draw(robots: &Vec<Robot>, width: i32, height: i32) {
    thread::sleep(time::Duration::from_millis(1000));
    let mut grid = Grid::fresh('.', width, height);
    for robot in robots {
        grid.set('X', robot.x, robot.y);
    }
    println!("\n");
    grid.draw(true);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(part_1(inputs::SAMPLE), 12);
    }

    #[test]
    fn part_1_real() {
        assert_eq!(part_1(inputs::REAL), 211773366);
    }

    /*#[test]
    fn part_2_sample() {
        assert_eq!(part_2(inputs::SAMPLE), 0);
    }*/

    #[test]
    fn part_2_real() {
        assert_eq!(part_2(inputs::REAL), 7344);
    }
}
