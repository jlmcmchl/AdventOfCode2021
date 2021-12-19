use std::{fmt::Debug, thread, time::Duration};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use serde_json::Value;

#[derive(PartialEq, Clone)]
pub enum Node {
    Value(usize),
    Pair(Pair),
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value(arg0) => f.write_str(&format!("{}", arg0)),
            Self::Pair(Pair { left, right }) => f.debug_list().entry(left).entry(right).finish(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Pair {
    left: Box<Node>,
    right: Box<Node>,
}

#[derive(Debug, Clone)]
enum Direction {
    Left,
    Right,
}

impl Node {
    fn is_pair_of_values(&self) -> bool {
        match self {
            Node::Pair(Pair { left, right }) => {
                matches!(**left, Node::Value(_)) && matches!(**right, Node::Value(_))
            }
            _ => false,
        }
    }
}

fn magnitude(tree: &Node) -> usize {
    match tree {
        Node::Value(v) => *v,
        Node::Pair(Pair { left, right }) => 3 * magnitude(left) + 2 * magnitude(right),
    }
}

fn split_once(tree: &mut Node) -> bool {
    let mut stack = vec![];
    let mut first = true;

    while !stack.is_empty() || first {
        first = false;
        let reftree = tree.clone();
        let current_node = get_node(&reftree, &stack);
        match *current_node {
            Node::Value(_) => {
                // maybe split?
                if split_node(tree, &stack, current_node) {
                    // println!("split   {:?}", tree);
                    // thread::sleep(Duration::from_secs(1));
                    return true;
                }

                // move up until we can move right
                while !stack.is_empty() {
                    let dir = stack.pop();
                    match dir {
                        Some(Direction::Left) => {
                            stack.push(Direction::Right);
                            break;
                        }
                        Some(Direction::Right) => {}
                        None => unreachable!(),
                    }
                }
            }
            Node::Pair(_) => {
                // thread::sleep(Duration::from_secs(1));
                stack.push(Direction::Left);
            }
        }
    }

    false
}

fn split_node(tree: &mut Node, stack: &[Direction], current_node: &Node) -> bool {
    if let Some((dir, stack)) = stack.split_last() {
        let parent_node = get_node_mut(tree, stack);
        if let Some(split_node) = split(current_node) {
            // println!("{:?}", parent_node);
            if let Node::Pair(pair) = parent_node {
                // println!("{:?} {:?} {:?} {:?} {:?}", stack, current_node, split_node, pair, dir);
                match dir {
                    Direction::Left => {
                        pair.left = split_node.into();
                    }
                    Direction::Right => {
                        pair.right = split_node.into();
                    }
                }
            }

            return true;
        }
    }
    false
}

fn split(node: &Node) -> Option<Node> {
    match node {
        Node::Value(v) if *v > 9 => {
            let half = v / 2;
            if v % 2 == 0 {
                Some(Node::Pair(Pair {
                    left: Node::Value(half).into(),
                    right: Node::Value(half).into(),
                }))
            } else {
                Some(Node::Pair(Pair {
                    left: Node::Value(half).into(),
                    right: Node::Value(half + 1).into(),
                }))
            }
        }
        _ => None,
    }
}

fn get_node<'a>(tree: &'a Node, steps: &[Direction]) -> &'a Node {
    steps.iter().fold(tree, |node, dir| match node {
        Node::Pair(Pair { left, right }) => match dir {
            Direction::Left => left,
            Direction::Right => right,
        },
        Node::Value(_) => panic!("attempted to navigate a value w/ stack {:?}", steps),
    })
}

fn get_node_mut<'a>(tree: &'a mut Node, steps: &[Direction]) -> &'a mut Node {
    steps.iter().fold(tree, |node, dir| match node {
        Node::Pair(Pair { left, right }) => match dir {
            Direction::Left => left,
            Direction::Right => right,
        },
        Node::Value(_) => panic!("attempted to navigate a value w/ stack {:?}", steps),
    })
}

#[allow(unused)]
fn inorder_traversal(tree: &mut Node, mut action: impl FnMut(&Node, &Vec<Direction>) -> bool) {
    let mut stack = vec![];
    let mut first = true;

    while !stack.is_empty() || first {
        first = false;
        let current_node = get_node_mut(tree, &stack);
        match *current_node {
            Node::Value(_) => {
                if action(current_node, &stack) {
                    return;
                }

                // move up until we can move right
                while !stack.is_empty() {
                    let dir = stack.pop();
                    match dir {
                        Some(Direction::Left) => {
                            stack.push(Direction::Right);
                            break;
                        }
                        Some(Direction::Right) => {}
                        None => unreachable!(),
                    }
                }
            }
            Node::Pair(_) => {
                if action(current_node, &stack) {
                    return;
                }
                stack.push(Direction::Left);
            }
        }
    }
}

#[allow(unused)]
fn explode_once(tree: &mut Node) -> bool {
    let mut stack = vec![];
    let mut first = true;

    while !stack.is_empty() || first {
        first = false;
        let current_node = get_node_mut(tree, &stack);
        match *current_node {
            Node::Value(_) => {
                // move up until we can move right
                while !stack.is_empty() {
                    let dir = stack.pop();
                    match dir {
                        Some(Direction::Left) => {
                            stack.push(Direction::Right);
                            break;
                        }
                        Some(Direction::Right) => {}
                        None => unreachable!(),
                    }
                }
            }
            Node::Pair(_) => {
                if stack.len() >= 4 && current_node.is_pair_of_values() {
                    explode_node(tree, &stack);
                    return true;
                }

                // thread::sleep(Duration::from_secs(1));
                stack.push(Direction::Left);
            }
        }
    }

    false
}

fn explode_node(tree: &mut Node, stack: &[Direction]) {
    let stack = stack.to_owned();
    let current_node = get_node(tree, &stack);
    let (left_value, right_value) = if let Node::Pair(Pair { left, right }) = &*current_node {
        let lv = match **left {
            Node::Value(v) => v,
            _ => unreachable!(),
        };
        let rv = match **right {
            Node::Value(v) => v,
            _ => unreachable!(),
        };

        (lv, rv)
    } else {
        unreachable!()
    };

    // println!(
    //     "exploding {:?} => {:?} {:?}",
    //     stack, left_value, right_value
    // );

    explode_left(tree, &stack, left_value);
    explode_right(tree, &stack, right_value);

    if let Some((dir, stack)) = stack.split_last() {
        if let Node::Pair(pair) = get_node_mut(tree, stack) {
            match dir {
                Direction::Left => {
                    pair.left = Node::Value(0).into();
                }
                Direction::Right => {
                    pair.right = Node::Value(0).into();
                }
            }
        }
    }
}

fn explode_left(tree: &mut Node, stack: &[Direction], left_value: usize) {
    // explode
    {
        let mut left_stack = stack.to_owned();
        // traverse leftward until we are coming from the right
        while !left_stack.is_empty() {
            let dir = left_stack.last().unwrap();
            match dir {
                Direction::Left => {
                    left_stack.pop();
                }
                Direction::Right => {
                    left_stack.pop();
                    left_stack.push(Direction::Left);
                    break;
                }
            }
        }

        // println!("left_stack found top: {:?}", left_stack);

        // check top of stack
        // if value, add left_value to node
        // otherwise descend right
        while !left_stack.is_empty() {
            let current_node = get_node_mut(tree, &left_stack);
            match current_node {
                Node::Value(v) => {
                    *v += left_value;
                    break;
                }
                Node::Pair(_) => {
                    left_stack.push(Direction::Right);
                }
            }
        }

        // println!("done with left");
    }
}

fn explode_right(tree: &mut Node, stack: &[Direction], right_value: usize) {
    // explode
    let mut right_stack = stack.to_owned();
    // traverse rightward until we are coming from the left
    while !right_stack.is_empty() {
        let dir = right_stack.last().unwrap();
        match dir {
            Direction::Right => {
                right_stack.pop();
            }
            Direction::Left => {
                right_stack.pop();
                right_stack.push(Direction::Right);
                break;
            }
        }
    }

    // check top of stack
    // if value, add right_value to node
    // otherwise descend left
    while !right_stack.is_empty() {
        let current_node = get_node_mut(&mut *tree, &right_stack);
        match current_node {
            Node::Value(v) => {
                *v += right_value;
                break;
            }
            Node::Pair(_) => {
                right_stack.push(Direction::Left);
            }
        }
    }
}

fn reduce(tree: &mut Node) {
    while reduce_once(tree) {}
}

fn reduce_once(tree: &mut Node) -> bool {
    let mut changed = true;

    while changed {
        changed = explode_once(tree) || split_once(tree);
    }

    false
}

fn fold(problem: &[Node]) -> Node {
    problem
        .iter()
        .cloned()
        .reduce(|agg, new| {
            // println!("state {:?}", agg);
            let mut full = add(&agg, &new);
            // println!("add     {:?}", full);
            // thread::sleep(Duration::from_secs(1));
            reduce(&mut full);
            // panic!("early exit");
            full
        })
        .unwrap()
}

fn add(first: &Node, second: &Node) -> Node {
    Node::Pair(Pair {
        left: first.clone().into(),
        right: second.clone().into(),
    })
}

fn parse_node_recurse(input: &Value) -> Node {
    match input {
        Value::Number(n) => Node::Value(n.as_u64().unwrap() as usize),
        Value::Array(vs) => Node::Pair(Pair {
            left: parse_node_recurse(&vs[0]).into(),
            right: parse_node_recurse(&vs[1]).into(),
        }),
        _ => unreachable!(),
    }
}

fn parse_node(input: &str) -> Node {
    let v: Value = match serde_json::from_str(input) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };
    parse_node_recurse(&v)
}

fn parse_input(input: &str) -> Vec<Node> {
    input.lines().map(parse_node).collect()
}

fn solve_p1(target: &[Node]) -> usize {
    let reduced = fold(target);
    magnitude(&reduced)
}

fn solve_p2(target: &[Node]) -> usize {
    target.iter().cartesian_product(target).map(|(first, second)| {
        let mut full = add(&first, &second);
        reduce(&mut full);
        magnitude(&full)
    }).max().unwrap()
}

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Vec<Node> {
    parse_input(input)
}

#[aoc(day18, part1)]
pub fn wrapper_p1(input: &[Node]) -> usize {
    solve_p1(input)
}

#[aoc(day18, part2)]
pub fn wrapper_p2(input: &[Node]) -> usize {
    solve_p2(input)
}

#[cfg(test)]
mod tests {
    use crate::day18::{Node, Pair};

    #[test]
    fn test_split() {
        assert_eq!(
            super::split(&Node::Value(10)),
            Some(Node::Pair(Pair {
                left: Node::Value(5).into(),
                right: Node::Value(5).into()
            }))
        );
        assert_eq!(
            super::split(&Node::Value(11)),
            Some(Node::Pair(Pair {
                left: Node::Value(5).into(),
                right: Node::Value(6).into()
            }))
        );
        assert_eq!(
            super::split(&Node::Value(12)),
            Some(Node::Pair(Pair {
                left: Node::Value(6).into(),
                right: Node::Value(6).into()
            }))
        );
    }

    #[test]
    fn test_add() {
        let first = super::parse_node("[1,2]");
        let second = super::parse_node("[[3,4],5]");

        assert_eq!(
            super::add(&first, &second),
            Node::Pair(Pair {
                left: first.into(),
                right: second.into()
            })
        );
    }

    #[test]
    fn test_explode() {
        let explode_tests = vec![
            ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
            ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
            ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
            (
                "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            ),
            (
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
            ),
        ];

        for (input, expect) in explode_tests {
            let mut parsed_input = super::parse_node(input);
            let parsed_expect = super::parse_node(expect);

            super::explode_once(&mut parsed_input);
            assert_eq!(parsed_input, parsed_expect);
        }
    }

    #[test]
    fn basic_reduce_test() {
        let (first, second) = (
            super::parse_node("[[[[4,3],4],4],[7,[[8,4],9]]]"),
            super::parse_node("[1,1]"),
        );
        let expect = super::parse_node("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");

        let mut concat = super::add(&first, &second);

        // println!("start   {:?}", concat);
        super::reduce(&mut concat);

        assert_eq!(concat, expect);
    }

    #[test]
    fn test_reduce() {
        let reduce_tests = vec![
            (
                "[1,1]\n[2,2]\n[3,3]\n[4,4]", 
                "[[[[1,1],[2,2]],[3,3]],[4,4]]"
            ),
            (
                "[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]", 
                "[[[[3,0],[5,3]],[4,4]],[5,5]]"
            ),
            (
                "[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]\n[6,6]", 
                "[[[[5,0],[7,4]],[5,5]],[6,6]]"
            ),
            (
                "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]\n[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]\n[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]\n[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]\n[7,[5,[[3,8],[1,4]]]]\n[[2,[2,2]],[8,[8,1]]]\n[2,9]\n[1,[[[9,3],9],[[9,0],[0,7]]]]\n[[[5,[7,4]],7],1]\n[[[[4,2],2],6],[8,7]]", 
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
            ),
            (
                "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]\n[[[5,[2,8]],4],[5,[[9,9],0]]]\n[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]\n[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]\n[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]\n[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]\n[[[[5,4],[7,7]],8],[[8,3],8]]\n[[9,3],[[9,9],[6,[4,9]]]]\n[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]\n[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]", 
                "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"
            )
        ];

        for (input, expect) in reduce_tests {
            let parsed_input = super::parse_input(input);
            let parsed_expect = super::parse_node(expect);


            // println!("start   {:?}", parsed_input);

            let result = super::fold(&parsed_input);
            assert_eq!(result, parsed_expect);
        }
    }

    #[test]
    fn test_magnitude() {
        let magnitude_tests = vec![
            ("[9,1]", 29),
            ("[1,9]", 21),
            ("[[9,1],[1,9]]", 129),
            ("[[1,2],[[3,4],5]]", 143),
            ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384),
            ("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445),
            ("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791),
            ("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137),
            (
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
                3488,
            ),
            (
                "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]",
                4140,
            ),
        ];

        for (input, expect) in magnitude_tests {
            let parsed_input = super::parse_node(input);
            assert_eq!(super::magnitude(&parsed_input), expect);
        }
    }

    #[test]
    fn test_p2() {
        let input = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]\n[[[5,[2,8]],4],[5,[[9,9],0]]]\n[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]\n[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]\n[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]\n[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]\n[[[[5,4],[7,7]],8],[[8,3],8]]\n[[9,3],[[9,9],[6,[4,9]]]]\n[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]\n[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
        let parsed_input = super::parse_input(input);
        assert_eq!(3993, super::solve_p2(&parsed_input));
    }
}
