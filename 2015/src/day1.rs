use aoc_runner_derive::{aoc, aoc_generator};

fn parse_input(input: &str) -> Vec<isize> {
    input
        .bytes()
        .map(|i| match i {
            b'(' => 1,
            b')' => -1,
            _ => 0,
        })
        .collect()
}

fn solve_p1(input: &[isize]) -> isize {
    input.iter().sum()
}

fn solve_p2(input: &[isize]) -> usize {
    let mut floor = 0;
    for (round, action) in input.iter().enumerate() {
        floor += action;
        if floor == -1 {
            return round + 1;
        }
    }

    0
}

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<isize> {
    parse_input(input)
}

#[aoc(day1, part1)]
pub fn wrapper_p1(input: &[isize]) -> isize {
    solve_p1(input)
}

#[aoc(day1, part2)]
pub fn wrapper_p2(input: &[isize]) -> usize {
    solve_p2(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let inputs = vec![
            ("(())", 0),
            ("()()", 0),
            ("(((", 3),
            ("(()(()(", 3),
            ("))(((((", 3),
            ("())", -1),
            ("))(", -1),
            (")))", -3),
            (")())())", -3),
        ];

        for (input, expect1) in inputs {
            let parsed_input = super::input_generator(input);
            assert_eq!(expect1, super::solve_p1(&parsed_input));
        }
    }
}
