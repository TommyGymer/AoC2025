use std::collections::VecDeque;

use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

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

fn fewest_buttons_counters_recersive(
    item: QueuedCounters,
    buttons: &Vec<Vec<usize>>,
) -> Option<usize> {
    // NOTE: check if all the counters are zero:
    // by going from the required state to the starting state
    if item.counters.iter().all(|l| *l == 0) {
        Some(item.depth)
    } else {
        buttons
            .iter()
            .map(|button| {
                let mut counters = item.counters.to_owned();
                let sucess: bool = button
                    .iter()
                    .map(|i| match counters.get(*i).unwrap().checked_sub(1) {
                        Some(value) => {
                            *counters.get_mut(*i).unwrap() = value;
                            true
                        }
                        None => false,
                    })
                    .all(|b| b);
                if sucess {
                    fewest_buttons_counters_recersive(
                        QueuedCounters {
                            depth: item.depth + 1,
                            counters,
                        },
                        buttons,
                    )
                } else {
                    None
                }
            })
            .fold(None, |l, r| match (l, r) {
                (None, None) => None,
                (None, Some(i)) => Some(i),
                (Some(i), None) => Some(i),
                (Some(l), Some(r)) => {
                    if l > r {
                        Some(r)
                    } else {
                        Some(l)
                    }
                }
            })
    }
}

fn fewest_buttons_counters(joltages: Vec<usize>, buttons: &Vec<Vec<usize>>) -> usize {
    fewest_buttons_counters_recersive(
        QueuedCounters {
            depth: 0,
            counters: joltages,
        },
        buttons,
    )
    .unwrap()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let machine_line = input.split('\n');
    let machines: Vec<Machine> = machine_line
        .filter(|line| line.len() > 0)
        .map(|line| Machine::from(line))
        .collect();

    let machines_len = machines.len();

    let res: usize = machines
        .into_par_iter()
        .enumerate()
        .map(|(i, machine)| {
            let res = fewest_buttons_counters(machine.joltages, &machine.buttons);
            println!("finished machine {} of {}", i, machines_len);
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
