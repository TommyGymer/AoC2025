use std::fs;

fn main() {
    let input: String = fs::read_to_string("input.txt").unwrap().trim().to_owned();
}
