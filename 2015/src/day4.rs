use std::fmt::UpperHex;

use aoc_runner_derive::{aoc, aoc_generator};
use hex_literal::hex;
use md5::{Digest, Md5};
use rayon::prelude::*;

fn parse_input(input: &str) -> String {
    input.to_string()
}

fn solve_p1(input: &str) -> usize {
    (1..)
        .find(|suffix| {
            let mut hasher = Md5::new();
            let content = format!("{}{}", input, suffix);
            hasher.update(content.as_bytes());
            let result = hasher.finalize();
            result[0] == 0 && result[1] == 0 && result[2] & 0xf0 == 0
        })
        .unwrap()
}

fn solve_p2(input: &str) -> usize {
    (1..)
        .find(|suffix| {
            let mut hasher = Md5::new();
            let content = format!("{}{}", input, suffix);
            hasher.update(content.as_bytes());
            let result = hasher.finalize();
            result[0] == 0 && result[1] == 0 && result[2] == 0
        })
        .unwrap()
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> String {
    parse_input(input)
}

#[aoc(day4, part1)]
pub fn wrapper_p1(input: &str) -> usize {
    solve_p1(input)
}

#[aoc(day4, part2)]
pub fn wrapper_p2(input: &str) -> usize {
    solve_p2(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_p1() {
        let inputs = vec![("abcdef", 609043), ("pqrstuv", 1048970)];

        for (input, expect1) in inputs {
            let parsed_input = super::input_generator(input);
            assert_eq!(expect1, super::solve_p1(&parsed_input));
        }
    }
}
