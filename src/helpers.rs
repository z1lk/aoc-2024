// https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
pub fn to_lines(file: &str) -> Vec<String> {
    file.lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}
