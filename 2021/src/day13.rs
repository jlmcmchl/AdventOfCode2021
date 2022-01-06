use aoc_runner_derive::{aoc, aoc_generator};

type Paper = (Vec<(usize, usize)>, Vec<(usize, usize)>);

fn parse_input(input: &str) -> Paper {
    let mut dots = Vec::new();
    let mut actions = Vec::new();

    let mut nodes_done = false;
    input.lines().for_each(|line| {
        nodes_done = nodes_done || line.is_empty();
        if nodes_done {
            // actions
            if let Some((first, rest)) = line.split_once("=") {
                let len = first.len();
                match first.as_bytes()[len - 1] {
                    b'x' => {
                        actions.push((rest.parse().unwrap(), 0));
                    }
                    b'y' => {
                        actions.push((0, rest.parse().unwrap()));
                    }
                    _ => unreachable!(),
                }
            };
        } else {
            let pair = line
                .split(',')
                .map(str::parse::<usize>)
                .map(Result::unwrap)
                .collect::<Vec<_>>();
            dots.push((pair[0], pair[1]));
        }
    });

    (dots, actions)
}

fn print_dots(dots: &[(usize, usize)]) {
    let max_x = dots.iter().max_by_key(|x| x.0).unwrap().0;
    let max_y = dots.iter().max_by_key(|x| x.1).unwrap().1;

    for y in 0..=max_y {
        for x in 0..=max_x {
            if dots.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn fold_once(dots: &[(usize, usize)], fold: (usize, usize)) -> Vec<(usize, usize)> {
    dots.iter()
        .map(|(x, y)| match fold {
            (fx, 0) => {
                if x > &fx {
                    (fx * 2 - x, *y)
                } else {
                    (*x, *y)
                }
            }
            (0, fy) => {
                if y > &fy {
                    (*x, fy * 2 - y)
                } else {
                    (*x, *y)
                }
            }
            _ => unreachable!(),
        })
        .collect()
}

fn solve_p1((dots, folds): &Paper) -> usize {
    let mut dots = fold_once(dots, folds[0]);

    dots.sort_unstable();
    dots.dedup();
    dots.len()
}

fn solve_p2((dots, folds): &Paper) -> usize {
    let dots = folds.iter().fold(dots.clone(), |dots, fold| {
        let mut dots = fold_once(&dots, *fold);
        dots.sort_unstable();
        dots.dedup();
        dots
    });

    print_dots(&dots);

    dots.len()
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Paper {
    parse_input(input)
}

#[aoc(day13, part1)]
pub fn wrapper_p1(input: &Paper) -> usize {
    solve_p1(input)
}

#[aoc(day13, part2)]
pub fn wrapper_p2(input: &Paper) -> usize {
    solve_p2(input)
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let input = "6,10\n0,14\n9,10\n0,3\n10,4\n4,11\n6,0\n6,12\n4,1\n0,13\n10,12\n3,4\n3,0\n8,4\n1,10\n2,14\n8,10\n9,0\n\nfold along y=7\nfold along x=5";

        let parsed_input = super::parse_input(input);

        super::print_dots(&parsed_input.0);
        println!("{:?}", &parsed_input.1);
        assert_eq!(17, super::solve_p1(&parsed_input));
        assert_eq!(16, super::solve_p2(&parsed_input));
    }
}
