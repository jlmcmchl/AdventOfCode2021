use aoc_runner_derive::{aoc, aoc_generator};

fn parse_input(input: &str) -> String {
    input.to_string()
}

fn is_nice(input: &str) -> bool {
    let vowel_count = input
        .bytes()
        .filter(|b| matches!(*b, b'a' | b'e' | b'i' | b'o' | b'u'))
        .count();
    let double_letters = input
        .as_bytes()
        .windows(2)
        .filter(|pair| pair[0] == pair[1])
        .count();
    let bad_strings = input.as_bytes().windows(2).any(|pair| {
        matches!(
            *pair,
            [b'a', b'b'] | [b'c', b'd'] | [b'p', b'q'] | [b'x', b'y']
        )
    });
    vowel_count >= 3 && double_letters >= 1 && !bad_strings
}

fn is_nice_p2(input: &str) -> bool {
    let pairs_of_pairs = input
        .clone()
        .chars()
        .collect::<Vec<_>>()
        .windows(2)
        .any(|pair| {
            let segment = format!("{}{}", pair[0], pair[1]);
            let (beg, rest) = input.split_once(&segment).unwrap();
            match beg.split_once(&segment) {
                Some(_) => true,
                None => match rest.split_once(&segment) {
                    Some(_) => true,
                    None => false,
                },
            }
        });
    let repeat_with_sep = input
        .as_bytes()
        .windows(3)
        .any(|triplet| triplet[0] == triplet[2] && triplet[0] != triplet[1]);

    pairs_of_pairs && repeat_with_sep
}

fn solve_p1(input: &str) -> usize {
    input.lines().filter(|line| is_nice(line)).count()
}

fn solve_p2(input: &str) -> usize {
    input.lines().filter(|line| is_nice_p2(line)).count()
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> String {
    parse_input(input)
}

#[aoc(day5, part1)]
pub fn wrapper_p1(input: &str) -> usize {
    solve_p1(input)
}

#[aoc(day5, part2)]
pub fn wrapper_p2(input: &str) -> usize {
    solve_p2(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_p1() {
        let inputs = vec![
            ("ugknbfddgicrmopn", true),
            ("aaa", true),
            ("jchzalrnumimnmhp", false),
            ("haegwjzuvuyypxyu", false),
            ("dvszwmarrgswjxmb", false),
        ];

        for (input, expect1) in inputs {
            let parsed_input = super::input_generator(input);
            assert_eq!(expect1, super::is_nice(&parsed_input));
        }
    }

    #[test]
    fn test_p2() {
        let inputs = vec![
            ("qjhvhtzxzqqjkmpb", true),
            ("xxyxx", true),
            ("uurcxstgmygtbstg", false),
            ("ieodomkazucvgmuy", false),
        ];

        for (input, expect1) in inputs {
            let parsed_input = super::input_generator(input);
            assert_eq!(expect1, super::is_nice_p2(&parsed_input));
        }
    }
}
