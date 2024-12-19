pub mod inputs {
    pub const REAL: &str = include_str!("real");
    pub const SAMPLE: &str = include_str!("sample");
}

fn parse(input: &str) -> (Vec<String>, Vec<String>) {
    let mut lines = crate::helpers::to_lines(input);

    let patterns: Vec<String> = lines.remove(0).split(", ").map(|s| s.to_string()).collect();

    lines.remove(0); // empty line

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

fn possible_design(design: &str, cursor: i32, patterns: &Vec<String>) -> bool {
    if cursor == (design.len() as i32) {
        return true;
    }

    for pattern in patterns {
        let r = (cursor as usize)..((cursor + pattern.len() as i32) as usize);
        if design.get(r.clone()) == Some(pattern) {
            if possible_design(design, cursor + pattern.len() as i32, patterns) {
                return true;
            }
        }
    }

    false
}

pub fn part_2(input: &str) -> i32 {
    let (patterns, designs) = parse(input);
    //println!("{:?}", patterns);
    //println!("{:?}", designs);

    let mut variants = 0;

    for design in designs {
        println!("{:?}", design);

        let mut possible_pats: Vec<String> = Vec::new();
        for pattern in &patterns {
            if design.contains(&*pattern) {
                possible_pats.push(pattern.clone());
            }
        }

        println!("{:?} possible pats", possible_pats.len());

        let design_vars = num_variants(&design, 0, &possible_pats, 1);
        println!("\n{:?}", design_vars);
        variants += design_vars;
    }

    variants
}

fn num_variants(design: &str, cursor: i32, patterns: &Vec<String>, depth: i32) -> i32 {
    if cursor == (design.len() as i32) {
        return 1;
    }

    let mut num = 0;

    for pattern in patterns {
        let r = (cursor as usize)..((cursor + pattern.len() as i32) as usize);
        if design.get(r.clone()) == Some(pattern) {
            //println!("\n");
            //for _ in 1..=depth { print!(" "); }
            //print!("{:?}", pattern);
            num += num_variants(design, cursor + pattern.len() as i32, patterns, depth+1);
        }
    }

    num
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
        //assert_eq!(part_2(inputs::REAL), 0);
    }
}
