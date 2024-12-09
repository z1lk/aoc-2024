pub mod inputs {
    pub const REAL: &str = include_str!("real");
    pub const SAMPLE: &str = include_str!("sample");
}

fn parse(input: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let lines = crate::helpers::to_lines(input);
    let mut rules: Vec<(i32, i32)> = Vec::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    for line in lines {
        if (line.contains("|")) {
            let nums: Vec<i32> = line.split("|").map(|n| n.parse::<i32>().unwrap_or(0)).collect();
            rules.push((nums[0], nums[1]));
        } else if (line.contains(",")) {
            updates.push(
                line.split(",").map(|n| n.parse::<i32>().unwrap_or(0)).collect()
            );
        }
    }

    (rules, updates)
}

pub fn part_1(input: &str) -> i32 {
    let (rules, updates) = parse(input);
    let mut sum = 0;
    for update in updates {
        if is_good(&rules, &update) {
            sum += get_middle(&update);
        }
    }
    sum
}

pub fn part_2(input: &str) -> i32 {
    let (rules, updates) = parse(input);

    let mut sum = 0;
    for update in updates {
        if !is_good(&rules, &update) {
            let update = sort_by_rules(&rules, update);
            sum += get_middle(&update);
        }
    }
    sum
}

fn is_good(rules: &Vec<(i32, i32)>, update: &Vec<i32>) -> bool {
    let mut good = true;
    for (i, num) in update.iter().enumerate() {
        for other in &update[(i+1)..] {
            if rules.iter().any(|rule| rule.0 == *other && rule.1 == *num) {
                good = false;
            }
        }
    }
    good
}

fn get_middle(update: &Vec<i32>) -> i32 {
    let mid_i = (update.len() - 1) / 2;
    update[mid_i]
}

fn sort_by_rules(rules: &Vec<(i32, i32)>, mut update: Vec<i32>) -> Vec<i32> {
    update.sort_by(|a, b| {
        // there is a rule for every pair of numbers, so find the one for these two
        let rule = rules.iter().find(|r| (r.0 == *a && r.1 == *b) || (r.0 == *b && r.1 == *a) ).unwrap();
        // if a is first, then sort a < b, else reverse
        if rule.0 == *a {
            return 0.cmp(&1);
        } else {
            return 1.cmp(&0);
        }
    });
    update
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(part_1(inputs::SAMPLE), 143);
    }

    #[test]
    fn part_1_real() {
        assert_eq!(part_1(inputs::REAL), 5762);
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(part_2(inputs::SAMPLE), 123);
    }

    #[test]
    fn part_2_real() {
        assert_eq!(part_2(inputs::REAL), 4130);
    }
}
