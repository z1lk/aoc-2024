use std::fs::read_to_string;

// https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
pub fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

pub fn read_input(name: &str) -> Vec<String> {
    read_lines(&("src/inputs/".to_owned() + name))
}
