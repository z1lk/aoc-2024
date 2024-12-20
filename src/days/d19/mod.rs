use std::collections::HashMap;

pub mod inputs {
    pub const REAL: &str = include_str!("real");
    pub const SAMPLE: &str = include_str!("sample");
}

fn parse(input: &str) -> (Vec<String>, Vec<String>) {
    let mut lines = crate::helpers::to_lines(input);
    let patterns: Vec<String> = lines.remove(0).split(", ").map(|s| s.to_string()).collect();

    lines.remove(0); // remove empty line. remaining `lines` is the designs

    (patterns, lines)
}

pub fn part_1(input: &str) -> i32 {
    let (patterns, designs) = parse(input);
    let mut possible = 0;
    for design in designs {
        if possible_design(&design, 0, &patterns) {
            possible += 1;
        }
    }
    possible
}

// Move through slices of the design, checking if any pattern matches.
// We only need to know if it's possible with any set of patterns,
// so return early if we find one.
fn possible_design(design: &str, cursor: i32, patterns: &Vec<String>) -> bool {
    // reached the end, so it's possible
    if cursor == (design.len() as i32) { return true; }

    for pattern in patterns {
        // range for slicing the design, from cursor + num chars in pattern
        let r = (cursor as usize)..((cursor + pattern.len() as i32) as usize);
        // check if that slice of the design matches the pattern
        if design.get(r) == Some(pattern) {
            // move to next slice of design
            if possible_design(design, cursor + pattern.len() as i32, patterns) {
                return true;
            }
        }
    }
    false
}

// Similar to part 1, except we need to count all possible variants.
// Modified the part 1 solution to sum instead of return a bool.
// This alone will take a long time because there are many variants.
// By memoizing with a HashMap on design/cursor => count, we
// greatly speed up the solution.
pub fn part_2(input: &str) -> i64 {
    let (patterns, designs) = parse(input);
    let mut variants = 0;

    let mut cache: HashMap::<String, i64> = HashMap::new();

    for design in designs {
        let mut possible_pats: Vec<String> = Vec::new();
        for pattern in &patterns {
            if design.contains(&*pattern) {
                possible_pats.push(pattern.clone());
            }
        }
        let (design_vars, c) = num_variants(&design, 0, &possible_pats, cache);
        cache = c;
        variants += design_vars;
    }

    variants
}

fn num_variants(design: &str, cursor: i64, patterns: &Vec<String>, mut cache: HashMap<String, i64>) -> (i64, HashMap<String, i64>) {
    let cache_key = format!("{}:{}", design, cursor);

    // cache hit, return cached value
    if let Some(v) = cache.get(&cache_key) { return (*v, cache) }

    // reached end
    if cursor == (design.len() as i64) { return (1, cache) }

    // no cache, we'll count the variants
    let mut num = 0;

    for pattern in patterns {
        let r = (cursor as usize)..((cursor + pattern.len() as i64) as usize);
        if design.get(r.clone()) == Some(pattern) {
            let (n, c) = num_variants(design, cursor + pattern.len() as i64, patterns, cache);
            cache = c;
            num += n;
        }
    }

    // write to the cache
    cache.insert(cache_key, num);

    (num, cache)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(part_1(inputs::SAMPLE), 6);
    }

    #[test]
    fn part_1_real() {
        assert_eq!(part_1(inputs::REAL), 220);
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(part_2(inputs::SAMPLE), 16);
    }

    #[test]
    fn part_2_real() {
        assert_eq!(part_2(inputs::REAL), 565600047715343);
    }
}
