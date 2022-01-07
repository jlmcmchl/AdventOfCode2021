use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

fn parse_input(input: &str) -> Vec<(isize, isize)> {
    input
        .bytes()
        .map(|i| match i {
            b'<' => (-1, 0),
            b'>' => (1, 0),
            b'^' => (0, 1),
            b'v' => (0, -1),
            _ => (0, 0),
        })
        .collect()
}

fn visit(input: &[(isize, isize)]) -> HashSet<(isize, isize)> {
    let mut houses = HashSet::<(isize, isize)>::new();
    let mut position = (0, 0);
    houses.insert(position);

    input.iter().for_each(|action| {
        position = (position.0 + action.0, position.1 + action.1);
        houses.insert(position);
    });

    houses
}

fn solve_p1(input: &[(isize, isize)]) -> usize {
    visit(input).len()
}

fn solve_p2(input: &[(isize, isize)]) -> usize {
    let santa_input = input.iter().step_by(2).cloned().collect::<Vec<_>>();
    let robo_santa_input = input.iter().skip(1).step_by(2).cloned().collect::<Vec<_>>();

    let mut santa_houses = visit(&santa_input);
    let robo_santa_houses = visit(&robo_santa_input);

    santa_houses.extend(robo_santa_houses);

    santa_houses.len()
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<(isize, isize)> {
    parse_input(input)
}

#[aoc(day3, part1)]
pub fn wrapper_p1(input: &[(isize, isize)]) -> usize {
    solve_p1(input)
}

#[aoc(day3, part2)]
pub fn wrapper_p2(input: &[(isize, isize)]) -> usize {
    solve_p2(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_p1() {
        let inputs = vec![(">", 2), ("^>v<", 4), ("^v^v^v^v^v", 2)];

        for (input, expect1) in inputs {
            let parsed_input = super::input_generator(input);
            assert_eq!(expect1, super::solve_p1(&parsed_input));
        }
    }

    #[test]
    fn test_p2() {
        let inputs = vec![("^v", 3), ("^>v<", 3), ("^v^v^v^v^v", 11)];

        for (input, expect1) in inputs {
            let parsed_input = super::input_generator(input);
            assert_eq!(expect1, super::solve_p2(&parsed_input));
        }
    }
}
