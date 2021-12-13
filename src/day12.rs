use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};
use nalgebra::DMatrix;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cave {
    Start,
    End,
    Small(usize),
    Large(usize),
}

impl Cave {
    fn new(id: usize, tag: &str) -> Self {
        match tag {
            "start" => Cave::Start,
            "end" => Cave::End,
            _ => {
                if tag.as_bytes()[0] <= 96 {
                    Cave::Large(id)
                } else {
                    Cave::Small(id)
                }
            }
        }
    }

    fn compatible(&self, path: &[Cave]) -> bool {
        match self {
            Cave::Start => false,
            Cave::End | Cave::Large(_) => true,
            Cave::Small(_) => !path.iter().any(|cave| cave == self),
        }
    }

    fn is_small(&self) -> bool {
        matches!(self, Cave::Small(_))
    }

    fn compatible2(&self, path: &[Cave], nodes: &[Cave]) -> bool {
        match self {
            Cave::Start => false,
            Cave::End | Cave::Large(_) => true,
            Cave::Small(_) => {
                if !path.contains(self) {
                    return true;
                }
                
                let mut small_node_counts = nodes.iter().map(|_| 0u8).collect::<Vec<_>>();

                path.iter()
                    .filter(|cave| cave.is_small())
                    .for_each(|cave| match cave {
                        &Cave::Small(id) => small_node_counts[id] += 1,
                        _ => {}
                    });

                !small_node_counts.iter().any(|cnt| *cnt == 2)
            }
        }
    }
}

fn parse_input(input: &str) -> (Vec<Cave>, DMatrix<u8>) {
    let mut nodes = Vec::new();
    let mut node_names = Vec::new();
    let mut tunnels = DMatrix::zeros(0, 0);

    input.lines().for_each(|line| {
        let pair = line.split('-').map(str::to_owned).collect::<Vec<_>>();
        if !node_names.contains(&pair[0]) {
            let (rows, cols) = tunnels.shape();

            node_names.push(pair[0].clone());
            let node_id = node_names.len() - 1;
            nodes.push(Cave::new(node_id, &node_names[node_id]));

            // add row, col to DMatrix
            tunnels = tunnels.clone().insert_row(rows, 0);
            tunnels = tunnels.clone().insert_column(cols, 0);
        }

        if !node_names.contains(&pair[1]) {
            let (rows, cols) = tunnels.shape();

            node_names.push(pair[1].clone());
            let node_id = node_names.len() - 1;
            nodes.push(Cave::new(node_id, &node_names[node_id]));

            // add row, col to DMatrix
            tunnels = tunnels.clone().insert_row(rows, 0);
            tunnels = tunnels.clone().insert_column(cols, 0);
        }

        let first_id = node_names.iter().position(|cave| *cave == pair[0]).unwrap();
        let second_id = node_names.iter().position(|cave| *cave == pair[1]).unwrap();

        tunnels[(first_id, second_id)] = 1;
        tunnels[(second_id, first_id)] = 1;
    });

    (nodes, tunnels)
}

fn solve_p1((nodes, tunnels): &(Vec<Cave>, DMatrix<u8>)) -> usize {
    let mut paths = VecDeque::new();
    let mut finished_paths = Vec::new();
    paths.push_back(vec![Cave::Start]);

    while !paths.iter().all(|path| path.last() == Some(&Cave::End)) {
        let pathcnt = paths.len();
        for _ in 0..pathcnt {
            let path = paths.pop_front().unwrap();
            let node = path.last().unwrap();

            if node == &Cave::End {
                finished_paths.push(path);
                continue
            }

            let node_id = nodes.iter().position(|cave| cave == node).unwrap();

            tunnels
                .row(node_id)
                .iter()
                .enumerate()
                .filter(|(id, &tunnel)| tunnel == 1 && nodes[*id].compatible(&path))
                .for_each(|(id, _)| {
                    let mut new_path = path.clone();
                    new_path.push(nodes[id]);
                    paths.push_back(new_path);
                });
        }
    }

    finished_paths.len()
}

fn solve_p2((nodes, tunnels): &(Vec<Cave>, DMatrix<u8>)) -> usize {
    let mut paths = VecDeque::new();
    let mut finished_paths = Vec::new();
    paths.push_back(vec![Cave::Start]);

    while paths.len() > 0 {
        let pathcnt = paths.len();
        for _ in 0..pathcnt {
            let path = paths.pop_front().unwrap();
            let node = path.last().unwrap();

            if node == &Cave::End {
                finished_paths.push(path);
                continue
            } 
            
            let node_id = nodes.iter().position(|cave| cave == node).unwrap();

            tunnels
                .row(node_id)
                .iter()
                .enumerate()
                .filter(|(id, &tunnel)| tunnel == 1 && nodes[*id].compatible2(&path, &nodes))
                .for_each(|(id, _)| {
                    let mut new_path = path.clone();
                    new_path.push(nodes[id]);
                    paths.push_back(new_path);
                });
        }
    }

    finished_paths.len()
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> (Vec<Cave>, DMatrix<u8>) {
    parse_input(input)
}

#[aoc(day12, part1)]
pub fn wrapper_p1(input: &(Vec<Cave>, DMatrix<u8>)) -> usize {
    solve_p1(input)
}

#[aoc(day12, part2)]
pub fn wrapper_p2(input: &(Vec<Cave>, DMatrix<u8>)) -> usize {
    solve_p2(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let input = "start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end";

        let parsed_input = super::parse_input(input);
        assert_eq!(10, super::solve_p1(&parsed_input));
        assert_eq!(36, super::solve_p2(&parsed_input));

        let input =
            "dc-end\nHN-start\nstart-kj\ndc-start\ndc-HN\nLN-dc\nHN-end\nkj-sa\nkj-HN\nkj-dc";

        let parsed_input = super::parse_input(input);
        assert_eq!(19, super::solve_p1(&parsed_input));
        assert_eq!(103, super::solve_p2(&parsed_input));

        let input = "fs-end\nhe-DX\nfs-he\nstart-DX\npj-DX\nend-zg\nzg-sl\nzg-pj\npj-he\nRW-he\nfs-DX\npj-RW\nzg-RW\nstart-pj\nhe-WI\nzg-he\npj-fs\nstart-RW";

        let parsed_input = super::parse_input(input);
        assert_eq!(226, super::solve_p1(&parsed_input));
        assert_eq!(3509, super::solve_p2(&parsed_input));
    }
}
