use aoc_runner_derive::{aoc, aoc_generator};

fn get_cost(input: &[usize], target: usize, cost_fn: &impl Fn(usize) -> usize) -> usize {
    input.iter().map(|i| cost_fn(i.abs_diff(target))).sum()
}

fn get_best_cost(input: &[usize], cost_fn: impl Fn(usize) -> usize) -> usize {
    let mut best_cost = usize::MAX;

    for target in 0.. {
        let cost = get_cost(input, target, &cost_fn);

        if cost < best_cost {
            best_cost = cost;
        } else {
            break;
        }
    }

    best_cost
}

fn solve_p1(input: &[usize]) -> usize {
    get_best_cost(input, |i| i)
}

fn solve_p2(input: &[usize]) -> usize {
    get_best_cost(input, |i| i * (i + 1) / 2)
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input.split(',').map(|l| l.parse().unwrap()).collect()
}

#[aoc(day7, part1)]
pub fn wrapper_p1(input: &[usize]) -> usize {
    solve_p1(input)
}

#[aoc(day7, part2)]
pub fn wrapper_p2(input: &[usize]) -> usize {
    solve_p2(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let input = "16,1,2,0,4,2,7,1,2,14";

        let parsed_input = super::input_generator(input);
        assert_eq!(37, super::solve_p1(&parsed_input));
        assert_eq!(168, super::solve_p2(&parsed_input));
    }
}
