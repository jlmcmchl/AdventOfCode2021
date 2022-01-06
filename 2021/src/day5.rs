use std::{cmp::Ordering, collections::HashMap, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub struct Point {
    x: u32,
    y: u32,
}

impl FromIterator<u32> for Point {
    fn from_iter<T: IntoIterator<Item = u32>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let x = iter.next().unwrap();
        let y = iter.next().unwrap();

        Point { x, y }
    }
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.split(',')
            .map(|i| i.parse::<u32>().unwrap())
            .collect::<Point>())
    }
}

#[derive(Debug)]
pub struct Line {
    start: Point,
    end: Point,
}

impl FromIterator<Point> for Line {
    fn from_iter<T: IntoIterator<Item = Point>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let start = iter.next().unwrap();
        let end = iter.next().unwrap();

        match start.x.cmp(&end.x) {
            Ordering::Equal => {
                if start.y > end.y {
                    Line {
                        start: end,
                        end: start,
                    }
                } else {
                    Line { start, end }
                }
            }
            Ordering::Greater => Line {
                start: end,
                end: start,
            },
            Ordering::Less => Line { start, end },
        }
    }
}

impl FromStr for Line {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.split(" -> ")
            .map(|i| i.parse::<Point>().unwrap())
            .collect::<Line>())
    }
}

pub fn solve_p1(input: &[Line]) -> usize {
    // println!("{:?}", input);

    let mut map: HashMap<(u32, u32), u32> = HashMap::new();

    for line in input {
        if line.start.x != line.end.x && line.start.y != line.end.y {
            continue;
        }
        for i in line.start.x..=line.end.x {
            for j in line.start.y..=line.end.y {
                map.entry((i, j)).and_modify(|i| *i += 1).or_insert(1);
            }
        }
    }

    map.iter().filter(|(_, v)| **v > 1).count()
}

pub fn solve_p2(input: &[Line]) -> usize {
    // println!("{:?}", input);

    let mut map: HashMap<(u32, u32), u32> = HashMap::new();

    for line in input {
        if line.start.x != line.end.x && line.start.y != line.end.y {
            // assume 45* diag
            // start.x < end.x guarantee, just check y
            if line.end.y > line.start.y {
                // up-right diag
                for i in 0..=(line.end.x - line.start.x) {
                    map.entry((line.start.x + i, line.start.y + i))
                        .and_modify(|i| *i += 1)
                        .or_insert(1);
                }
            } else {
                // down-right diag
                for i in 0..=(line.end.x - line.start.x) {
                    map.entry((line.start.x + i, line.start.y - i))
                        .and_modify(|i| *i += 1)
                        .or_insert(1);
                }
            }
        } else {
            for i in line.start.x..=line.end.x {
                for j in line.start.y..=line.end.y {
                    map.entry((i, j)).and_modify(|i| *i += 1).or_insert(1);
                }
            }
        }
    }

    map.iter().filter(|(_, v)| **v > 1).count()
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<Line> {
    input.lines().map(|l| l.parse::<Line>().unwrap()).collect()
}

#[aoc(day5, part1)]
pub fn wrapper_p1(input: &[Line]) -> usize {
    solve_p1(input)
}

#[aoc(day5, part2)]
pub fn wrapper_p2(input: &[Line]) -> usize {
    solve_p2(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let input = "0,9 -> 5,9\n8,0 -> 0,8\n9,4 -> 3,4\n2,2 -> 2,1\n7,0 -> 7,4\n6,4 -> 2,0\n0,9 -> 2,9\n3,4 -> 1,4\n0,0 -> 8,8\n5,5 -> 8,2";

        let parsed_input = super::input_generator(input);
        assert_eq!(5, super::solve_p1(&parsed_input));
        assert_eq!(12, super::solve_p2(&parsed_input));
    }
}
