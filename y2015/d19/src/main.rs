use std::collections::{HashMap, HashSet};

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

    let mut new: HashSet<String> = HashSet::new();

    for (i, c) in medcine.char_indices() {
        for rule in rules {
            if rule.from.len() == 1 {
                if rule.from.chars().next().unwrap() == c {
                    new.insert(medcine.clone());
                }
            }
        }
    }

    let mut prev = None;
    for c in medcine.chars() {
        if let Some(p) = prev {}
        prev = Some(c);
    }
}
