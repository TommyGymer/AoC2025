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

fn score(ingredients: &Vec<Ingredient>, f: u32, c: u32, b: u32, s: u32) -> (u64, u64) {
    let frosting = ingredients.get(0).unwrap();
    let candy = ingredients.get(1).unwrap();
    let butterscotch = ingredients.get(2).unwrap();
    let sugar = ingredients.get(3).unwrap();

    let capacity = frosting.capacity * f as i64
        + candy.capacity * c as i64
        + butterscotch.capacity * b as i64
        + sugar.capacity * s as i64;

    let durability = frosting.durability * f as i64
        + candy.durability * c as i64
        + butterscotch.durability * b as i64
        + sugar.durability * s as i64;

    let flavour = frosting.flavour * f as i64
        + candy.flavour * c as i64
        + butterscotch.flavour * b as i64
        + sugar.flavour * s as i64;

    let texture = frosting.texture * f as i64
        + candy.texture * c as i64
        + butterscotch.texture * b as i64
        + sugar.texture * s as i64;

    let calories = frosting.calories * f as i64
        + candy.calories * c as i64
        + butterscotch.calories * b as i64
        + sugar.calories * s as i64;

    if capacity <= 0 || durability <= 0 || flavour <= 0 || texture <= 0 {
        (0, 0)
    } else {
        let score = (capacity * durability * flavour * texture) as u64;
        (score, if calories == 500 { score } else { 0 })
    }
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

    let (mut part_1, mut part_2) = (0, 0);

    for f in 0..100 {
        for c in 0..(100 - f) {
            for b in 0..(100 - f - c) {
                let s = 100 - f - c - b;
                assert_eq!(100, f + c + b + s);
                let (score_1, score_2) = score(&ingredients, f, c, b, s);
                if score_1 > part_1 {
                    part_1 = score_1;
                }
                if score_2 > part_2 {
                    part_2 = score_2;
                }
            }
        }
    }

    println!("{:?}", part_1);
    println!("{:?}", part_2);
}
