use aoc_runner_derive::{aoc, aoc_generator};

fn smallest_side((l, w, h): &(usize, usize, usize)) -> usize {
    (l * w).min(l * h).min(w * h)
}

fn sum_sides((l, w, h): &(usize, usize, usize)) -> usize {
    2 * (l * w + l * h + w * h)
}

fn smallest_perimeter((l, w, h): &(usize, usize, usize)) -> usize {
    2 * (l + w).min(l + h).min(w + h)
}

fn volume((l, w, h): &(usize, usize, usize)) -> usize {
    l * w * h
}

fn parse_input(input: &str) -> Vec<(usize, usize, usize)> {
    input
        .lines()
        .map(|line| {
            let (l_str, rest) = line.split_once('x').unwrap();
            let (w_str, h_str) = rest.split_once('x').unwrap();
            (
                l_str.parse().unwrap(),
                w_str.parse().unwrap(),
                h_str.parse().unwrap(),
            )
        })
        .collect()
}

fn solve_p1(input: &[(usize, usize, usize)]) -> usize {
    input
        .iter()
        .map(|dimensions| smallest_side(dimensions) + sum_sides(dimensions))
        .sum()
}

fn solve_p2(input: &[(usize, usize, usize)]) -> usize {
    input
        .iter()
        .map(|dimensions| smallest_perimeter(dimensions) + volume(dimensions))
        .sum()
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<(usize, usize, usize)> {
    parse_input(input)
}

#[aoc(day2, part1)]
pub fn wrapper_p1(input: &[(usize, usize, usize)]) -> usize {
    solve_p1(input)
}

#[aoc(day2, part2)]
pub fn wrapper_p2(input: &[(usize, usize, usize)]) -> usize {
    solve_p2(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let inputs = vec![("2x3x4", 58, 34), ("1x1x10", 43, 14)];

        for (input, expect1, expect2) in inputs {
            let parsed_input = super::input_generator(input);
            assert_eq!(expect1, super::solve_p1(&parsed_input));
            assert_eq!(expect2, super::solve_p2(&parsed_input));
        }
    }
}
