use regex::Regex;

pub mod inputs {
    pub const REAL: &str = include_str!("real");
    pub const SAMPLE: &str = include_str!("sample");
    pub const SAMPLE2: &str = include_str!("sample2");
}

fn parse(input: &str) -> String {
    input.to_string()
}

pub fn part_1(input: &str) -> i32 {
    let mem = parse(input);

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum = 0;

    for (_full, [a, b]) in re.captures_iter(&mem).map(|c| c.extract()) {
        let a = a.parse::<i32>().unwrap();
        let b = b.parse::<i32>().unwrap();
        sum += a * b
    }

    sum
}

pub fn part_2(input: &str) -> i32 {
    let mem = parse(input);

    let re = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();

    let mut on = true;
    let mut sum = 0;

    for cap in re.captures_iter(&mem) {
        let full = cap.get(0).unwrap().as_str();
        if (full == "do()") {
            on = true;
        } else if (full == "don't()") {
            on = false;
        } else { // mul
            if on {
                let a = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
                let b = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
                sum += a * b
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(part_1(inputs::SAMPLE), 161);
    }

    #[test]
    fn part_1_real() {
        assert_eq!(part_1(inputs::REAL), 165225049);
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(part_2(inputs::SAMPLE2), 48);
    }

    #[test]
    fn part_2_real() {
        assert_eq!(part_2(inputs::REAL), 108830766);
    }
}
