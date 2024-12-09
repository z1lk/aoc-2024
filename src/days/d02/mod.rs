use itertools::Itertools;

pub mod inputs {
    pub const REAL: &str = include_str!("real");
    pub const SAMPLE: &str = include_str!("sample");
}

fn parse(input: &str) -> Vec<Vec<i32>> {
    let lines = crate::helpers::to_lines(input);
    lines.iter().map(|l| {
        l.split(" ").map(|n| n.parse::<i32>().unwrap_or(0)).collect()
    }).collect()
}

pub fn part_1(input: &str) -> i32 {
    let reports = parse(input);
    let mut safe = 0;
    for report in reports {
        if report_safe(&report) { safe += 1 }
    }
    safe
}

pub fn part_2(input: &str) -> i32 {
    let reports = parse(input);
    let mut safe = 0;
    for report in reports {
        if report_safe(&report) {
            safe += 1;
        } else {
            for (i, n) in report.iter().enumerate() {
                let mut clone = report.clone();
                clone.remove(i);
                if report_safe(&clone) {
                    safe += 1;
                    break;
                }
            }
        }
    }
    safe
}

fn report_safe(report: &Vec<i32>) -> bool {
    let diffs: Vec<i32> = report.iter().tuple_windows().map(|(a,b)| a - b).collect();
    let diff_pos = diffs.iter().all(|n| *n > 0);
    let diff_neg = diffs.iter().all(|n| *n < 0);
    if diff_pos || diff_neg {
        return diffs.iter().all(|n| {
            let abs = n.abs();
            abs >= 1 && abs <= 3
        });
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(part_1(inputs::SAMPLE), 2);
    }

    #[test]
    fn part_1_real() {
        assert_eq!(part_1(inputs::REAL), 463);
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(part_2(inputs::SAMPLE), 4);
    }

    #[test]
    fn part_2_real() {
        assert_eq!(part_2(inputs::REAL), 514);
    }
}
