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

fn score(ingredients: &Vec<Ingredient>, counts: &Vec<u64>) -> Option<i64> {
    let mut sum = ingredients.iter().enumerate().fold(
        Ingredient {
            name: String::from("Sum"),
            capacity: 0,
            durability: 0,
            flavour: 0,
            texture: 0,
            calories: 0,
        },
        |acc, (i, next)| Ingredient {
            name: acc.name,
            capacity: acc.capacity + next.capacity * counts[i] as i64,
            durability: acc.durability + next.durability * counts[i] as i64,
            flavour: acc.flavour + next.flavour * counts[i] as i64,
            texture: acc.texture + next.texture * counts[i] as i64,
            calories: acc.calories + next.calories * counts[i] as i64,
        },
    );

    if sum.capacity < 0 {
        sum.capacity = 0
    }
    if sum.durability < 0 {
        sum.durability = 0
    }
    if sum.flavour < 0 {
        sum.flavour = 0
    }
    if sum.texture < 0 {
        sum.texture = 0
    }

    if counts.iter().sum::<u64>() > 100 {
        None
    } else {
        Some(sum.capacity * sum.durability * sum.flavour * sum.texture)
    }
}

fn local_maxima(ingredients: &Vec<Ingredient>, counts: Vec<u64>) -> i64 {
    let mut inc_capacity = counts.to_owned();
    inc_capacity[0] += 1;
    let inc_capacity_score = score(ingredients, &inc_capacity);

    let mut inc_durability = counts.to_owned();
    inc_durability[0] += 1;
    let inc_durability_score = score(ingredients, &inc_durability);

    let mut inc_flavour = counts.to_owned();
    inc_flavour[0] += 1;
    let inc_flavour_score = score(ingredients, &inc_flavour);

    let mut inc_texture = counts.to_owned();
    inc_texture[0] += 1;
    let inc_texture_score = score(ingredients, &inc_texture);

    let mut scores: Vec<i64> = vec![
        inc_capacity_score,
        inc_durability_score,
        inc_flavour_score,
        inc_texture_score,
    ]
    .into_iter()
    .filter_map(|s| s)
    .collect();

    if scores.len() == 0 {
        return 0;
    }

    scores.sort();
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
}
