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

// program opcodes and operands:

// 2,4
// 1,1
// 7,5
// 1,5
// 4,3
// 0,3
// 5,5
// 3,0

// converted to readable instructions:

// bst 4
// bxl 1
// cdv 5
// bxl 5
// bxc 3
// adv 3
// out 5
// jnz 0

// converted to more high-level logic:

// B = A % 8                            take last 3 bits of A
// B = B XOR 1                          shift B around
// C = A / ( 2 ** B )                   C = some fraction of A, as small as (2**7)th = 128th / some variable number of bits of A
// B = B XOR 5                          shift B around
// B = B XOR C                          shift B around
// A = A / 8                            remove last 3 bits of A
// output B % 8                         output last 3 bits of B
// if A == 0, halt, else restart        continue until A == 0

// So what this program is doing is:
// - take the last 3 bits of A
// - hash them using a larger portion of A into a 3-bit value
// - output that 3-bit value
// - repeat until we've consumed all of A
//
// Because bits are consumed from the end of A, the last bits to be consumed will be the highest
// ones. So to find value of register A that recreates the program as output, we have to start
// constructing the number from the highest 3 bits first. First we find a value that recreates the
// *last* output value. Then we find a value for the second-highest 3-bits that recreates the
// *second-to-last* output value. And so on, 16 times to create the 16 3-bit values of our answer.
// There are values that seem to work initially but break down in the middle, so the process must
// be recursive to explore all potential answers. This is the `find_value` function. Because we are
// starting from the highest bits and iterating on potential values from 0 to 7, in a depth-first
// manner, the first matching value we find is necessarily the smallest of all potential values.

pub fn part_2(input: &str) -> i64 {
    let mut computer = parse(input);
    let cursor_max = computer.program.len();
    find_value(&computer, 1, cursor_max, 0).unwrap()
}

// based on this reddit comment which helped me figure out how to solve this
// https://www.reddit.com/r/adventofcode/comments/1hg38ah/comment/m2gge90/
fn find_value(computer: &Computer, curs: usize, cursor_max: usize, reg: i64) -> Option<i64> {
    // the potential 3-bit value
    for oct in 0..=7 {
        // shift our existing value up 3 bits (*8) and add the potential
        let reg2 = reg * 8 + oct;
        let mut c = computer.clone();
        // set register A and run the program
        c.reg_a = reg2;
        c.run();
        if c.out == computer.program.get((cursor_max-curs)..).unwrap() {
            // if the output matches the last n=curs digits of the program
            if curs == cursor_max {
                // we've found the 16 3-bit values that recreate the program
                return Some(reg2)
            } else {
                // recurse with an incremented cursor and updated register value
                let v = find_value(computer, curs + 1, cursor_max, reg2);
                if v != None { return v }
            }
        }
    }
    None
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
        assert_eq!(part_2(inputs::SAMPLE2), 117440);
    }

    #[test]
    fn part_2_real() {
        assert_eq!(part_2(inputs::REAL), 164541017976509);
    }
}
