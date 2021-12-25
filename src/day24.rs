use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    branch::alt, bytes::complete::tag, character::complete::i64, multi::separated_list0, IResult,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Register {
    W,
    X,
    Y,
    Z,
}

#[derive(Debug, Clone, Copy)]
pub enum Argument {
    Value(i64),
    Register(Register),
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Input(Argument),
    Add(Argument, Argument),
    Multiply(Argument, Argument),
    Divide(Argument, Argument),
    Modulo(Argument, Argument),
    Equal(Argument, Argument),
}

#[derive(Debug)]
struct ALU {
    counter: usize,
    instructions: Vec<Instruction>,
    registers: HashMap<Register, i64>,
}

#[derive(Debug)]
enum State {
    ContinueWithInput,
    Halt,
}

impl ALU {
    fn set_argument(&mut self, arg: Argument, input: i64) {
        assert!(matches!(arg, Argument::Register(_)));

        match arg {
            Argument::Register(reg) => self.registers.insert(reg, input),
            _ => unreachable!(),
        };
    }
    fn get_argument(&self, arg: Argument) -> i64 {
        match arg {
            Argument::Value(v) => v,
            Argument::Register(reg) => self.registers.get(&reg).map(|i| *i).unwrap_or_default(),
        }
    }
    fn simulate(&mut self, input: i64) -> (State, i64) {
        assert!(matches!(
            self.instructions[self.counter],
            Instruction::Input(_)
        ));

        match self.instructions[self.counter] {
            Instruction::Input(reg) => self.set_argument(reg, input),
            _ => unreachable!(),
        }
        self.registers.insert(Register::W, input);
        self.counter += 1;

        while self.counter != self.instructions.len() {
            let instr = self.instructions[self.counter];
            match instr {
                Instruction::Input(_) => {
                    return (State::ContinueWithInput, self.registers[&Register::Z])
                }
                Instruction::Add(arg1, arg2) => {
                    let v1 = self.get_argument(arg1);
                    let v2 = self.get_argument(arg2);
                    self.set_argument(arg1, v1 + v2)
                }
                Instruction::Multiply(arg1, arg2) => {
                    let v1 = self.get_argument(arg1);
                    let v2 = self.get_argument(arg2);
                    self.set_argument(arg1, v1 * v2)
                }
                Instruction::Divide(arg1, arg2) => {
                    let v1 = self.get_argument(arg1);
                    let v2 = self.get_argument(arg2);
                    self.set_argument(arg1, v1 / v2)
                }
                Instruction::Modulo(arg1, arg2) => {
                    let v1 = self.get_argument(arg1);
                    let v2 = self.get_argument(arg2);
                    self.set_argument(arg1, v1 % v2)
                }
                Instruction::Equal(arg1, arg2) => {
                    let v1 = self.get_argument(arg1);
                    let v2 = self.get_argument(arg2);
                    self.set_argument(arg1, if v1 == v2 { 1 } else { 0 })
                }
            }
            self.counter += 1;
        }

        (State::Halt, self.registers[&Register::Z])
    }
}

fn parse_w(input: &str) -> IResult<&str, Register> {
    let (rest, _) = tag("w")(input)?;
    Ok((rest, Register::W))
}

fn parse_x(input: &str) -> IResult<&str, Register> {
    let (rest, _) = tag("x")(input)?;
    Ok((rest, Register::X))
}

fn parse_y(input: &str) -> IResult<&str, Register> {
    let (rest, _) = tag("y")(input)?;
    Ok((rest, Register::Y))
}

fn parse_z(input: &str) -> IResult<&str, Register> {
    let (rest, _) = tag("z")(input)?;
    Ok((rest, Register::Z))
}

fn parse_register(input: &str) -> IResult<&str, Argument> {
    let (rest, reg) = alt((parse_w, parse_x, parse_y, parse_z))(input)?;

    Ok((rest, Argument::Register(reg)))
}

fn parse_value(input: &str) -> IResult<&str, Argument> {
    let (rest, value) = i64(input)?;
    Ok((rest, Argument::Value(value)))
}

fn parse_arg(input: &str) -> IResult<&str, Argument> {
    alt((parse_value, parse_register))(input)
}

fn parse_inp(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("inp")(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, arg) = parse_arg(input)?;

    Ok((input, Instruction::Input(arg)))
}

fn parse_add(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("add")(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, arg) = parse_arg(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, arg2) = parse_arg(input)?;

    Ok((input, Instruction::Add(arg, arg2)))
}

fn parse_mul(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul")(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, arg) = parse_arg(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, arg2) = parse_arg(input)?;

    Ok((input, Instruction::Multiply(arg, arg2)))
}

fn parse_div(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("div")(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, arg) = parse_arg(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, arg2) = parse_arg(input)?;

    Ok((input, Instruction::Divide(arg, arg2)))
}

fn parse_mod(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mod")(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, arg) = parse_arg(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, arg2) = parse_arg(input)?;

    Ok((input, Instruction::Modulo(arg, arg2)))
}

fn parse_equ(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("eql")(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, arg) = parse_arg(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, arg2) = parse_arg(input)?;

    Ok((input, Instruction::Equal(arg, arg2)))
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        parse_inp, parse_add, parse_mul, parse_div, parse_mod, parse_equ,
    ))(input)
}

fn parse_input(input: &str) -> Vec<Instruction> {
    let parsed = separated_list0(tag("\n"), parse_instruction)(input).unwrap();

    parsed.1
}

fn find_highest_pair(input: &[Instruction], first_input: usize, second_input: usize) -> (i64, i64) {
    for i in (1..=9).rev() {
        for j in (1..=9).rev() {
            let mut alu = ALU {
                counter: first_input * 18,
                instructions: input.to_owned(),
                registers: HashMap::new(),
            };

            alu.simulate(i);
            alu.counter = second_input * 18;
            let (_, res) = alu.simulate(j);
            if res == 0 {
                return (i, j);
            }
        }
    }

    (0, 0)
}

fn find_lowest_pair(input: &[Instruction], first_input: usize, second_input: usize) -> (i64, i64) {
    for i in 1..=9 {
        for j in 1..=9 {
            let mut alu = ALU {
                counter: first_input * 18,
                instructions: input.to_owned(),
                registers: HashMap::new(),
            };

            alu.simulate(i);
            alu.counter = second_input * 18;
            let (_, res) = alu.simulate(j);
            if res == 0 {
                return (i, j);
            }
        }
    }

    (0, 0)
}

fn solve_p1(input: &[Instruction]) -> usize {
    // println!("{}", input.len());
    // println!("{:?}", input);
    println!("{:?}", find_highest_pair(input, 0, 13));
    println!("{:?}", find_highest_pair(input, 1, 12));
    println!("{:?}", find_highest_pair(input, 2, 11));
    println!("{:?}", find_highest_pair(input, 3, 4));
    println!("{:?}", find_highest_pair(input, 5, 10));
    println!("{:?}", find_highest_pair(input, 6, 7));
    println!("{:?}", find_highest_pair(input, 8, 9));

    Default::default()
}

fn solve_p2(input: &[Instruction]) -> usize {
    // println!("{}", input.len());
    // println!("{:?}", input);
    println!("{:?}", find_lowest_pair(input, 0, 13));
    println!("{:?}", find_lowest_pair(input, 1, 12));
    println!("{:?}", find_lowest_pair(input, 2, 11));
    println!("{:?}", find_lowest_pair(input, 3, 4));
    println!("{:?}", find_lowest_pair(input, 5, 10));
    println!("{:?}", find_lowest_pair(input, 6, 7));
    println!("{:?}", find_lowest_pair(input, 8, 9));

    Default::default()
}

#[aoc_generator(day24)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    parse_input(input)
}

#[aoc(day24, part1)]
pub fn wrapper_p1(input: &[Instruction]) -> usize {
    solve_p1(input)
}

#[aoc(day24, part2)]
pub fn wrapper_p2(input: &[Instruction]) -> usize {
    solve_p2(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let input = "inp w\nadd z w\nmod z 2\ndiv w 2\nadd y w\nmod y 2\ndiv w 2\nadd x w\nmod x 2\ndiv w 2\nmod w 2";

        let parsed_input = super::input_generator(input);
        println!("{:?}", parsed_input);
    }
}
