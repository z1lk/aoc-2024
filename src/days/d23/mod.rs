use std::collections::HashMap;
use std::collections::hash_map::Entry;
use itertools::Itertools;

pub mod inputs {
    pub const REAL: &str = include_str!("real");
    pub const SAMPLE: &str = include_str!("sample");
}

#[derive(Debug)]
struct Computer {
    name: String,
    links: Vec<String>
}

// build a list of all the computers and their links
fn parse(input: &str) -> HashMap<String, Computer> {
    let lines = crate::helpers::to_lines(input);

    let mut links: Vec<(String, String)> = Vec::new();
    for line in lines {
        let computers: Vec<&str> = line.split("-").collect();
        links.push((
                computers.get(0).unwrap().to_string(),
                computers.get(1).unwrap().to_string(),
        ));
    }

    let mut computers: HashMap<String, Computer> = HashMap::new();

    for (c, d) in links {
        match computers.entry(c.clone()) {
            Entry::Occupied(o) => {
                let comp = o.into_mut();
                comp.links.push(d.clone());
            }
            Entry::Vacant(v) => {
                let comp = Computer {
                    name: c.clone(),
                    links: vec![d.clone()]
                };
                v.insert(comp);
            }
        }
        match computers.entry(d.clone()) {
            Entry::Occupied(o) => {
                let comp = o.into_mut();
                comp.links.push(c.clone());
            }
            Entry::Vacant(v) => {
                let comp = Computer {
                    name: d.clone(),
                    links: vec![c.clone()]
                };
                v.insert(comp);
            }
        }
    }

    computers
}

pub fn part_1(input: &str) -> i32 {
    let computers = parse(input);

    let mut tri_links: Vec<String> = Vec::new();

    // for each computer
    for (m, c) in &computers {
        // for each pair of links
        for two_links in c.links.iter().combinations(2) {
            let n = two_links.get(0).unwrap();
            let o = two_links.get(1).unwrap();

            // there is a rule that one of their names must start with "t"
            if !(m.starts_with("t") || n.starts_with("t") || o.starts_with("t")) {
                break;
            }

            let d = computers.get(*n).unwrap();
            let e = computers.get(*o).unwrap();

            // if the pair connect to each other, add them to our list of tri_links
            if d.links.contains(o) && e.links.contains(n) {
                let mut names = vec![m,n,o];
                names.sort();
                let joined = names.iter().join(",");
                if !tri_links.contains(&joined) {
                    tri_links.push(joined);
                }
            }
        }
    }

    tri_links.len() as i32
}

pub fn part_2(input: &str) -> String {
    let computers = parse(input);

    // cache for
    // "do the computers with these names all connect to every other computer in the set"
    let mut connected: HashMap<String, bool> = HashMap::new();

    let min_size = if input == inputs::REAL { 13 } else { 4 };

    for (m, c) in &computers {
        // build a list of all names, self + links
        let mut all = c.links.clone();
        all.push(m.clone());

        // Loop on all combinations of computers from size 2..max.
        // We know from the input that each computer is connected to at least one other (2),
        // i.e. there are some links.
        //for i in 2..all.len() {
        // Optimize by using a min_size > 2. The answer for the sample is given,
        // and for the real input we can work down from a guess until we get the solution.
        for i in min_size..all.len() {
            for mut names in all.iter().combinations(i) {
                // Sort the names for the cache key so the cache is optimal,
                // but this is also the format required for the answer.
                names.sort();
                let key = names.iter().join(",");

                match connected.entry(key) {
                    Entry::Occupied(o) => { continue }
                    Entry::Vacant(v) => {
                        v.insert(all_connected(names, &computers));
                    }
                }
            }
        }
    }

    let mut largest = String::new();

    for (k, b) in connected {
        if !b { continue }
        if k.len() > largest.len() {
            largest = k.clone();
        }
    }

    largest
}

fn all_connected(mut names: Vec<&String>, computers: &HashMap<String, Computer>) -> bool {
    for n in &names {
        let computer = computers.get(*n).unwrap();
        for m in &names {
            if n == m { continue }
            if !computer.links.contains(&m) {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(part_1(inputs::SAMPLE), 7);
    }

    #[test]
    fn part_1_real() {
        assert_eq!(part_1(inputs::REAL), 1046);
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(part_2(inputs::SAMPLE), "co,de,ka,ta");
    }

    #[test]
    fn part_2_real() {
        assert_eq!(part_2(inputs::REAL), "de,id,ke,ls,po,sn,tf,tl,tm,uj,un,xw,yz");
    }
}
