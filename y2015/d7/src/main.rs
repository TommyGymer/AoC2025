use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt::Display,
    fs,
};

use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_till},
    combinator::map_res,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Operator {
    AND,
    OR,
    NOT,
    LSHIFT,
    RSHIFT,
    ASSIGN,
}

#[derive(Debug)]
enum ParseOperatorError {
    UnknownOperator(String),
}

impl Display for ParseOperatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownOperator(value) => writeln!(f, "{} is an unknown operator", value),
        }
    }
}

impl Error for ParseOperatorError {}

impl Operator {
    fn from_str(value: &str) -> Result<Self, ParseOperatorError> {
        match value {
            "AND" => Ok(Operator::AND),
            "OR" => Ok(Operator::OR),
            "NOT" => Ok(Operator::NOT),
            "LSHIFT" => Ok(Operator::LSHIFT),
            "RSHIFT" => Ok(Operator::RSHIFT),
            _ => Err(ParseOperatorError::UnknownOperator(value.to_owned())),
        }
    }
}

fn operator(input: &str) -> IResult<&str, Operator> {
    map_res(take_till(|c| c == ' ' || c == '\n'), Operator::from_str).parse(input)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Variable {
    name: String,
}

impl Variable {
    fn from_str(input: &str) -> Self {
        Self {
            name: input.to_owned(),
        }
    }
}

fn variable(input: &str) -> IResult<&str, Variable> {
    take_till(|c| c == ' ' || c == '\n')
        .map(Variable::from_str)
        .parse(input)
}

#[derive(Debug, Clone)]
enum Operand {
    VAR(Variable),
    LIT(u16),
}

enum OperandParseError {}

impl Operand {
    fn from_str(input: &str) -> Result<Operand, OperandParseError> {
        if let Ok(i) = u16::from_str_radix(input, 10) {
            Ok(Operand::LIT(i))
        } else {
            Ok(Operand::VAR(Variable::from_str(input)))
        }
    }
}

fn operand(input: &str) -> IResult<&str, Operand> {
    map_res(take_till(|c| c == ' ' || c == '\n'), Operand::from_str).parse(input)
}

#[derive(Debug, Clone)]
struct Expr {
    operator: Operator,
    operands: Vec<Operand>,
    destination: Variable,
}

fn assign_expr(input: &str) -> IResult<&str, Expr> {
    let (input, a) = operand(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, dest) = variable(input)?;

    Ok((
        input,
        Expr {
            operator: Operator::ASSIGN,
            operands: vec![a],
            destination: dest,
        },
    ))
}

fn single_operand_expr(input: &str) -> IResult<&str, Expr> {
    let (input, op) = operator(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, a) = operand(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, dest) = variable(input)?;

    Ok((
        input,
        Expr {
            operator: op,
            operands: vec![a],
            destination: dest,
        },
    ))
}

fn double_operand_expr(input: &str) -> IResult<&str, Expr> {
    let (input, a) = operand(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, op) = operator(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, b) = operand(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, dest) = variable(input)?;

    Ok((
        input,
        Expr {
            operator: op,
            operands: vec![a, b],
            destination: dest,
        },
    ))
}

fn expr(input: &str) -> IResult<&str, Expr> {
    alt((single_operand_expr, double_operand_expr, assign_expr)).parse(input)
}

#[derive(Debug, Default)]
struct Machine<'a> {
    state: HashMap<&'a Variable, u16>,
}

impl<'a> Machine<'a> {
    #[inline]
    fn apply(&mut self, expr: &'a Expr) -> bool {
        match expr.operator {
            Operator::ASSIGN => match expr.operands.first().unwrap() {
                Operand::LIT(value) => {
                    self.state.insert(&expr.destination, *value);
                    true
                }
                Operand::VAR(name) => {
                    if let Some(value) = self.state.get(name) {
                        self.state.insert(&expr.destination, *value);
                        true
                    } else {
                        false
                    }
                }
            },
            Operator::AND => match (expr.operands.get(0).unwrap(), expr.operands.get(1).unwrap()) {
                (Operand::LIT(a), Operand::LIT(b)) => {
                    self.state.insert(&expr.destination, a & b);
                    true
                }
                (Operand::LIT(a), Operand::VAR(b)) => {
                    if let Some(value) = self.state.get(b) {
                        self.state.insert(&expr.destination, a & value);
                        true
                    } else {
                        false
                    }
                }
                (Operand::VAR(a), Operand::LIT(b)) => {
                    if let Some(value) = self.state.get(a) {
                        self.state.insert(&expr.destination, value & b);
                        true
                    } else {
                        false
                    }
                }
                (Operand::VAR(a), Operand::VAR(b)) => {
                    if let (Some(a_value), Some(b_value)) = (self.state.get(a), self.state.get(b)) {
                        self.state.insert(&expr.destination, a_value & b_value);
                        true
                    } else {
                        false
                    }
                }
            },
            Operator::OR => match (expr.operands.get(0).unwrap(), expr.operands.get(1).unwrap()) {
                (Operand::LIT(a), Operand::LIT(b)) => {
                    self.state.insert(&expr.destination, a | b);
                    true
                }
                (Operand::LIT(a), Operand::VAR(b)) => {
                    if let Some(value) = self.state.get(b) {
                        self.state.insert(&expr.destination, a | value);
                        true
                    } else {
                        false
                    }
                }
                (Operand::VAR(a), Operand::LIT(b)) => {
                    if let Some(value) = self.state.get(a) {
                        self.state.insert(&expr.destination, value | b);
                        true
                    } else {
                        false
                    }
                }
                (Operand::VAR(a), Operand::VAR(b)) => {
                    if let (Some(a_value), Some(b_value)) = (self.state.get(a), self.state.get(b)) {
                        self.state.insert(&expr.destination, a_value | b_value);
                        true
                    } else {
                        false
                    }
                }
            },
            Operator::LSHIFT => {
                match (expr.operands.get(0).unwrap(), expr.operands.get(1).unwrap()) {
                    (Operand::LIT(a), Operand::LIT(b)) => {
                        self.state.insert(&expr.destination, a << b);
                        true
                    }
                    (Operand::LIT(a), Operand::VAR(b)) => {
                        if let Some(value) = self.state.get(b) {
                            self.state.insert(&expr.destination, a << value);
                            true
                        } else {
                            false
                        }
                    }
                    (Operand::VAR(a), Operand::LIT(b)) => {
                        if let Some(value) = self.state.get(a) {
                            self.state.insert(&expr.destination, value << b);
                            true
                        } else {
                            false
                        }
                    }
                    (Operand::VAR(a), Operand::VAR(b)) => {
                        if let (Some(a_value), Some(b_value)) =
                            (self.state.get(a), self.state.get(b))
                        {
                            self.state.insert(&expr.destination, a_value << b_value);
                            true
                        } else {
                            false
                        }
                    }
                }
            }
            Operator::RSHIFT => {
                match (expr.operands.get(0).unwrap(), expr.operands.get(1).unwrap()) {
                    (Operand::LIT(a), Operand::LIT(b)) => {
                        self.state.insert(&expr.destination, a >> b);
                        true
                    }
                    (Operand::LIT(a), Operand::VAR(b)) => {
                        if let Some(value) = self.state.get(b) {
                            self.state.insert(&expr.destination, a >> value);
                            true
                        } else {
                            false
                        }
                    }
                    (Operand::VAR(a), Operand::LIT(b)) => {
                        if let Some(value) = self.state.get(a) {
                            self.state.insert(&expr.destination, value >> b);
                            true
                        } else {
                            false
                        }
                    }
                    (Operand::VAR(a), Operand::VAR(b)) => {
                        if let (Some(a_value), Some(b_value)) =
                            (self.state.get(a), self.state.get(b))
                        {
                            self.state.insert(&expr.destination, a_value >> b_value);
                            true
                        } else {
                            false
                        }
                    }
                }
            }
            Operator::NOT => match expr.operands.first().unwrap() {
                Operand::LIT(value) => {
                    self.state.insert(&expr.destination, !*value);
                    true
                }
                Operand::VAR(name) => {
                    if let Some(value) = self.state.get(name) {
                        self.state.insert(&expr.destination, !*value);
                        true
                    } else {
                        false
                    }
                }
            },
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let lines: Vec<&str> = input.split('\n').filter(|line| line.len() > 0).collect();

    let exprs: Vec<(usize, Expr)> = lines
        .into_iter()
        .enumerate()
        .filter_map(|(i, line)| match expr(line) {
            Ok((_, e)) => Some((i, e)),
            Err(error) => {
                println!("line {}: {:?}", i, error);
                None
            }
        })
        .collect();

    let mut machine = Machine::default();
    let mut applied: HashSet<usize> = HashSet::new();

    let exprs_len = exprs.len();
    while applied.len() < exprs_len {
        for (i, expr) in exprs.iter() {
            if applied.contains(i) {
                continue;
            }

            if machine.apply(expr) {
                applied.insert(*i);
            }
        }
    }

    println!("{:?}", machine.state.get(&Variable::from_str("a")));

    let mut exprs: Vec<Expr> = exprs
        .iter()
        .filter_map(|(_, expr)| {
            if expr.destination != Variable::from_str("b") {
                Some(expr.to_owned())
            } else {
                None
            }
        })
        .collect();

    exprs.push(Expr {
        operator: Operator::ASSIGN,
        operands: vec![Operand::LIT(
            *machine.state.get(&Variable::from_str("a")).unwrap(),
        )],
        destination: Variable::from_str("b"),
    });

    let mut machine = Machine::default();
    let mut applied: HashSet<usize> = HashSet::new();

    let exprs_len = exprs.len();
    while applied.len() < exprs_len {
        for (i, expr) in exprs.iter().enumerate() {
            if applied.contains(&i) {
                continue;
            }

            if machine.apply(expr) {
                applied.insert(i);
            }
        }
    }

    println!("{:?}", machine.state.get(&Variable::from_str("a")));
}
