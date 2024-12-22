use std::collections::HashMap;

pub mod inputs {
    pub const REAL: &str = include_str!("real");
    pub const SAMPLE: &str = include_str!("sample");
    pub const SAMPLE2: &str = include_str!("sample2");
}

fn parse(input: &str) -> Vec<i64> {
    crate::helpers::to_lines(input).iter().map(|s| s.parse::<i64>().unwrap()).collect()
}

pub fn part_1(input: &str) -> i64 {
    let starting_secret = parse(input);
    let mut sum = 0;
    for starting_secret in starting_secret {
        let mut secret = starting_secret;
        for _ in 0..2000 {
            secret = evolve(secret);
        }
        sum += secret;
    }
    sum
}

type Diffs = HashMap::<[i64; 4], i64>;

pub fn part_2(input: &str) -> i64 {
    let starting_secrets = parse(input);
    let mut caches: Vec<Diffs> = Vec::new();

    // For each secret, find the diffs and first price at each 4-diff sequence
    for starting_secret in starting_secrets {
        let mut cache: Diffs = HashMap::new();
        let mut prices: Vec<i64> = Vec::new();
        let mut diffs: Vec<i64> = Vec::new();
        let mut secret = starting_secret;

        let digit_char = secret.to_string().chars().last().unwrap();
        let price = digit_char.to_digit(10).unwrap() as i64;
        prices.push(price);

        for i in 0..2000 {
            secret = evolve(secret);
            let digit_char = secret.to_string().chars().last().unwrap();
            let price = digit_char.to_digit(10).unwrap() as i64;
            let diff = price - prices.last().unwrap();
            diffs.push(diff);
            if i >= 3 {
                let r = (diffs.len() - 4)..;
                let last_4_diffs: [i64; 4] = diffs.get(r).unwrap().try_into().unwrap();
                cache.entry(last_4_diffs).or_insert(price);
            }
            prices.push(price);
        }

        caches.push(cache);
    }

    // Find the unique diff sequences across all secrets
    let mut diffs: Vec<[i64; 4]> = Vec::new();
    for cache in &caches {
        for (k,v) in cache {
            if (diffs.contains(&k)) { continue }
            diffs.push(*k);
        }
    }

    // For each unique diff sequences, calculate the total
    // bananas we'd get if we sold to all sellers on that diff.
    // Highest number is itself our answer.
    let mut best = 0;
    for diff in diffs {
        let mut bananas = 0;
        for cache in &caches {
            bananas += cache.get(&diff).unwrap_or(&0);
        }
        if bananas > best { best = bananas }
    }

    best
}

fn evolve(mut secret: i64) -> i64 {
    secret = prune(mix(secret, secret * 64));
    secret = prune(mix(secret, secret / 32));
    secret = prune(mix(secret, secret * 2048));
    secret
}

fn mix(secret: i64, n: i64) -> i64 {
    secret ^ n
}

fn prune(secret: i64) -> i64 {
    secret % 16777216
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(part_1(inputs::SAMPLE), 37327623);
    }

    #[test]
    fn part_1_real() {
        assert_eq!(part_1(inputs::REAL), 15303617151);
    }

    #[test]
    fn part_2_sample2() {
        assert_eq!(part_2(inputs::SAMPLE2), 23);
    }

    #[test]
    fn part_2_real() {
        // takes a minute on a release build
        //assert_eq!(part_2(inputs::REAL), 1727);
    }
}
