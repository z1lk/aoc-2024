pub mod inputs {
    pub const REAL: &str = include_str!("real");
    pub const SAMPLE: &str = include_str!("sample");
}

fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    let lines = crate::helpers::to_lines(input);
    let mut ids1: Vec<i32> = Vec::new();
    let mut ids2: Vec<i32> = Vec::new();

    for line in lines {
        println!("{}", line);
        let items: Vec<&str> = line.split("   ").collect();
        ids1.push(items[0].parse::<i32>().unwrap_or(0));
        ids2.push(items[1].parse::<i32>().unwrap_or(0));
    }

    (ids1, ids2)
}

pub fn part_1(input: &str) -> i32 {
    let (mut ids1, mut ids2) = parse(input);

    ids1.sort();
    ids2.sort();

    ids1.iter()
        .zip(ids2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

pub fn part_2(input: &str) -> i32 {
    let (ids1, ids2) = parse(input);
    
    ids1.iter()
        .map(|id| {
            let count = ids2.iter().filter(|&x| *x == *id).count() as i32;
            id * count
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(part_1(inputs::SAMPLE), 11);
    }

    #[test]
    fn part_1_real() {
        assert_eq!(part_1(inputs::REAL), 1580061);
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(part_2(inputs::SAMPLE), 31);
    }

    #[test]
    fn part_2_real() {
        assert_eq!(part_2(inputs::REAL), 23046913);
    }
}
