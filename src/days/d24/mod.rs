use std::str::FromStr;
use std::collections::HashMap;
use regex::Regex;

pub mod inputs {
    pub const REAL: &str = include_str!("real");
    pub const SAMPLE: &str = include_str!("sample");
    pub const SAMPLE2: &str = include_str!("sample2");
    pub const SAMPLE3: &str = include_str!("sample3");
}

#[derive(Clone,Debug)]
struct Gate {
    name: String,
    value: i32,
    ready: bool,
    input1: Option<String>,
    input2: Option<String>,
    operator: Op
}

#[derive(Clone,Debug,PartialEq)]
enum Op { And, Or, Xor, None }

type Gates = HashMap<String, Gate>;

fn parse(input: &str) -> Gates {
    let lines = crate::helpers::to_lines(input);

    let mut gates: Gates = HashMap::new();

    let start_re = Regex::new(r"(.+): (\d)").unwrap();
    let conns_re = Regex::new(r"(.+) (AND|OR|XOR) (.+) -> (.+)").unwrap();

    let mut done_start = false;

    for line in &lines {
        if !done_start {
            if line == "" {
                done_start = true;
            } else {
                let caps = start_re.captures(&line).unwrap();
                let name = caps.get(1).unwrap().as_str().to_string();
                let value: i32 = FromStr::from_str(caps.get(2).unwrap().as_str()).unwrap();
                let gate = Gate {
                    name: name.clone(),
                    value,
                    ready: true,
                    input1: None,
                    input2: None,
                    operator: Op::None
                };
                gates.insert( name, gate );
            }
            continue;
        }

        let caps = conns_re.captures(&line).unwrap();
        let input1 = caps.get(1).unwrap().as_str().to_string();
        let operator_str = caps.get(2).unwrap().as_str();
        let operator: Op = match operator_str {
            "AND" => Op::And,
            "OR" => Op::Or,
            "XOR" => Op::Xor,
            _ => panic!("unexpected operator {}", operator_str)
        };

        let input2 = caps.get(3).unwrap().as_str().to_string();
        let name = caps.get(4).unwrap().as_str().to_string();
        let gate = Gate {
            name: name.clone(),
            value: 0,
            ready: false,
            input1: Some(input1),
            input2: Some(input2),
            operator
        };
        gates.insert( name, gate );
    }

    gates
}

pub fn part_1(input: &str) -> i64 {
    let mut gates = parse(input);
    run(&mut gates);
    get_z(&gates)
}

// Identified swapped wires by manually inspecting the input's wiring.
// dkr,ggk,hhh,htp,rhv,z05,z15,z20
// The function body does the swapping and asserts that it works.
pub fn part_2(input: &str) {
    let mut gates = parse(input);
    // y05 AND x05 -> z05    carry - WRONG - should be -> dkr
    // gcs XOR hdc -> dkr    sum out - WRONG - should be -> z05
    // bhw XOR sth -> htp    sum out - WRONG - should be -> z15
    // sth AND bhw -> z15    sum overflow - WRONG - should be -> htp
    // mvv XOR fvm -> hhh    sum out - WRONG - should be -> z20
    // qfj OR mqg -> z20     carry out - WRONG - should be -> hhh
    // y36 XOR x36 -> rhv      sum - WRONG - should be -> ggk
    // x36 AND y36 -> ggk      carry - WRONG - should be -> rhv
    let swaps = [ ("z05", "dkr"), ("htp", "z15"), ("hhh", "z20"), ("rhv", "ggk") ];

    // the expected sum from the initial x/y values
    let sum = get_x(&gates) + get_y(&gates);

    // run mis-wired adder
    run(&mut gates);
    assert_ne!(get_z(&gates), sum);

    // swap the wires
    for (m,n) in swaps {
        let mut a = gates.get(m).unwrap().clone();
        let mut b = gates.get(n).unwrap().clone();

        a.name = n.to_string();
        gates.insert(n.to_string(), a);

        b.name = m.to_string();
        gates.insert(m.to_string(), b);
    }

    // run properly wired adder
    reset(&mut gates);
    run(&mut gates);
    assert_eq!(get_z(&gates), sum);
}

fn run(mut gates: &mut Gates) {
    let mut names: Vec<String> = Vec::new();
    for (name, gate) in &mut *gates {
        names.push(name.clone());
    }
    // not sure why this uses a borrow making get_mut not work
    //let names = gates.keys().clone();

    loop {
        let mut todo = 0;
        for name in &names {
            let gate = gates.get(name).unwrap();
            if gate.operator != Op::None {
                let input1 = gates.get(&gate.input1.clone().unwrap()).unwrap();
                let input2 = gates.get(&gate.input2.clone().unwrap()).unwrap();
                if !input1.ready || !input2.ready {
                    todo += 1;
                    continue;
                }

                let input1val = input1.value.clone();
                let input2val = input2.value.clone();

                let val = match gate.operator {
                    Op::And => input1val & input2val,
                    Op::Or => input1val | input2val,
                    Op::Xor => input1val ^ input2val,
                    Op::None => panic!("unexpected None operator")
                };

                let gate = gates.get_mut(name).unwrap();
                gate.value = val;
                gate.ready = true;
            }
        }
        //println!("todo: {:?}", todo);
        if todo == 0 { break }
    }
}

// get the Nth bit (from right) in the binary rep of num
fn get_bit(num: i64, place: i32) -> i32 {
    // ( num % (2 ** p+1) ) / (2 ** p)
    let numer: i64 = (num % 2_i64.pow((place+1) as u32)).try_into().unwrap();
    let denom: i64 = 2_i64.pow(place as u32).try_into().unwrap();
    (numer / denom).try_into().unwrap()
}

fn set_x(mut gates: &mut Gates, num: i64) {
    for (name, gate) in gates {
        if name.starts_with("x") {
            //println!("setting {}", name);
            let id = name.get(1..).unwrap().parse::<i32>().unwrap();
            //println!("id={:?}", id);
            let bit = get_bit(num, id);
            //println!("bit={:?}", bit);
            gate.value = bit;
        }
    }
}

fn set_y(mut gates: &mut Gates, num: i64) {
    for (name, gate) in gates {
        if name.starts_with("y") {
            let id = name.get(1..).unwrap().parse::<i32>().unwrap();
            gate.value = get_bit(num, id);
        }
    }
}

fn get_x(gates: &Gates) -> i64 {
    let mut num = 0_i64;
    for (name, gate) in gates {
        if name.starts_with("x") {
            //println!("{}", name);
            if gate.value != 0 {
                let id = name.get(1..).unwrap().parse::<i32>().unwrap();
                num += 2_i64.pow(id.try_into().unwrap());
            }
        }
    }
    num
}

fn get_y(gates: &Gates) -> i64 {
    let mut num = 0_i64;
    for (name, gate) in gates {
        if name.starts_with("y") {
            //println!("{}", name);
            if gate.value != 0 {
                let id = name.get(1..).unwrap().parse::<i32>().unwrap();
                num += 2_i64.pow(id.try_into().unwrap());
            }
        }
    }
    num
}

fn get_z(gates: &Gates) -> i64 {
    let mut num = 0_i64;
    for (name, gate) in gates {
        if name.starts_with("z") {
            //println!("{}", name);
            if gate.value != 0 {
                let id = name.get(1..).unwrap().parse::<i32>().unwrap();
                num += 2_i64.pow(id.try_into().unwrap());
            }
        }
    }
    num
}

// reset the ready flags
fn reset(mut gates: &mut Gates) {
    for (name, gate) in gates {
        gate.ready = (gate.operator == Op::None);
    }
}

/*
pub fn get_bad_bits(n: i64, m: i64) -> Vec<i32> {
    let mut bits: Vec<i32> = Vec::new();
    //for i in 0..6 {
    for i in 0..46 {
        if get_bit(n, i) != get_bit(m, i) {
            bits.push(i);
        }
    }
    bits
}
*/

/*
fn get_all_input_gates(gates: &Gates, name: String) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    let gate = gates.get(&name).unwrap();
    match gate.operator {
        Op::None => return names,
        _ => {
            let input1 = gate.input1.clone().unwrap();
            let input2 = gate.input2.clone().unwrap();
            //names.push(input1.clone());
            //names.push(input2.clone());
            names.push(name);
            return [
                names,
                get_all_input_gates(gates, input1),
                get_all_input_gates(gates, input2)
            ].concat();
        }
    }
}
*/

/*
pub fn part_2_id_bad_bits(input: &str) -> i32 {
    let mut gates = parse(input);

    // testing that setting/getting works
    /*set_x(&mut gates, 0);
    println!("{:?}", get_x(&gates));
    set_x(&mut gates, 31);
    println!("{:?}", get_x(&gates));
    set_x(&mut gates, 15);
    println!("{:?}", get_x(&gates));*/

    let mut bad = 0;

    let mut bad_bits: Vec<i32> = Vec::new();
    
    // 8796093022208

    for _ in 0..20 {
        for _ in 0..20 {
            set_x(&mut gates, x);
            set_y(&mut gates, y);
            run(&mut gates);
            let z = get_z(&gates);
            reset(&mut gates);

            //println!("x={:?} y={:?} z={:?}", x, y, z);
            let expected = x + y;
            //let expected = x & y;
            if z != expected {
                //println!("bad!");
                for i in get_bad_bits(z, expected) {
                    if !bad_bits.contains(&i) {
                        bad_bits.push(i);
                    }
                }
                bad += 1;
            }
        }
    }


    println!("---");
    println!("{:?}", bad_bits);

    let mut z_names = vec![];
    for b in &bad_bits {
        let z = format!("z{:02}", b);
        z_names.push(z);
    }
    //println!("{:?}", z_names);

    let mut potential_gates = vec![];
    for name in z_names {
        potential_gates = [
            potential_gates,
            get_all_input_gates(&gates, name)
        ].concat();
    }
    potential_gates.sort();
    potential_gates.dedup();

    println!("{:?}", potential_gates);

    bad
    0
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(part_1(inputs::SAMPLE), 4);
    }

    #[test]
    fn part_1_sample2() {
        assert_eq!(part_1(inputs::SAMPLE2), 2024);
    }

    #[test]
    fn part_1_real() {
        assert_eq!(part_1(inputs::REAL), 53755311654662);
    }

    /*#[test]
    fn part_2_sample() {
        assert_eq!(part_2(inputs::SAMPLE), 0);
    }*/

    #[test]
    fn part_2_real() {
        // assertions in part_2 verify that swapped inputs make the adder function properly
        part_2(inputs::REAL);
        //assert_eq!(part_2(inputs::REAL), 0);
    }
}
