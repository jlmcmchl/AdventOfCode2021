use std::{cmp::Reverse, collections::BinaryHeap};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use ndarray::Array2;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Route {
    path: Vec<usize>,
    distance: usize,
}

impl Route {
    fn is_complete(&self, graph: &Array2<usize>) -> bool {
        let shape = graph.shape()[0];
        (0..shape).all(|node| self.path.contains(&node))
    }
}

impl PartialOrd for Route {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl Ord for Route {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance)
    }
}

fn parse_input(input: &str) -> (Vec<String>, Array2<usize>) {
    let mut nodes = Vec::new();
    let mut distances = Vec::new();
    input.lines().for_each(|line| {
        let (path, dist_str) = line.split_once(" = ").unwrap();
        let dist: usize = dist_str.parse().unwrap();

        let (first, second) = path.split_once(" to ").unwrap();

        // if first node does not exist, create an entry for it
        if !nodes.contains(&first.to_owned()) {
            nodes.push(first.to_owned());
            distances.push(vec![0]);
        }

        // if second node does not exist, create an entry for it
        if !nodes.contains(&second.to_owned()) {
            nodes.push(second.to_owned());
            distances.push(vec![0]);
        }

        // append dist between first and second node to first's list
        let index = nodes.iter().find_position(|node| *node == first);
        distances[index.unwrap().0].push(dist);
    });

    let matrix = Array2::from_shape_fn((nodes.len(), nodes.len()), |(i, j)| {
        distances[i.min(j)][i.abs_diff(j)]
    });

    (nodes, matrix)
}

fn solve_p1((nodes, graph): &(Vec<String>, Array2<usize>)) -> usize {
    let mut routes = BinaryHeap::<Reverse<Route>>::new();
    let node_count = nodes.len();
    println!("{:?}", nodes);
    println!("{}", graph);

    for node in 0..nodes.len() {
        routes.push(Reverse(Route {
            path: vec![node],
            distance: 0,
        }))
    }

    loop {
        let route = routes.pop().unwrap().0;

        if route.is_complete(graph) {
            return route.distance;
        }

        for node in (0..node_count).filter(|node| !route.path.contains(node)) {
            let last = route.path.last().unwrap();

            let mut route = route.clone();
            route.path.push(node);
            route.distance += graph[(*last, node)];
            routes.push(Reverse(route));
        }
    }
}

fn solve_p2((nodes, graph): &(Vec<String>, Array2<usize>)) -> usize {
    let mut routes = BinaryHeap::<Route>::new();
    let node_count = nodes.len();
    println!("{:?}", nodes);
    println!("{}", graph);

    for node in 0..nodes.len() {
        routes.push(Route {
            path: vec![node],
            distance: 0,
        })
    }

    loop {
        let route = routes.pop().unwrap();

        if route.is_complete(graph) {
            return route.distance;
        }

        for node in (0..node_count).filter(|node| !route.path.contains(node)) {
            let last = route.path.last().unwrap();

            let mut route = route.clone();
            route.path.push(node);
            route.distance += graph[(*last, node)];
            routes.push(route);
        }
    }
}

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> (Vec<String>, Array2<usize>) {
    parse_input(input)
}

#[aoc(day9, part1)]
pub fn wrapper_p1(input: &(Vec<String>, Array2<usize>)) -> usize {
    solve_p1(input)
}

#[aoc(day9, part2)]
pub fn wrapper_p2(input: &(Vec<String>, Array2<usize>)) -> usize {
    solve_p2(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_p1() {
        let input = "London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141";

        let parsed_input = super::parse_input(input);
        assert_eq!(605, super::solve_p1(&parsed_input));
    }

    #[test]
    fn test_p2() {
        let input = "London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141";

        let parsed_input = super::parse_input(input);
        assert_eq!(982, super::solve_p2(&parsed_input));
    }
}
