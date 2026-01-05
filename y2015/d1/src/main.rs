use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    // part 1
    let ups = input.chars().filter(|c| *c == '(').count();
    let downs = input.chars().filter(|c| *c == ')').count();

    println!("{}", ups - downs);

    // part 2
    let mut acc = 0i64;
    for (i, c) in input.char_indices() {
        acc = match c {
            '(' => acc + 1,
            ')' => acc - 1,
            _ => acc,
        };
        if acc < 0 {
            println!("{}", i + 1);
            break;
        };
    }
}
