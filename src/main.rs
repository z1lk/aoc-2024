mod helpers;
mod grid;
mod days;

use days::d20 as day;

fn main() {
    println!("{}", day::part_1(day::inputs::SAMPLE));
    //println!("{}", day::part_1(day::inputs::REAL));
    //println!("{}", day::part_2(day::inputs::SAMPLE));
    //println!("{}", day::part_2(day::inputs::REAL));
}
