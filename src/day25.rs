use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Map {
    easts: HashSet<(usize, usize)>,
    souths: HashSet<(usize, usize)>,
}

fn parse_input(input: &str) -> ((usize, usize), Map) {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();

    let souths = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.bytes()
                .enumerate()
                .filter(|(_, c)| *c == b'v')
                .map(move |(col, _)| (row, col))
        })
        .collect();
    let easts = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.bytes()
                .enumerate()
                .filter(|(_, c)| *c == b'>')
                .map(move |(col, _)| (row, col))
        })
        .collect();

    ((rows, cols), Map { easts, souths })
}

fn step(maps: &Map, grid: (usize, usize)) -> (Map, usize) {
    let mut moves = 0;

    let mut easts = HashSet::new();

    maps.easts.iter().for_each(|(row, col)| {
        let target = (*row, (col + 1) % grid.1);
        if !maps.easts.contains(&target) && !maps.souths.contains(&target) {
            moves += 1;

            easts.insert(target);
        } else {
            easts.insert((*row, *col));
        }
    });

    let mut souths = HashSet::new();

    maps.souths.iter().for_each(|(row, col)| {
        let target = ((row + 1) % grid.0, *col);
        if !easts.contains(&target) && !maps.souths.contains(&target) {
            moves += 1;

            souths.insert(target);
        } else {
            souths.insert((*row, *col));
        }
    });

    (Map { easts, souths }, moves)
}

#[allow(unused)]
fn after_steps(map: &Map, grid: (usize, usize), step_count: usize) -> Map {
    (0..step_count).fold(map.clone(), |map, _| {
        let (new_map, _) = step(&map, grid);
        new_map
    })
}

#[allow(unused)]
fn print_grid(map: &Map, grid: (usize, usize)) {
    for i in 0..grid.0 {
        for j in 0..grid.1 {
            if map.easts.contains(&(i, j)) {
                print!(">");
            } else if map.souths.contains(&(i, j)) {
                print!("v");
            } else {
                print!(".");
            }
        }
        println!()
    }
}

fn solve_p1(input: &((usize, usize), Map)) -> usize {
    let mut steps = 0;
    let mut map = input.1.clone();

    loop {
        steps += 1;
        let (new_map, moves) = step(&map, input.0);

        // println!("step: {} moves: {}", steps, moves);

        if moves == 0 {
            return steps;
        }

        map = new_map;
    }
}

#[aoc_generator(day25)]
pub fn input_generator(input: &str) -> ((usize, usize), Map) {
    parse_input(input)
}

#[aoc(day25, part1)]
pub fn wrapper_p1(input: &((usize, usize), Map)) -> usize {
    solve_p1(input)
}

#[cfg(test)]
mod tests {

    #[test]
    fn step_test() {
        let input = "..........\n.>v....v..\n.......>..\n..........";

        let expect = "..........\n.>........\n..v....v>.\n..........";

        let parsed_input = super::parse_input(input);
        let expected = super::parse_input(expect);

        super::print_grid(&parsed_input.1, parsed_input.0);
        println!();
        super::print_grid(&expected.1, expected.0);
        println!();
        super::print_grid(
            &super::after_steps(&parsed_input.1, parsed_input.0, 1),
            parsed_input.0,
        );

        assert_eq!(
            expected.1,
            super::after_steps(&parsed_input.1, parsed_input.0, 1)
        );
    }

    #[test]
    fn it_works() {
        let input = "v...>>.vv>\n.vv>>.vv..\n>>.>v>...v\n>>v>>.>.v.\nv>v.vv.v..\n>.>>..v...\n.vv..>.>v.\nv.v..>>v.v\n....v..v.>";

        // let expects = vec![
        //     (, 0),
        // ];

        let parsed_input = super::input_generator(input);
        println!("{:?}", parsed_input);

        assert_eq!(58, super::solve_p1(&parsed_input));
    }
}
