use std::str::FromStr;
use std::collections::HashMap;

pub mod inputs {
    pub const REAL: &str = include_str!("real");
    pub const SAMPLE: &str = include_str!("sample");
}

fn parse(input: &str) -> Vec<i64> {
    input
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

pub fn part_1(input: &str) -> i32 {
    let mut stones = parse(input);
    for n in (0..25) {
        stones = blink(stones);
    }
    stones.len() as i32
}

fn blink(old_stones: Vec<i64>) -> Vec<i64> {
    let mut stones: Vec<i64> = Vec::new();
    for stone in old_stones {
        if stone == 0 {
            stones.push(1);
            continue;
        }

        let string = stone.to_string();
        if string.len() % 2 == 0 {
            let (a, b) = string.split_at(string.len() / 2);
            stones.push(FromStr::from_str(a).unwrap());
            stones.push(FromStr::from_str(b).unwrap());
            continue;
        }

        stones.push(stone * 2024);
    }
    stones
}

// Very similar to part 1, and the solution here could be used for part 1.
// For part 1, we tracked all values in a vec at each step. The key insight is
// that the 3 rules create repeat stone values at each step. The "blink" algorithm
// of creating the stones for the next step can be highly optimized by tracking the
// count of each value at each step, and then sum them after 75 steps to get the final count.
fn parse_2(input: &str) -> HashMap<i64, i64> {
    let mut stones = HashMap::new();
    let values: Vec<i64> = input
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    for value in values {
        let count = stones.entry(value).or_insert(0);
        *count += 1;
    }
    stones
}

pub fn part_2(input: &str) -> i64 {
    let mut stones = parse_2(input);
    for n in (0..75) {
        stones = blink_2(stones);
    }
    stones.values().sum::<i64>() as i64
}

fn blink_2(old_stones: HashMap<i64, i64>) -> HashMap<i64, i64> {
    let mut stones: HashMap<i64, i64> = HashMap::new();
    for (stone, count) in old_stones {
        if stone == 0 {
            let ex = stones.entry(1).or_insert(0);
            *ex += count;
            continue;
        }

        let string = stone.to_string();
        if string.len() % 2 == 0 {
            let (a, b) = string.split_at(string.len() / 2);
            let ex = stones.entry(FromStr::from_str(a).unwrap()).or_insert(0);
            *ex += count;
            let ex = stones.entry(FromStr::from_str(b).unwrap()).or_insert(0);
            *ex += count;
            continue;
        }

        let ex = stones.entry(stone * 2024).or_insert(0);
        *ex += count;
    }
    stones
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(part_1(inputs::SAMPLE), 55312);
    }

    #[test]
    fn part_1_real() {
        assert_eq!(part_1(inputs::REAL), 189167);
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(part_2(inputs::SAMPLE), 65601038650482);
    }

    #[test]
    fn part_2_real() {
        assert_eq!(part_2(inputs::REAL), 225253278506288);
    }
}
