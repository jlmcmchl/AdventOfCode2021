use aoc_runner_derive::{aoc, aoc_generator};

fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|line| line.trim().to_string()).collect()
}

fn solve_p1(input: &[String]) -> usize {
    input
        .iter()
        .map(|line| {
            println!("{:?} -> {:?}", line, enquote::unescape(line, None));
            line.len() - enquote::unescape(line, None).unwrap().len()
        })
        .sum()
}

fn solve_p2(input: &[String]) -> usize {
    Default::default()
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<String> {
    parse_input(input)
}

#[aoc(day8, part1)]
pub fn wrapper_p1(input: &[String]) -> usize {
    solve_p1(input)
}

#[aoc(day8, part2)]
pub fn wrapper_p2(input: &[String]) -> usize {
    solve_p2(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_p1() {
        let input = r#"""
            "abc"
            "aaa\"aaa"
            "\x27""#;

        let parsed_input = super::parse_input(input);
        assert_eq!(12, super::solve_p1(&parsed_input));
    }

    #[test]
    fn test_p2() {}
}
