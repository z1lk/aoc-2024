// use itertools::Itertools;
use regex::Regex;

fn read(input: &str) -> String {
    crate::helpers::read_input(input)
}

pub fn part_1(input: &str) -> i32 {
    let mem = read(input);

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
    let mem = read(input);

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
    fn sample_1() {
        assert_eq!(part_1("03_sample"), 161);
    }

    #[test]
    fn answer_1() {
        assert_eq!(part_1("03"), 165225049);
    }

    #[test]
    fn sample_2() {
        assert_eq!(part_2("03_sample2"), 48);
    }

    #[test]
    fn answer_2() {
        assert_eq!(part_2("03"), 108830766);
    }
}
