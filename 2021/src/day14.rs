use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

type Polymer = (HashMap<u16, (u16, usize)>, HashMap<u16, usize>);

fn step(reactions: &mut HashMap<u16, (u16, usize)>, polymer: &mut HashMap<u16, usize>) {
    reactions
        .clone()
        .iter()
        .for_each(|(pair, (insert, count))| {
            let first = pair & 0xff00 | *insert;
            let second = pair & 0xff | *insert << 8;
            reactions
                .entry(*pair)
                .and_modify(|record| record.1 -= count);
            reactions
                .entry(first)
                .and_modify(|record| record.1 += count);
            reactions
                .entry(second)
                .and_modify(|record| record.1 += count);

            polymer
                .entry(*insert)
                .and_modify(|i| *i += count)
                .or_insert(*count);
        });
}

fn steps(
    reactions: &mut HashMap<u16, (u16, usize)>,
    polymer: &mut HashMap<u16, usize>,
    steps: usize,
) {
    (0..steps).for_each(|_| {
        step(reactions, polymer);
    });
}

#[allow(unused)]
fn print_polymer(polymer: &HashMap<u16, (u16, usize)>) {
    polymer.iter().for_each(|(pair, (insert, count))| {
        let bytes = pair.to_ne_bytes();
        let insert = insert.to_ne_bytes()[0];

        println!(
            "{}{} => {}, {}",
            char::from(bytes[0]),
            char::from(bytes[1]),
            char::from(insert),
            count
        );
    });
}

fn parse_polymer(input: &str) -> Polymer {
    let reactions =
        input
            .as_bytes()
            .windows(2)
            .fold(HashMap::<u16, (u16, usize)>::new(), |mut agg, input| {
                let bytes = [input[0], input[1]];
                agg.entry(u16::from_ne_bytes(bytes))
                    .and_modify(|(_, count)| *count += 1)
                    .or_insert((0, 1));
                agg
            });

    let polymer = input.bytes().fold(HashMap::new(), |mut agg, i| {
        agg.entry(i as u16).and_modify(|i| *i += 1).or_insert(1);
        agg
    });

    (reactions, polymer)
}

fn parse_reactions(input: &str, reactions: &mut HashMap<u16, (u16, usize)>) {
    input.lines().for_each(|line| {
        if let Some((first, second)) = line.split_once(" -> ") {
            let bytes = first.as_bytes();
            let bytes = [bytes[0], bytes[1]];
            let insert = second.as_bytes()[0] as u16;
            reactions
                .entry(u16::from_ne_bytes(bytes))
                .and_modify(|(replace, _)| *replace = insert)
                .or_insert((insert, 0));
        }
    });
}

fn parse_input(input: &str) -> Polymer {
    if let Some((first, rest)) = input.split_once("\n\n") {
        let (mut reactions, polymer) = parse_polymer(first);

        parse_reactions(rest, &mut reactions);

        (reactions, polymer)
    } else {
        unreachable!();
    }
}

#[allow(unused)]
fn equal_reactions(
    first: &HashMap<u16, (u16, usize)>,
    second: &HashMap<u16, (u16, usize)>,
) -> bool {
    let mut first_vec = first
        .iter()
        .filter(|(_, (_, count))| *count != 0)
        .map(|(&pair, &(_, count))| (pair, count))
        .collect::<Vec<_>>();
    let mut second_vec = second
        .iter()
        .filter(|(_, (_, count))| *count != 0)
        .map(|(&pair, &(_, count))| (pair, count))
        .collect::<Vec<_>>();

    first_vec.sort_unstable();
    second_vec.sort_unstable();

    first_vec == second_vec
}

#[allow(unused)]
fn equal_polymers(first: &HashMap<u16, usize>, second: &HashMap<u16, usize>) -> bool {
    let mut first_vec = first
        .iter()
        .filter(|(_, &count)| count != 0)
        .map(|(&pair, &count)| (pair, count))
        .collect::<Vec<_>>();
    let mut second_vec = second
        .iter()
        .filter(|(_, &count)| count != 0)
        .map(|(&pair, &count)| (pair, count))
        .collect::<Vec<_>>();

    first_vec.sort_unstable();
    second_vec.sort_unstable();

    first_vec == second_vec
}

fn polymer_score(polymer: &HashMap<u16, usize>) -> usize {
    let mut chars = polymer
        .iter()
        .map(|(char, count)| (*count, *char))
        .collect::<Vec<_>>();

    chars.sort_unstable();

    let least = chars.first().unwrap();
    let most = chars.last().unwrap();

    most.0 - least.0
}

fn solve_p1((reactions, polymer): &Polymer) -> usize {
    let mut polymer = polymer.clone();
    let mut reactions = reactions.clone();

    steps(&mut reactions, &mut polymer, 10);

    // print_polymer(&reactions);

    polymer_score(&polymer)
}

fn solve_p2((reactions, polymer): &Polymer) -> usize {
    let mut polymer = polymer.clone();
    let mut reactions = reactions.clone();

    steps(&mut reactions, &mut polymer, 40);

    // print_polymer(&reactions);

    polymer_score(&polymer)
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Polymer {
    parse_input(input)
}

#[aoc(day14, part1)]
pub fn wrapper_p1(input: &Polymer) -> usize {
    solve_p1(input)
}

#[aoc(day14, part2)]
pub fn wrapper_p2(input: &Polymer) -> usize {
    solve_p2(input)
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let input = "NNCB\n\nCH -> B\nHH -> N\nCB -> H\nNH -> C\nHB -> C\nHC -> B\nHN -> C\nNN -> C\nBH -> H\nNC -> B\nNB -> B\nBN -> B\nBB -> N\nBC -> B\nCC -> N\nCN -> C";

        let (parsed_reactions, parsed_polymer) = super::parse_input(input);
        println!("{:?} => {:?}", parsed_polymer, &parsed_reactions);

        let tests = vec![
            (1usize, "NCNBCHB"),
            (2usize, "NBCCNBBBCBHCB"),
            (3usize, "NBBBCNCCNBBNBNBBCHBHHBCHB"),
            (4usize, "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB"),
        ];

        tests.iter().for_each(|&(steps, polymer)| {
            let (target_reactions, target_polymer) = super::parse_polymer(polymer);

            let mut parsed_polymer = parsed_polymer.clone();
            let mut parsed_reactions = parsed_reactions.clone();
            super::steps(&mut parsed_reactions, &mut parsed_polymer, steps);

            // println!("after {} steps:", steps);
            // super::print_polymer(&parsed_input);
            // println!("expected:");
            // super::print_polymer(&target_polymer);

            // println!("step: {}, expect: {}", steps, polymer);
            assert!(super::equal_reactions(&target_reactions, &parsed_reactions));
            assert!(super::equal_polymers(&target_polymer, &parsed_polymer));
        });

        assert_eq!(
            1588,
            super::solve_p1(&(parsed_reactions.clone(), parsed_polymer.clone()))
        );
        assert_eq!(
            2188189693529,
            super::solve_p2(&(parsed_reactions.clone(), parsed_polymer.clone()))
        );
    }
}
