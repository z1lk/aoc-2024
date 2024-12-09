pub mod inputs {
    pub const REAL: &str = include_str!("real");
    pub const SAMPLE: &str = include_str!("sample");
}

fn parse(input: &str) -> Vec<(i64, Vec<i64>)> {
    crate::helpers::to_lines(input)
        .iter()
        .map(|l| {
            let split: Vec<&str> = l.split(": ").collect();
            let test = split[0];
            let nums: Vec<&str> = split[1].split(" ").collect();
            (
                test.parse::<i64>().unwrap(),
                nums.iter().map(|n| n.parse::<i64>().unwrap()).collect()
            )
        })
        .collect()
}

pub fn part_1(input: &str) -> i64 {
    let eqs = parse(input);
    let mut cal_res = 0;
    for eq in eqs {
        if can_eval_true(eq.clone(), false) { cal_res += eq.0 }
    }
    cal_res
}

pub fn part_2(input: &str) -> i64 {
    let eqs = parse(input);
    let mut cal_res = 0;
    for eq in eqs {
        if can_eval_true(eq.clone(), true) { cal_res += eq.0 }
    }
    cal_res
}

enum Op { Add, Mul, Join }

fn can_eval_true(eq: (i64, Vec<i64>), do_join: bool) -> bool {
    let (test, nums) = eq;
    if can_eval_true_op(test, nums.clone(), Op::Add, do_join) { return true; }
    if can_eval_true_op(test, nums.clone(), Op::Mul, do_join) { return true; }
    if do_join {
        if can_eval_true_op(test, nums.clone(), Op::Join, do_join) { return true; }
    }
    false
}

fn can_eval_true_op(test: i64, mut nums: Vec<i64>, op: Op, do_join: bool) -> bool {
    if nums.len() == 0 { panic!("nums is empty!") }
    if nums.len() == 1 { return test == nums[0] }

    let a = nums.remove(0);
    let b = nums.remove(0);

    match op {
        Op::Add => { nums.insert(0, a + b) }
        Op::Mul => { nums.insert(0, a * b) }
        Op::Join => {
            let joined_s = a.to_string() + &b.to_string();
            let joined = joined_s.parse::<i64>().unwrap();

            nums.insert( 0, joined );
        }
    };
    if can_eval_true_op(test, nums.clone(), Op::Add, do_join) { return true; }
    if can_eval_true_op(test, nums.clone(), Op::Mul, do_join) { return true; }
    if do_join {
        if can_eval_true_op(test, nums.clone(), Op::Join, do_join) { return true; }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(part_1(inputs::SAMPLE), 3749);
    }

    #[test]
    fn part_1_real() {
        assert_eq!(part_1(inputs::REAL), 5030892084481);
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(part_2(inputs::SAMPLE), 11387);
    }

    #[test]
    fn part_2_real() {
        // takes 5-10 secs
        //assert_eq!(part_2(inputs::REAL), 91377448644679);
    }
}
