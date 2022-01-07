use aoc_runner_derive::{aoc, aoc_generator};
use rayon::prelude::*;

pub enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

fn parse_input(input: &str) -> Vec<(Action, (usize, usize), (usize, usize))> {
    input
        .lines()
        .map(|line| {
            let (line, action) = if line.starts_with("turn on") {
                (line.replace("turn on ", ""), Action::TurnOn)
            } else if line.starts_with("toggle") {
                (line.replace("toggle ", ""), Action::Toggle)
            } else if line.starts_with("turn off") {
                (line.replace("turn off ", ""), Action::TurnOff)
            } else {
                unreachable!()
            };
            let (first, last) = line.split_once(" through ").unwrap();
            let first = first
                .split(",")
                .map(|num| num.parse().unwrap())
                .collect::<Vec<_>>();
            let last = last
                .split(",")
                .map(|num| num.parse().unwrap())
                .collect::<Vec<_>>();

            (action, (first[0], last[0]), (first[1], last[1]))
        })
        .collect()
}

fn solve_p1(input: &[(Action, (usize, usize), (usize, usize))]) -> usize {
    let mut board = (0..1000)
        .map(|_| (0..1000).map(|_| false).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for (action, range_x, range_y) in input {
        for x in range_x.0..=range_x.1 {
            for y in range_y.0..=range_y.1 {
                match action {
                    &Action::TurnOn => board[x][y] = true,
                    &Action::Toggle => board[x][y] = !board[x][y],
                    &Action::TurnOff => board[x][y] = false,
                }
            }
        }
    }

    board
        .par_iter()
        .map(|row| row.iter().filter(|v| **v).count())
        .sum()
}

fn solve_p2(input: &[(Action, (usize, usize), (usize, usize))]) -> usize {
    let mut board = (0..1000)
        .map(|_| (0..1000).map(|_| 0).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for (action, range_x, range_y) in input {
        for x in range_x.0..=range_x.1 {
            for y in range_y.0..=range_y.1 {
                match action {
                    &Action::TurnOn => board[x][y] += 1,
                    &Action::Toggle => board[x][y] += 2,
                    &Action::TurnOff => {
                        board[x][y] = (board[x][y] as usize).checked_sub(1usize).unwrap_or(0)
                    }
                }
            }
        }
    }

    board
        .par_iter()
        .map::<_, usize>(|row| row.iter().sum())
        .sum()
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<(Action, (usize, usize), (usize, usize))> {
    parse_input(input)
}

#[aoc(day6, part1)]
pub fn wrapper_p1(input: &[(Action, (usize, usize), (usize, usize))]) -> usize {
    solve_p1(input)
}

#[aoc(day6, part2)]
pub fn wrapper_p2(input: &[(Action, (usize, usize), (usize, usize))]) -> usize {
    solve_p2(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_p1() {
        let inputs = vec![
            ("turn on 0,0 through 999,999\ntoggle 0,0 through 999,0\nturn off 499,499 through 500,500", 1_000_000 - 1_000 - 4)
        ];

        for (input, expect1) in inputs {
            let parsed_input = super::parse_input(input);
            assert_eq!(expect1, super::solve_p1(&parsed_input));
        }
    }

    #[test]
    fn test_p2() {
        let inputs = vec![(
            "turn on 0,0 through 0,0\ntoggle 0,0 through 999,999",
            2000000 + 1,
        )];

        for (input, expect1) in inputs {
            let parsed_input = super::parse_input(input);
            assert_eq!(expect1, super::solve_p2(&parsed_input));
        }
    }
}
