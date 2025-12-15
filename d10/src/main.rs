use std::collections::VecDeque;

use good_lp::{
    Constraint, Expression, ProblemVariables, Solution, SolverModel, Variable, constraint,
    default_solver, variable, variables,
};

#[derive(Debug)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<u64>>,
    joltages: Vec<u64>,
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
                        .map(|i| u64::from_str_radix(i, 10).unwrap())
                        .collect()
                })
                .collect(),
            joltages: parts
                .last()
                .unwrap()
                .replace("{", "")
                .replace("}", "")
                .split(',')
                .map(|i| u64::from_str_radix(i, 10).unwrap())
                .collect(),
        }
    }
}

#[derive(Debug)]
struct QueuedLights {
    depth: u64,
    lights: Vec<bool>,
}

fn fewest_buttons_lights(lights: Vec<bool>, buttons: Vec<Vec<u64>>) -> u64 {
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
                        .map(|i| {
                            *lights.get_mut(*i as usize).unwrap() =
                                !lights.get(*i as usize).unwrap()
                        })
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
    depth: u64,
    counters: Vec<u64>,
}

struct JoltageSet {
    required: u64,
    acheived: Expression,
}

struct ButtonCounterProblem {
    vars: ProblemVariables,
    joltages: Vec<JoltageSet>,
    total_presses: Expression,
}

impl ButtonCounterProblem {
    fn new(joltages: &Vec<u64>) -> Self {
        Self {
            vars: variables! {},
            joltages: joltages
                .into_iter()
                .map(|joltage| JoltageSet {
                    required: *joltage,
                    acheived: 0.into(),
                })
                .collect(),
            total_presses: 0.into(),
        }
    }

    fn add_button(&mut self, button: Vec<u64>) -> Variable {
        let presses = self.vars.add(variable().min(0).integer());
        self.total_presses += presses;
        let _ = button
            .into_iter()
            .map(|b| self.joltages.get_mut(b as usize).unwrap().acheived += presses)
            .collect::<()>();
        presses
    }

    fn constraints(joltages: Vec<JoltageSet>) -> Vec<Constraint> {
        let mut constraints = Vec::with_capacity(joltages.len());
        for joltage in joltages {
            constraints.push(constraint!(joltage.acheived == joltage.required as u32));
        }
        constraints
    }

    fn least_presses(self) -> impl Solution {
        let objective = self.total_presses;
        self.vars
            .minimise(objective)
            .using(default_solver)
            .with_all(Self::constraints(self.joltages))
            .solve()
            .unwrap()
    }
}

fn fewest_buttons_counters(joltages: Vec<u64>, buttons: Vec<Vec<u64>>) -> u64 {
    let mut button_problem = ButtonCounterProblem::new(&joltages);
    let presses: Vec<Variable> = buttons
        .into_iter()
        .map(|b| button_problem.add_button(b))
        .collect();
    let solution = button_problem.least_presses();
    presses
        .into_iter()
        .map(|p| {
            let f = solution.value(p);
            println!("{}", f);
            f.round() as u64
        })
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("unable to read file");

    let machine_line = input.split('\n');
    let machines: Vec<Machine> = machine_line
        .filter(|line| line.len() > 0)
        .map(|line| Machine::from(line))
        .collect();

    let machines_len = machines.len();

    let res: u64 = machines
        .into_iter()
        .enumerate()
        .map(|(i, machine)| {
            let res = fewest_buttons_counters(machine.joltages, machine.buttons);
            println!(
                "finished machine {} of {} with {}",
                i + 1,
                machines_len,
                res
            );
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
