fn read(input: &str) -> (Vec<i32>, Vec<i32>) {
    let lines = crate::helpers::read_input(input);

    let mut ids1: Vec<i32> = Vec::new();
    let mut ids2: Vec<i32> = Vec::new();

    for line in lines {
        let items: Vec<&str> = line.split("   ").collect();
        ids1.push(items[0].parse::<i32>().unwrap_or(0));
        ids2.push(items[1].parse::<i32>().unwrap_or(0));
    }

    (ids1, ids2)
}

pub fn part_1(input: &str) -> i32 {
    let (mut ids1, mut ids2) = read(input);

    ids1.sort();
    ids2.sort();

    ids1.iter()
        .zip(ids2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

pub fn part_2(input: &str) -> i32 {
    let (ids1, ids2) = read(input);
    
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
    fn sample_1() {
        assert_eq!(part_1("01_sample"), 11);
    }

    #[test]
    fn sample_2() {
        assert_eq!(part_2("01_sample"), 31);
    }

    #[test]
    fn answer_1() {
        assert_eq!(part_1("01"), 1580061);
    }

    #[test]
    fn answer_2() {
        assert_eq!(part_2("01"), 23046913);
    }
}
