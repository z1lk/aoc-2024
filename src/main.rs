mod helpers;
mod grid;
mod days;

use days::d15 as day;

fn main() {
    //println!("{}", day::part_1(day::inputs::SAMPLE2));
    //println!("{}", day::part_1(day::inputs::REAL));
    println!("{}", day::part_2(day::inputs::SAMPLE2));
    println!("{}", day::part_2(day::inputs::REAL));
}
