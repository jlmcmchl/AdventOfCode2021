use std::{cmp::Ordering, collections::BinaryHeap};

use aoc_runner_derive::{aoc, aoc_generator};
use ndarray::{Array2, Axis};

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

trait Coord {
    fn convert(&self, shape: &[usize]) -> usize;
    fn from(coord: usize, shape: &[usize]) -> Self;
}

impl Coord for (usize, usize) {
    fn convert(&self, shape: &[usize]) -> usize {
        self.0 * shape[0] + self.1
    }

    fn from(coord: usize, shape: &[usize]) -> Self {
        (coord / shape[0], coord % shape[0])
    }
}

fn nearby(grid: &Array2<usize>, coord: &(usize, usize)) -> Vec<(usize, usize)> {
    let shape = grid.shape();
    let mut possibles = vec![];

    if coord.0 != 0 {
        possibles.push((coord.0 - 1, coord.1));
    }

    if coord.0 + 1 < shape[0] {
        possibles.push((coord.0 + 1, coord.1));
    }

    if coord.1 != 0 {
        possibles.push((coord.0, coord.1 - 1));
    }

    if coord.1 + 1 < shape[1] {
        possibles.push((coord.0, coord.1 + 1));
    }

    possibles
}

fn dijkstra(grid: &Array2<usize>, start: &(usize, usize)) -> (Vec<usize>, Vec<usize>) {
    let shape = grid.shape();
    let mut dist = (0..grid.len()).map(|_| usize::MAX).collect::<Vec<_>>();
    let mut prev = (0..grid.len()).map(|_| usize::MAX).collect::<Vec<_>>();

    let mut queue = BinaryHeap::new();

    dist[start.convert(shape)] = 0;
    queue.push(State {
        cost: 0,
        position: *start,
    });

    while let Some(State { cost, position }) = queue.pop() {
        if cost > dist[position.convert(shape)] {
            continue;
        }

        for edge in nearby(grid, &position) {
            let next = State {
                cost: cost + grid[edge],
                position: edge,
            };

            if next.cost < dist[edge.convert(shape)] {
                queue.push(next);

                dist[edge.convert(shape)] = next.cost;
                prev[edge.convert(shape)] = position.convert(shape);
            }
        }
    }

    (dist, prev)
}

#[allow(unused)]
fn rebuild_path(prev: Vec<usize>, start: usize, goal: usize) -> Vec<usize> {
    let mut coord = goal;
    let mut path = Vec::new();

    while coord != start {
        path.push(coord);
        coord = prev[coord];
    }

    path.push(coord);

    path.reverse();

    path
}

fn expand(grid: &Array2<usize>) -> Array2<usize> {
    let mut space = grid.clone();

    space
        .append(Axis(1), ((grid + 1 - 1) % 9 + 1).view())
        .unwrap();
    space
        .append(Axis(1), ((grid + 2 - 1) % 9 + 1).view())
        .unwrap();
    space
        .append(Axis(1), ((grid + 3 - 1) % 9 + 1).view())
        .unwrap();
    space
        .append(Axis(1), ((grid + 4 - 1) % 9 + 1).view())
        .unwrap();

    let mut galaxy = space.clone();

    galaxy
        .append(Axis(0), ((&space + 1 - 1) % 9 + 1).view())
        .unwrap();
    galaxy
        .append(Axis(0), ((&space + 2 - 1) % 9 + 1).view())
        .unwrap();
    galaxy
        .append(Axis(0), ((&space + 3 - 1) % 9 + 1).view())
        .unwrap();
    galaxy
        .append(Axis(0), ((&space + 4 - 1) % 9 + 1).view())
        .unwrap();

    galaxy
}

fn parse_input(input: &str) -> Array2<usize> {
    let rows = input.lines().count();
    let cols = input.lines().next().map(|line| line.len()).unwrap();
    let bytes = input
        .bytes()
        .filter(|c| *c != b'\n')
        .map(|i| i as usize - 48)
        .collect::<Vec<_>>();
    Array2::from_shape_vec((rows, cols), bytes).unwrap()
}

fn solve_p1(input: &Array2<usize>) -> usize {
    let shape = input.shape();

    let start = &(0, 0);
    let goal = &(shape[0] - 1, shape[1] - 1);

    let (dist, _) = dijkstra(input, start);

    // println!(
    //     "{:?}",
    //     rebuild_path(prev, start.convert(shape), goal.convert(shape))
    //         .iter()
    //         .map(|i| <(usize, usize) as Coord>::from(*i, shape))
    //         .collect::<Vec<_>>()
    // );

    dist[goal.convert(shape)]
}

fn solve_p2(input: &Array2<usize>) -> usize {
    let input = expand(input);
    let shape = input.shape();

    let start = &(0, 0);
    let goal = &(shape[0] - 1, shape[1] - 1);

    let (dist, _) = dijkstra(&input, start);

    // println!(
    //     "{:?}",
    //     rebuild_path(prev, start.convert(shape), goal.convert(shape))
    //         .iter()
    //         .map(|i| <(usize, usize) as Coord>::from(*i, shape))
    //         .collect::<Vec<_>>()
    // );

    dist[goal.convert(shape)]
}

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Array2<usize> {
    parse_input(input)
}

#[aoc(day15, part1)]
pub fn wrapper_p1(input: &Array2<usize>) -> usize {
    solve_p1(input)
}

#[aoc(day15, part2)]
pub fn wrapper_p2(input: &Array2<usize>) -> usize {
    solve_p2(input)
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let input = "1163751742\n1381373672\n2136511328\n3694931569\n7463417111\n1319128137\n1359912421\n3125421639\n1293138521\n2311944581";

        let parsed_input = super::parse_input(input);

        assert_eq!(40, super::solve_p1(&parsed_input));
        assert_eq!(315, super::solve_p2(&parsed_input));
    }
}
