fn read(input: &str) -> Vec<Vec<char>> {
    let lines = crate::helpers::read_input_to_lines(input);
    lines.iter().map(|line| line.chars().collect()).collect()
}

pub fn part_1(input: &str) -> i32 {
    0
}

pub fn part_2(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_1() {
        assert_eq!(part_1("06_sample"), 0);
    }

    #[test]
    fn answer_1() {
        assert_eq!(part_1("06"), 0);
    }

    #[test]
    fn sample_2() {
        assert_eq!(part_2("06_sample"), 0);
    }

    #[test]
    fn answer_2() {
        assert_eq!(part_2("06"), 0);
    }
}
