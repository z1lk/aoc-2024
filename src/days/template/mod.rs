pub mod inputs {
    pub const REAL: &str = include_str!("real");
    pub const SAMPLE: &str = include_str!("sample");
}

fn parse(input: &str) -> parsed_type {
    let lines = crate::helpers::to_lines(input);
    let parsed = // parse...
    parsed
}

pub fn part_1(input: &str) -> i32 {
    let parsed = parse(input);
    0
}

pub fn part_2(input: &str) -> i32 {
    let parsed = parse(input);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(part_1(inputs::SAMPLE), 0);
    }

    #[test]
    fn part_1_real() {
        assert_eq!(part_1(inputs::REAL), 0);
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(part_2(inputs::SAMPLE), 0);
    }

    #[test]
    fn part_2_real() {
        assert_eq!(part_2(inputs::REAL), 0);
    }
}
