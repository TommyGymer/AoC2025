use std::collections::{HashMap, VecDeque};

use z3::{
    Solver,
    ast::{Ast, Int},
};

#[derive(Debug)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<usize>,
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let mut parts = value.split(' ');
        Self {
            lights: parts
                .next()
                .unwrap()
                .replace("[", "")
                .replace("]", "")
                .chars()
                .map(|c| match c {
                    '.' => false,
                    '#' => true,
                    _ => panic!(),
                })
                .collect(),
            buttons: parts
                .to_owned()
                .take_while(|p| p.starts_with('('))
                .map(|button| {
                    button
                        .replace("(", "")
                        .replace(")", "")
                        .split(',')
                        .map(|i| usize::from_str_radix(i, 10).unwrap())
                        .collect()
                })
                .collect(),
            joltages: parts
                .last()
                .unwrap()
                .replace("{", "")
                .replace("}", "")
                .split(',')
                .map(|i| usize::from_str_radix(i, 10).unwrap())
                .collect(),
        }
    }
}

#[derive(Debug)]
struct QueuedLights {
    depth: usize,
    lights: Vec<bool>,
}

fn fewest_buttons_lights(lights: Vec<bool>, buttons: Vec<Vec<usize>>) -> usize {
    let mut queue: VecDeque<QueuedLights> = VecDeque::new();
    queue.push_back(QueuedLights {
        depth: 0,
        lights: lights,
    });

    loop {
        let item = queue.pop_front().unwrap();
        // NOTE: check if all the lights are off:
        // by going from the required state to the starting all off state
        if item.lights.iter().all(|l| !*l) {
            return item.depth;
        } else {
            let _ = buttons
                .iter()
                .map(|button| {
                    let mut lights = item.lights.to_owned();
                    let _ = button
                        .iter()
                        .map(|i| *lights.get_mut(*i).unwrap() = !lights.get(*i).unwrap())
                        .collect::<()>();
                    queue.push_back(QueuedLights {
                        depth: item.depth + 1,
                        lights,
                    });
                })
                .collect::<()>();
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct QueuedCounters {
    depth: usize,
    counters: Vec<usize>,
}

fn fewest_buttons_counters(joltages: Vec<usize>, buttons: Vec<Vec<usize>>) -> usize {
    let mut solver = Solver::new();

    // button press counters
    let button_presses: Vec<Int> = buttons
        .iter()
        .enumerate()
        .map(|(i, _)| Int::fresh_const(&format!("press_{}", i)))
        .collect();

    // joltage counters
    let joltage_counters: Vec<Int> = joltages
        .iter()
        .enumerate()
        .map(|(i, _)| Int::fresh_const(&format!("count_{}", i)))
        .collect();

    // cannot press buttons negative times
    let _ = button_presses
        .iter()
        .map(|p| solver.assert(p.ge(0)))
        .collect();

    // set joltage requirements
    let _ = joltage_counters
        .iter()
        .zip(joltages)
        .map(|(c, j)| solver.assert(c.eq(j as u64)))
        .collect();

    // link buttons and joltages
    let _ = joltage_counters
        .iter()
        .enumerate()
        .map(|(i, counter)| {
            solver.assert(
                counter.eq(buttons
                    .iter()
                    .zip(button_presses)
                    .filter_map(|(b, token)| if b.contains(&i) { Some(token) } else { None })
                    .sum()),
            )
        })
        .collect();

    solver
        .solutions(button_presses, false)
        .next()
        .unwrap()
        .into_iter()
        .filter_map(Int::as_u64)
        .map(|i| i as usize)
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("example.txt").expect("unable to read file");

    let machine_line = input.split('\n');
    let machines: Vec<Machine> = machine_line
        .filter(|line| line.len() > 0)
        .map(|line| Machine::from(line))
        .collect();

    let machines_len = machines.len();

    let res: usize = machines
        .into_iter()
        .enumerate()
        .map(|(i, machine)| {
            let res = fewest_buttons_counters(machine.joltages, machine.buttons);
            println!("finished machine {} of {}", i + 1, machines_len);
            res
        })
        .sum();

    println!("{}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fewest_buttons() {
        let res = fewest_buttons_lights(
            vec![false, true, true, true, false, true],
            vec![vec![0, 3, 4], vec![0, 1, 2, 4, 5]],
        );

        assert_eq!(res, 2);
    }
}
