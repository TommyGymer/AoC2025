use good_lp::{ProblemVariables, variable};
use regex::Regex;

#[derive(Debug, PartialEq)]
struct Ingredient {
    name: String,
    capacity: i64,
    durability: i64,
    flavour: i64,
    texture: i64,
    calories: i64,
}

fn main() {
    let re = Regex::new(r"([a-zA-Z]+): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)")
        .unwrap();

    let input: String = std::fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .to_owned();

    let ingredients: Vec<Ingredient> = re
        .captures_iter(&input)
        .map(|item| {
            let (_, [name, capacity, durability, flavour, texture, calories]) = item.extract();
            Ingredient {
                name: String::from(name),
                capacity: i64::from_str_radix(capacity, 10).unwrap(),
                durability: i64::from_str_radix(durability, 10).unwrap(),
                flavour: i64::from_str_radix(flavour, 10).unwrap(),
                texture: i64::from_str_radix(texture, 10).unwrap(),
                calories: i64::from_str_radix(calories, 10).unwrap(),
            }
        })
        .collect();

    println!("{:?}", ingredients);

    let mut problem = ProblemVariables::new();
    let mut teaspoon_counts = Vec::with_capacity(ingredients.len());

    for _ in 0..ingredients.len() {
        teaspoon_counts.push(problem.add(variable().min(0)))
    }
}
