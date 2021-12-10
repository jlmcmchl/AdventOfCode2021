use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Symbol {
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,
    OpenCaret,
    CloseCaret,
}

impl From<char> for Symbol {
    fn from(c: char) -> Self {
        match c {
            '(' => Symbol::OpenParen,
            ')' => Symbol::CloseParen,
            '[' => Symbol::OpenBracket,
            ']' => Symbol::CloseBracket,
            '{' => Symbol::OpenBrace,
            '}' => Symbol::CloseBrace,
            '<' => Symbol::OpenCaret,
            '>' => Symbol::CloseCaret,
            _ => unreachable!(),
        }
    }
}

impl Symbol {
    fn matching_reverse(self, other: &Self) -> bool {
        match self {
            Symbol::CloseParen => *other == Symbol::OpenParen,
            Symbol::CloseBracket => *other == Symbol::OpenBracket,
            Symbol::CloseBrace => *other == Symbol::OpenBrace,
            Symbol::CloseCaret => *other == Symbol::OpenCaret,
            _ => false,
        }
    }

    fn is_close(self) -> bool {
        matches!(
            self,
            Symbol::CloseParen | Symbol::CloseBracket | Symbol::CloseBrace | Symbol::CloseCaret
        )
    }

    fn score(self) -> usize {
        match self {
            Symbol::CloseParen => 3,
            Symbol::CloseBracket => 57,
            Symbol::CloseBrace => 1197,
            Symbol::CloseCaret => 25137,
            Symbol::OpenParen => 1,
            Symbol::OpenBracket => 2,
            Symbol::OpenBrace => 3,
            Symbol::OpenCaret => 4,
        }
    }
}

fn first_illegal_character(input: &[Symbol]) -> Result<Vec<Symbol>, Symbol> {
    let mut ind = 0usize;
    let mut stack = Vec::new();

    while ind < input.len() {
        let sym = input[ind];
        let top = stack.last();
        match top {
            Some(top) => {
                if sym.is_close() {
                    if sym.matching_reverse(top) {
                        stack.pop();
                    } else {
                        return Err(sym);
                    }
                } else {
                    stack.push(sym);
                }
            }
            None => stack.push(sym),
        }
        ind += 1;
    }

    stack.reverse();
    Ok(stack)
}

fn parse_input(input: &str) -> Vec<Vec<Symbol>> {
    input
        .lines()
        .map(|line| line.chars().map::<Symbol, _>(Symbol::from).collect())
        .collect()
}

fn solve_p1(input: &[Vec<Symbol>]) -> usize {
    input
        .iter()
        .filter_map(|line| first_illegal_character(line).err())
        .map(|sym| sym.score())
        .sum()
}

fn solve_p2(input: &[Vec<Symbol>]) -> usize {
    let mut scores: Vec<usize> = input
        .iter()
        .filter_map(|line| first_illegal_character(line).ok())
        .map(|stack| stack.iter().fold(0, |agg, sym| agg * 5 + sym.score()))
        .collect();

    scores.sort_unstable();

    let ind = scores.len() / 2;

    scores[ind]
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<Vec<Symbol>> {
    parse_input(input)
}

#[aoc(day10, part1)]
pub fn wrapper_p1(input: &[Vec<Symbol>]) -> usize {
    solve_p1(input)
}

#[aoc(day10, part2)]
pub fn wrapper_p2(input: &[Vec<Symbol>]) -> usize {
    solve_p2(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

        let parsed_input = super::input_generator(input);
        assert_eq!(26397, super::solve_p1(&parsed_input));
        assert_eq!(288957, super::solve_p2(&parsed_input));
    }
}
