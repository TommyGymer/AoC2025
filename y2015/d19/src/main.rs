#[derive(Debug)]
struct Rule {
    from: String,
    to: String,
}

fn main() {
    let input: String = std::fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .to_owned();

    let lines: Vec<&str> = input.split("\n").collect();

    let mut medcine: String = String::from("");
    let mut rules: Vec<Rule> = vec![];

    let mut found_empty = false;
    for line in lines {
        if found_empty {
            medcine = line.trim().to_owned();
        } else if line.len() == 0 {
            found_empty = true;
        } else {
            let mut bits = line.split(" => ");
            rules.push(Rule {
                from: bits.next().unwrap().to_owned(),
                to: bits.next().unwrap().to_owned(),
            })
        }
    }

    println!("Rules: {:#?}", rules);
    println!("Medicine: {}", medcine);
}
