use std::collections::HashMap;

fn helper_eq<T: PartialEq>(a: Option<T>, b: Option<T>) -> bool {
    match (a, b) {
        (Some(a), Some(b)) => a == b,
        _ => true,
    }
}

fn helper_g<T: PartialOrd>(a: Option<T>, b: Option<T>) -> bool {
    match (a, b) {
        (Some(a), Some(b)) => a > b,
        _ => true,
    }
}

fn helper_l<T: PartialOrd>(a: Option<T>, b: Option<T>) -> bool {
    match (a, b) {
        (Some(a), Some(b)) => a < b,
        _ => true,
    }
}

#[derive(Debug, Clone, Copy)]
struct Sue {
    number: Option<u16>,
    children: Option<u16>,
    cats: Option<u16>,
    samoyeds: Option<u16>,
    pomeranians: Option<u16>,
    akitas: Option<u16>,
    vizslas: Option<u16>,
    goldfish: Option<u16>,
    trees: Option<u16>,
    cars: Option<u16>,
    perfumes: Option<u16>,
}

impl PartialEq for Sue {
    fn eq(&self, other: &Self) -> bool {
        helper_eq(self.children, other.children)
            && helper_g(self.cats, other.cats)
            && helper_eq(self.samoyeds, other.samoyeds)
            && helper_l(self.pomeranians, other.pomeranians)
            && helper_eq(self.akitas, other.akitas)
            && helper_eq(self.vizslas, other.vizslas)
            && helper_l(self.goldfish, other.goldfish)
            && helper_g(self.trees, other.trees)
            && helper_eq(self.cars, other.cars)
            && helper_eq(self.perfumes, other.perfumes)
    }
}

fn main() {
    let input: String = std::fs::read_to_string("input.txt").unwrap();

    let lines: Vec<&str> = input.trim().split("\n").collect();

    let sues: Vec<Sue> = lines
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.split(&[':', ',']).map(|part| part.trim()).collect();

            let num: u16 = u16::from_str_radix(
                parts
                    .first()
                    .unwrap()
                    .split(" ")
                    .collect::<Vec<&str>>()
                    .last()
                    .unwrap(),
                10,
            )
            .unwrap();

            let things: HashMap<&str, u16> = parts
                .into_iter()
                .skip(1)
                .collect::<Vec<&str>>()
                .chunks(2)
                .map(|item| (item[0], u16::from_str_radix(item[1], 10).unwrap()))
                .collect();

            Sue {
                number: Some(num),
                children: things.get("children").cloned(),
                cats: things.get("cats").cloned(),
                samoyeds: things.get("samoyeds").cloned(),
                pomeranians: things.get("pomeranians").cloned(),
                akitas: things.get("akitas").cloned(),
                vizslas: things.get("vizslas").cloned(),
                goldfish: things.get("goldfish").cloned(),
                trees: things.get("trees").cloned(),
                cars: things.get("cars").cloned(),
                perfumes: things.get("perfumes").cloned(),
            }
        })
        .collect();

    let wanted_sue = Sue {
        number: None,
        children: Some(3),
        cats: Some(7),
        samoyeds: Some(2),
        pomeranians: Some(3),
        akitas: Some(0),
        vizslas: Some(0),
        goldfish: Some(5),
        trees: Some(3),
        cars: Some(2),
        perfumes: Some(1),
    };

    let filtered_sues: Vec<Sue> = sues.into_iter().filter(|sue| *sue == wanted_sue).collect();

    println!("{:?}", filtered_sues);
}
