use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

pub type Register = u16;
pub type Literal = u16;

pub enum Value {
    Literal(Literal),
    Register(Register),
}

impl Value {
    fn eval(&self, circuit: &HashMap<Register, Option<Literal>>) -> Option<Literal> {
        match self {
            Value::Literal(v) => Some(*v),
            Value::Register(reg) => {
                if circuit.contains_key(reg) {
                    circuit[reg]
                } else {
                    None
                }
            }
        }
    }
}

pub enum Operation {
    NOP(Value),
    NOT(Value),
    AND(Value, Value),
    OR(Value, Value),
    LSHIFT(Value, Value),
    RSHIFT(Value, Value),
}

impl Operation {
    fn compute(&self, circuit: &mut HashMap<Register, Option<Literal>>, target: Register) {
        if circuit.contains_key(&target) && circuit[&target].is_some() {
            return;
        }

        let result = match self {
            Operation::NOP(first) => first.eval(circuit),
            Operation::NOT(first) => {
                let first = first.eval(circuit);
                first.map(|v| !v)
            }
            Operation::AND(first, second) => {
                let first = first.eval(circuit);
                let second = second.eval(circuit);
                first.and_then(|first| second.map(|second| first & second))
            }
            Operation::OR(first, second) => {
                let first = first.eval(circuit);
                let second = second.eval(circuit);
                first.and_then(|first| second.map(|second| first | second))
            }
            Operation::LSHIFT(first, second) => {
                let first = first.eval(circuit);
                let second = second.eval(circuit);
                first.and_then(|first| second.map(|second| first << second))
            }
            Operation::RSHIFT(first, second) => {
                let first = first.eval(circuit);
                let second = second.eval(circuit);
                first.and_then(|first| second.map(|second| first >> second))
            }
        };

        circuit.entry(target).insert(result);
    }
}

fn parse_value(input: &str) -> Value {
    let target: Result<u16, _> = input.parse();
    match target {
        Ok(v) => Value::Literal(v),
        Err(_) => {
            let reg = input.bytes().fold(0, |acc, v| acc << 8 | (v as u16));
            Value::Register(reg)
        }
    }
}

fn parse_input(input: &str) -> Vec<(Register, Operation)> {
    input
        .lines()
        .map(|line| {
            let (eqn, reg_str) = line.split_once(" -> ").unwrap();
            let target_reg = reg_str.bytes().fold(0, |acc, v| acc << 8 | (v as u16));

            let op = if eqn.contains("NOT") {
                let (_, end) = eqn.split_once("NOT ").unwrap();
                let first = parse_value(end);
                Operation::NOT(first)
            } else if eqn.contains("AND") {
                let (beg, end) = eqn.split_once(" AND ").unwrap();
                let first = parse_value(beg);
                let second = parse_value(end);
                Operation::AND(first, second)
            } else if eqn.contains("OR") {
                let (beg, end) = eqn.split_once(" OR ").unwrap();
                let first = parse_value(beg);
                let second = parse_value(end);
                Operation::OR(first, second)
            } else if eqn.contains("LSHIFT") {
                let (beg, end) = eqn.split_once(" LSHIFT ").unwrap();
                let first = parse_value(beg);
                let second = parse_value(end);
                Operation::LSHIFT(first, second)
            } else if eqn.contains("RSHIFT") {
                let (beg, end) = eqn.split_once(" RSHIFT ").unwrap();
                let first = parse_value(beg);
                let second = parse_value(end);
                Operation::RSHIFT(first, second)
            } else {
                let first = parse_value(eqn);
                Operation::NOP(first)
            };

            (target_reg, op)
        })
        .collect()
}

fn compute_once(input: &[(Register, Operation)], circuit: &mut HashMap<Register, Option<Literal>>) {
    for (target, op) in input {
        op.compute(circuit, *target);
    }
}

fn compute(input: &[(Register, Operation)], circuit: &mut HashMap<Register, Option<Literal>>) {
    compute_once(input, circuit);

    while circuit.iter().any(|(_, v)| v.is_none()) {
        compute_once(input, circuit);
    }
}

fn solve_p1(input: &[(Register, Operation)]) -> u16 {
    let mut circuit = HashMap::<Register, Option<Literal>>::new();
    compute(input, &mut circuit);
    circuit[&(b'a' as u16)].unwrap()
}

fn solve_p2(input: &[(Register, Operation)]) -> u16 {
    let a = solve_p1(input);
    let mut circuit = HashMap::<Register, Option<Literal>>::new();
    circuit.entry(b'b' as u16).insert(Some(a));
    compute(input, &mut circuit);
    circuit[&(b'a' as u16)].unwrap()
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<(Register, Operation)> {
    parse_input(input)
}

#[aoc(day7, part1)]
pub fn wrapper_p1(input: &[(Register, Operation)]) -> u16 {
    solve_p1(input)
}

#[aoc(day7, part2)]
pub fn wrapper_p2(input: &[(Register, Operation)]) -> u16 {
    solve_p2(input)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::day7::{Literal, Register};

    #[test]
    fn test_p1() {
        let input = "123 -> x\n456 -> y\nx AND y -> d\nx OR y -> e\nx LSHIFT 2 -> f\ny RSHIFT 2 -> g\nNOT x -> h\nNOT y -> i";

        let parsed_input = super::parse_input(input);
        let mut circuit = HashMap::<Register, Option<Literal>>::new();
        super::compute(&parsed_input, &mut circuit);
        assert_eq!(circuit[&(b'd' as u16)], Some(72));
        assert_eq!(circuit[&(b'e' as u16)], Some(507));
        assert_eq!(circuit[&(b'f' as u16)], Some(492));
        assert_eq!(circuit[&(b'g' as u16)], Some(114));
        assert_eq!(circuit[&(b'h' as u16)], Some(65412));
        assert_eq!(circuit[&(b'i' as u16)], Some(65079));
        assert_eq!(circuit[&(b'x' as u16)], Some(123));
        assert_eq!(circuit[&(b'y' as u16)], Some(456));
    }

    #[test]
    fn test_p2() {}
}
