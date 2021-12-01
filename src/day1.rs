use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i16> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_p1(input: &[i16]) -> u32 {
    input
        .windows(2)
        .map(|sl| if sl[1] > sl[0] { 1 } else { 0 })
        .sum()
}

#[aoc(day1, part2)]
pub fn solve_p2(input: &[i16]) -> u32 {
    let sums = input
        .windows(3)
        .map(|sl| sl.iter().sum())
        .collect::<Vec<_>>();
    solve_p1(&sums)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let input = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";

        let parsed_input = super::input_generator(input);
        assert_eq!(7, super::solve_p1(&parsed_input));
        assert_eq!(5, super::solve_p2(&parsed_input));
    }
}
