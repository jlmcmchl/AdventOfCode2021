use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<(i32, i32)> {
    input
        .lines()
        .map(|l| {
            let mut segments = l.split(" ");
            match segments.next() {
                Some("forward") => (segments.next().unwrap().parse().unwrap(), 0),
                Some("up") => (0, -segments.next().unwrap().parse::<i32>().unwrap()),
                Some("down") => (0, segments.next().unwrap().parse::<i32>().unwrap()),
                Some(_) => unreachable!(),
                None => unreachable!(),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_p1(input: &[(i32, i32)]) -> i32 {
    let agg = input
        .iter()
        .fold((0, 0), |(tot_x, tot_y), (this_x, this_y)| {
            (tot_x + this_x, tot_y + this_y)
        });
    agg.0 * agg.1
}

#[aoc(day2, part2)]
pub fn solve_p2(input: &[(i32, i32)]) -> i32 {
    let agg = input.iter().fold((0, 0, 0), |agg, this| match this {
        (0, aim) => (agg.0, agg.1, agg.2 + aim),
        (fwd, 0) => (agg.0 + fwd, agg.1 + fwd * agg.2, agg.2),
        _ => unreachable!(),
    });
    agg.0 * agg.1
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let input = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";

        let parsed_input = super::input_generator(input);
        assert_eq!(150, super::solve_p1(&parsed_input));
        assert_eq!(900, super::solve_p2(&parsed_input));
    }
}
