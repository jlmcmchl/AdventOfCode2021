use aoc_runner_derive::{aoc, aoc_generator};

fn valid_password(password: &[u8]) -> bool {
    let pairs = password
        .windows(2)
        .enumerate()
        .filter(|(_, window)| window[0] == window[1])
        .collect::<Vec<_>>();
    // passwords must contain a run of 3 letters in increasing order
    password.windows(3).any(|window| window[0] + 1 == window[1] && window[1] + 1 == window[2])
    // and not contain i, l, or o
    && !password.iter().any(|char| matches!(char, b'i' | b'l' | b'o'))
    // and must contain two distinct pairs of letters
    && pairs.len() >= 2 && pairs.last().unwrap().0 - 1 != pairs.first().unwrap().0
}

fn parse_input(input: &str) -> Vec<u8> {
    input.as_bytes().to_vec()
}

fn next_password(input: &[u8]) -> Vec<u8> {
    let mut input = input.iter().cloned().collect::<Vec<_>>();
    input.reverse();

    let mut carry = true;
    let mut ind = 0;
    loop {
        if !carry {
            break;
        }
        match input[ind] {
            b'z' => input[ind] = b'a',
            b'a'..=b'y' => input[ind] += 1,
            _ => unreachable!(),
        }

        carry = input[ind] == b'a';
        ind += 1;
    }

    input.reverse();
    input.into_iter().collect()
}

fn solve_p1(input: &[u8]) -> String {
    let mut password = next_password(input);
    while !valid_password(&password) {
        password = next_password(&password);
    }

    String::from_utf16(&password.iter().map(|v| *v as u16).collect::<Vec<_>>()).unwrap()
}

fn solve_p2(input: &[u8]) -> String {
    let mut password = next_password(input);
    while !valid_password(&password) {
        password = next_password(&password);
    }

    password = next_password(&password);
    while !valid_password(&password) {
        password = next_password(&password);
    }

    String::from_utf16(&password.iter().map(|v| *v as u16).collect::<Vec<_>>()).unwrap()
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<u8> {
    parse_input(input)
}

#[aoc(day11, part1)]
pub fn wrapper_p1(input: &[u8]) -> String {
    solve_p1(input)
}

#[aoc(day11, part2)]
pub fn wrapper_p2(input: &[u8]) -> String {
    solve_p2(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_validation() {
        let inputs = vec![
            ("hijklmmn", false),
            ("abbceffg", false),
            ("abcdefgh", false),
            ("ghijklmn", false),
            ("abcdffaa", true),
            ("ghjaabcc", true),
        ];

        for (input, expect) in inputs {
            assert_eq!(
                expect,
                super::valid_password(input.as_bytes()),
                "{} was not considered {}valid",
                input,
                if expect { "" } else { "in" }
            );
        }
    }

    #[test]
    fn test_p1() {
        let inputs = vec![("abcdefgh", "abcdffaa"), ("ghijklmn", "ghjaabcc")];

        for (input, expect) in inputs {
            assert_eq!(expect, super::solve_p1(input.as_bytes()));
        }
    }

    #[test]
    fn test_p2() {}
}
