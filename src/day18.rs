use std::{fmt::Debug, ops::Add};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use serde_json::Value;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Node {
    Value(usize),
    Branch,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tree {
    content: [Node; 64],
}

impl Tree {
    fn new() -> Self {
        Tree {
            content: [Node::Branch; 64],
        }
    }

    fn from_json_array(json: &Value) -> Self {
        let mut result = Tree::new();

        result.from_json_array_recursive(1, json);

        result
    }

    fn from_json_array_recursive(&mut self, current_index: usize, json: &Value) {
        match json {
            Value::Number(n) => {
                self.content[current_index] = Node::Value(n.as_u64().unwrap() as usize)
            }
            Value::Array(vs) => {
                self.from_json_array_recursive(Tree::get_left_child(current_index), &vs[0]);
                self.from_json_array_recursive(Tree::get_right_child(current_index), &vs[1]);
            }
            _ => unreachable!(),
        }
    }

    fn depth(node: usize) -> u32 {
        node.log2()
    }

    fn get_left_child(node: usize) -> usize {
        node << 1
    }

    fn get_right_child(node: usize) -> usize {
        (node << 1) | 1
    }

    fn get_parent(node: usize) -> usize {
        match node % 2 {
            0 => node >> 1,
            1 => (node - 1) >> 1,
            _ => unreachable!(),
        }
    }

    fn is_left_of_parent(node: usize) -> bool {
        node % 2 == 0
    }

    fn is_right_of_parent(node: usize) -> bool {
        node % 2 == 1
    }

    fn is_value(&self, node: usize) -> bool {
        matches!(self.content[node], Node::Value(_))
    }

    fn is_node_pair_of_values(&self, index: usize) -> bool {
        match self.content[index] {
            Node::Branch => {
                let left_ind = Self::get_left_child(index);
                let right_ind = Self::get_right_child(index);

                matches!(self.content[left_ind], Node::Value(_))
                    && matches!(self.content[right_ind], Node::Value(_))
            }
            _ => false,
        }
    }

    fn get_next_left_value(&self, index: usize) -> Option<usize> {
        let mut index = index;

        // traverse leftward until we are coming from the right
        while Tree::is_left_of_parent(index) && index != 1 {
            index = Tree::get_parent(index);
        }

        index = Tree::get_parent(index);

        if index == 0 {
            return None;
        }

        //go to left child
        index = Tree::get_left_child(index);

        // traverse until you have found the rightmost value of this subtree
        while !self.is_value(index) {
            index = Tree::get_right_child(index);
        }

        Some(index)
    }

    fn get_next_right_value(&self, index: usize) -> Option<usize> {
        let mut index = index;

        // traverse rightward until we are coming from the left
        while Tree::is_right_of_parent(index) && index != 0 {
            index = Tree::get_parent(index);
        }

        index = Tree::get_parent(index);

        if index == 0 {
            return None;
        }

        //go to right child
        index = Tree::get_right_child(index);

        // traverse until you have found the leftmost value of this subtree
        while !self.is_value(index) {
            index = Tree::get_left_child(index);
        }

        Some(index)
    }

    fn inorder_traversal(&mut self, mut action: impl FnMut(&mut Self, usize) -> bool) -> bool {
        let mut index = 1;
        loop {
            let current_node = &self.content[index];
            match current_node {
                Node::Branch => index = Tree::get_left_child(index),

                Node::Value(_) => {
                    // this is the next inorder node (value)
                    if action(self, index) {
                        return true;
                    }

                    // search until we are left of parent
                    while Tree::is_right_of_parent(index) {
                        index = Tree::get_parent(index);
                    }

                    index = Tree::get_parent(index);

                    if index == 0 {
                        return false;
                    }

                    // this is the next inorder node (branch)
                    if action(self, index) {
                        return true;
                    }

                    //get right (other) child
                    index = Tree::get_right_child(index);
                }
            }
        }
    }

    fn split_node(&mut self, current_index: usize) {
        if let Node::Value(v) = self.content[current_index] {
            self.content[current_index] = Node::Branch;
            let left_index = Tree::get_left_child(current_index);
            let right_index = Tree::get_right_child(current_index);

            let (left_value, right_value) = if v % 2 == 0 {
                (v / 2, v / 2)
            } else {
                (v / 2, v / 2 + 1)
            };

            self.content[left_index] = Node::Value(left_value);
            self.content[right_index] = Node::Value(right_value);
        }
    }

    fn split_once(&mut self) -> bool {
        self.inorder_traversal(|tree, index| {
            match &tree.content[index] {
                Node::Value(v) if *v > 9 => {
                    tree.split_node(index);
                    return true;
                }
                _ => {}
            }
            false
        })
    }

    fn explode_node(&mut self, index: usize) {
        let left_child = Tree::get_left_child(index);
        let left_value = if let Node::Value(v) = self.content[left_child] {
            v
        } else {
            0
        };

        if let Some(left_index) = self.get_next_left_value(left_child) {
            if let Node::Value(v) = &mut self.content[left_index] {
                *v += left_value;
            }
        }

        let right_child = Tree::get_right_child(index);
        let right_value = if let Node::Value(v) = self.content[right_child] {
            v
        } else {
            0
        };

        if let Some(right_index) = self.get_next_right_value(right_child) {
            if let Node::Value(v) = &mut self.content[right_index] {
                *v += right_value;
            }
        }

        self.content[index] = Node::Value(0);
        self.content[left_child] = Node::Branch;
        self.content[right_child] = Node::Branch;
    }

    fn explode_once(&mut self) -> bool {
        self.inorder_traversal(|tree, index| {
            match &tree.content[index] {
                Node::Branch if tree.is_node_pair_of_values(index) && Tree::depth(index) >= 4 => {
                    tree.explode_node(index);
                    return true;
                }
                _ => {}
            }
            false
        })
    }

    fn magnitude(&self) -> usize {
        self.magnitude_recursive(1)
    }

    fn magnitude_recursive(&self, index: usize) -> usize {
        let node = self.content[index];

        match node {
            Node::Value(v) => v,
            Node::Branch => {
                3 * self.magnitude_recursive(Tree::get_left_child(index))
                    + 2 * self.magnitude_recursive(Tree::get_right_child(index))
            }
        }
    }

    fn reduce(&mut self) {
        while self.explode_once() || self.split_once() {}
    }
}

impl Add for Tree {
    type Output = Tree;

    fn add(self, rhs: Self) -> Self::Output {
        let mut joined = Tree::new();

        self.content
            .iter()
            .enumerate()
            .filter(|(_, v)| matches!(v, Node::Value(_)))
            .for_each(|(i, v)| {
                let ind = 2usize.pow(Tree::depth(i)) + i;

                joined.content[ind] = *v;
            });

        rhs.content
            .iter()
            .enumerate()
            .filter(|(_, v)| matches!(v, Node::Value(_)))
            .for_each(|(i, v)| {
                let ind = 2usize.pow(Tree::depth(i) + 1) + i;

                joined.content[ind] = *v;
            });

        joined
    }
}

impl Add for &Tree {
    type Output = Tree;

    fn add(self, rhs: Self) -> Self::Output {
        let mut joined = Tree::new();

        self.content
            .iter()
            .enumerate()
            .filter(|(_, v)| matches!(v, Node::Value(_)))
            .for_each(|(i, v)| {
                let ind = 2usize.pow(Tree::depth(i)) + i;

                joined.content[ind] = *v;
            });

        rhs.content
            .iter()
            .enumerate()
            .filter(|(_, v)| matches!(v, Node::Value(_)))
            .for_each(|(i, v)| {
                let ind = 2usize.pow(Tree::depth(i) + 1) + i;

                joined.content[ind] = *v;
            });

        joined
    }
}

fn fold(problem: &[Tree]) -> Tree {
    problem
        .iter()
        .cloned()
        .reduce(|agg, new| {
            let mut full = agg + new;
            full.reduce();
            full
        })
        .unwrap()
}

fn parse_node(input: &str) -> Tree {
    let v: Value = match serde_json::from_str(input) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };
    Tree::from_json_array(&v)
}

fn parse_input(input: &str) -> Vec<Tree> {
    input.lines().map(parse_node).collect()
}

fn solve_p1(target: &[Tree]) -> usize {
    let tree = fold(target);
    tree.magnitude()
}

fn solve_p2(target: &[Tree]) -> usize {
    target
        .iter()
        .cartesian_product(target)
        .map(|(first, second)| {
            let mut tree = first + second;
            tree.reduce();
            tree.magnitude()
        })
        .max()
        .unwrap()
}

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Vec<Tree> {
    parse_input(input)
}

#[aoc(day18, part1)]
pub fn wrapper_p1(input: &[Tree]) -> usize {
    solve_p1(input)
}

#[aoc(day18, part2)]
pub fn wrapper_p2(input: &[Tree]) -> usize {
    solve_p2(input)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_add() {
        let first = super::parse_node("[1,2]");
        let second = super::parse_node("[[3,4],5]");

        let expect = super::parse_node("[[1,2],[[3,4],5]]");

        assert_eq!(first + second, expect);
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

            parsed_input.explode_once();
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

        let mut concat = first + second;

        // println!("start   {:?}", concat);
        concat.reduce();

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
            assert_eq!(parsed_input.magnitude(), expect);
        }
    }

    #[test]
    fn test_p2() {
        let input = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]\n[[[5,[2,8]],4],[5,[[9,9],0]]]\n[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]\n[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]\n[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]\n[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]\n[[[[5,4],[7,7]],8],[[8,3],8]]\n[[9,3],[[9,9],[6,[4,9]]]]\n[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]\n[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
        let parsed_input = super::parse_input(input);
        assert_eq!(3993, super::solve_p2(&parsed_input));
    }
}
