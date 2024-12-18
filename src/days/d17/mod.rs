use itertools::Itertools;
use std::{thread, time};

pub mod inputs {
    pub const REAL: &str = include_str!("real");
    pub const SAMPLE: &str = include_str!("sample");
    pub const SAMPLE2: &str = include_str!("sample2");
    pub const SMALL: &str = include_str!("small");
    pub const SMALL2: &str = include_str!("small2");
    pub const SMALL3: &str = include_str!("small3");
    pub const SMALL4: &str = include_str!("small4");
    pub const SMALL5: &str = include_str!("small5");
}

#[derive(Clone,Debug)]
struct Computer {
    pub reg_a: i64,
    reg_b: i64,
    reg_c: i64,
    pub program: Vec<i64>,
    ptr: i64,
    pub out: Vec<i64>,
}

impl Computer {
    pub fn new(reg_a: i64, reg_b: i64, reg_c: i64, program: Vec<i64>) -> Self {
        Self {
            reg_a,
            reg_b,
            reg_c,
            program,
            ptr: 0,
            out: Vec::new()
        }
    }

    pub fn run(&mut self) {
        loop {
            match self.program.get(self.ptr as usize) {
                Some(opcode) => {
                    let operand = self.program.get((self.ptr + 1) as usize).unwrap();
                    match opcode {
                        0 => self.adv(*operand),
                        1 => self.bxl(*operand),
                        2 => self.bst(*operand),
                        3 => self.jnz(*operand),
                        4 => self.bxc(*operand),
                        5 => self.out(*operand),
                        6 => self.bdv(*operand),
                        7 => self.cdv(*operand),
                        _ => panic!("unexpected opcode {:?}!", opcode)
                    }
                }
                None => break // halt
            }
        }
    }

    fn move_ptr(&mut self) { self.ptr += 2; }

    fn adv(&mut self, operand: i64) {
        self.reg_a = self.reg_a / 2_i64.pow(self.combo(operand).try_into().unwrap());
        self.move_ptr();
    }

    fn bxl(&mut self, operand: i64) {
        self.reg_b = self.reg_b ^ operand;
        self.move_ptr();
    }

    fn bst(&mut self, operand: i64) {
        self.reg_b = self.combo(operand) % 8;
        self.move_ptr();
    }

    fn jnz(&mut self, operand: i64) {
        if self.reg_a == 0 {
            self.move_ptr();
        } else {
            self.ptr = operand;
        }
    }

    fn bxc(&mut self, _operand: i64) {
        self.reg_b = self.reg_b ^ self.reg_c;
        self.move_ptr();
    }

    fn out(&mut self, operand: i64) {
        self.out.push(self.combo(operand) % 8);
        self.move_ptr();
    }

    fn bdv(&mut self, operand: i64) {
        self.reg_b = self.reg_a / 2_i64.pow(self.combo(operand).try_into().unwrap());
        self.move_ptr();
    }

    fn cdv(&mut self, operand: i64) {
        self.reg_c = self.reg_a / 2_i64.pow(self.combo(operand).try_into().unwrap());
        self.move_ptr();
    }

    fn combo(&self, operand: i64) -> i64 {
        match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            // "Combo operand 7 is reserved and will not appear in valid programs."
            7 => panic!("invalid combo operand 7!"),
            _ => panic!("unexpected combo operand {:?}!", operand)
        }
    }

    fn output(&self) -> String {
        self.out.iter().map(|&n| n.to_string()).join(",")
    }
}

fn parse(input: &str) -> Computer {
    let lines = crate::helpers::to_lines(input);

    let reg_a: i64 = lines[0].split(": ").collect::<Vec<&str>>()[1].parse().unwrap();
    let reg_b: i64 = lines[1].split(": ").collect::<Vec<&str>>()[1].parse().unwrap();
    let reg_c: i64 = lines[2].split(": ").collect::<Vec<&str>>()[1].parse().unwrap();
    let program = lines[4].split(": ").collect::<Vec<&str>>()[1]
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    Computer::new(reg_a, reg_b, reg_c, program)
}

pub fn part_1(input: &str) -> String {
    let mut computer = parse(input);
    //println!("{:?}",computer);
    computer.run();
    //println!("{:?}",computer);
    computer.output()
}

pub fn part_2(input: &str) -> i64 {
    //let computer_output = part_1(input);

    let mut computer = parse(input);
    //let mut i = 50_000_000_000_000;
    //let mut i = 0;
    let mut i = 35184372088832;

    loop {
        thread::sleep(time::Duration::from_millis(100));
        println!("{:?}", i);
        let mut computer2 = computer.clone();
        computer2.reg_a = i;
        computer2.run();
        let output = computer2.output();
        println!("{:?}", output);
        if computer2.out == computer.program {
            return i;
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(part_1(inputs::SAMPLE), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn part_1_real() {
        assert_eq!(part_1(inputs::REAL), "7,6,1,5,3,1,4,2,6");
    }

    #[test]
    fn part_2_sample() {
        //assert_eq!(part_2(inputs::SAMPLE), 0);
    }

    #[test]
    fn part_2_real() {
        // assert_eq!(part_2(inputs::REAL), 0);
    }
}
