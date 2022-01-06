use aoc_runner_derive::{aoc, aoc_generator};

fn in_bounding_box(position: &(isize, isize), target: &((isize, isize), (isize, isize))) -> bool {
    target.0 .0 <= position.0
        && position.0 <= target.0 .1
        && target.1 .0 <= position.1
        && position.1 <= target.1 .1
}

fn step_pos_vel(position: &mut (isize, isize), velocity: &mut (isize, isize)) {
    position.0 += velocity.0;
    position.1 += velocity.1;
    velocity.0 = velocity.0.signum() * (velocity.0.abs() - 1);
    velocity.1 -= 1;
}

fn max_y_if_collides(
    start: &(isize, isize),
    init_velocity: &(isize, isize),
    target: &((isize, isize), (isize, isize)),
) -> Option<isize> {
    let mut position = *start;
    let mut velocity = *init_velocity;

    let mut max_y = position.1;

    while position.1 >= target.1 .0 {
        max_y = max_y.max(position.1);
        if in_bounding_box(&position, target) {
            return Some(max_y);
        }
        step_pos_vel(&mut position, &mut velocity);
    }

    Default::default()
}

fn parse_input(input: &str) -> ((isize, isize), (isize, isize)) {
    let (_, input) = input.split_once(':').unwrap();
    let (x, y) = input.split_once(',').unwrap();
    let (_, xrange) = x.trim().split_at(2);
    let (_, yrange) = y.trim().split_at(2);
    let (x0, x1) = xrange.split_once("..").unwrap();
    let (y0, y1) = yrange.split_once("..").unwrap();

    (
        (x0.parse().unwrap(), x1.parse().unwrap()),
        (y0.parse().unwrap(), y1.parse().unwrap()),
    )
}

fn solve_p1(target: &((isize, isize), (isize, isize))) -> isize {
    (0..=target.0 .1)
        .flat_map(|i| (-100..=100).filter_map(move |j| max_y_if_collides(&(0, 0), &(i, j), target)))
        .max()
        .unwrap()
}

fn solve_p2(target: &((isize, isize), (isize, isize))) -> usize {
    (0..=target.0 .1)
        .flat_map(|i| (-100..=100).filter_map(move |j| max_y_if_collides(&(0, 0), &(i, j), target)))
        .count()
}

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> ((isize, isize), (isize, isize)) {
    parse_input(input)
}

#[aoc(day17, part1)]
pub fn wrapper_p1(input: &((isize, isize), (isize, isize))) -> isize {
    solve_p1(input)
}

#[aoc(day17, part2)]
pub fn wrapper_p2(input: &((isize, isize), (isize, isize))) -> usize {
    solve_p2(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let smth = super::max_y_if_collides(&(0, 0), &(6, 9), &((20, 30), (-10, -5)));

        assert_eq!(Some(45), smth);

        let input = "target area: x=20..30, y=-10..-5";

        let parsed_input = super::input_generator(input);
        assert_eq!(45, super::solve_p1(&parsed_input));
        assert_eq!(112, super::solve_p2(&parsed_input));
    }
}
