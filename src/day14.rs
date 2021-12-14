use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

fn step(polymer: &mut HashMap<u16, (u16, usize)>) {
    polymer.clone().iter().for_each(|(pair, (insert, count))| {
        let first = pair & 0xff00 | *insert;
        let second = pair & 0xff | *insert << 8;
        polymer.entry(*pair).and_modify(|record| record.1 -= count);
        polymer.entry(first).and_modify(|record| record.1 += count);
        polymer.entry(second).and_modify(|record| record.1 += count);
    });
}

fn steps(polymer: &mut HashMap<u16, (u16, usize)>, steps: usize) {
    (0..steps).for_each(|_| {
        step(polymer);        
    });
}

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

fn parse_polymer(input: &str) -> HashMap<u16, (u16, usize)> {
    input.as_bytes().windows(2).fold(
        HashMap::<u16, (u16, usize)>::new(),
        |mut agg, input| {
            let bytes = [input[0], input[1]];
            agg.entry(u16::from_ne_bytes(bytes))
                .and_modify(|(_, count)| *count += 1)
                .or_insert((0, 1));
            agg
        },
    )
}

fn parse_reactions(input: &str, polymer: &mut HashMap<u16, (u16, usize)>) {
    input.lines().for_each(|line| {
        if let Some((first, second)) = line.split_once(" -> ") {
            let bytes = first.as_bytes();
            let bytes = [bytes[0], bytes[1]];
            let insert = second.as_bytes()[0] as u16;
            polymer
                .entry(u16::from_ne_bytes(bytes))
                .and_modify(|(replace, _)| *replace = insert)
                .or_insert((insert, 0));
        }
    });
}

fn parse_input(input: &str) -> HashMap<u16, (u16, usize)> {
    if let Some((first, rest)) = input.split_once("\n\n") {
        let mut polymer = parse_polymer(first);

        parse_reactions(rest, &mut polymer);

        polymer
    } else {
        unreachable!();
    }
}

fn equal_polymers(first: &HashMap<u16, (u16, usize)>, second: &HashMap<u16, (u16, usize)>) -> bool {
    let mut first_vec = first.iter().filter(|(_, (_, count))| *count != 0).map(|(&pair, &(_, count))| (pair, (0u16, count))).collect::<Vec<_>>();
    let mut second_vec = second.iter().filter(|(_, (_, count))| *count != 0).map(|(&pair, &(_, count))| (pair, (0u16, count))).collect::<Vec<_>>();

    first_vec.sort_unstable();
    second_vec.sort_unstable();

    first_vec == second_vec
}

fn solve_p1(polymer: &HashMap<u16, (u16, usize)>) -> usize {
    let mut polymer = polymer.clone();

    steps(&mut polymer, 10);

    print_polymer(& polymer);

    let mut chars = polymer
        .iter()
        .fold(
            HashMap::<u8, usize>::new(),
            |mut chars, (pair, (_, count))| {
                pair.to_ne_bytes().iter().for_each(|ch| {
                    chars
                        .entry(*ch)
                        .and_modify(|i| *i += *count)
                        .or_insert(*count);
                });
                chars
            },
        )
        .iter()
        .map(|(char, count)| (*count, *char))
        .collect::<Vec<_>>();

    chars.sort_unstable();

    let least = chars.first().unwrap();
    let most = chars.last().unwrap();

    most.0 - least.0
}

fn solve_p2(polymer: &HashMap<u16, (u16, usize)>) -> usize {
    Default::default()
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> HashMap<u16, (u16, usize)> {
    parse_input(input)
}

#[aoc(day14, part1)]
pub fn wrapper_p1(input: &HashMap<u16, (u16, usize)>) -> usize {
    solve_p1(input)
}

#[aoc(day14, part2)]
pub fn wrapper_p2(input: &HashMap<u16, (u16, usize)>) -> usize {
    solve_p2(input)
}

#[cfg(test)]
mod tests {
    use crate::day14::equal_polymers;


    #[test]
    fn it_works() {
        let input = "NNCB\n\nCH -> B\nHH -> N\nCB -> H\nNH -> C\nHB -> C\nHC -> B\nHN -> C\nNN -> C\nBH -> H\nNC -> B\nNB -> B\nBN -> B\nBB -> N\nBC -> B\nCC -> N\nCN -> C";

        let parsed_input = super::parse_input(input);
        println!("{:?}", &parsed_input);

        let tests = vec![
            (1usize, "NCNBCHB"),
            (2usize, "NBCCNBBBCBHCB"),
            (3usize, "NBBBCNCCNBBNBNBBCHBHHBCHB"),
            (4usize, "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB")];

        tests.iter().for_each(|&(steps, polymer)| {
            let target_polymer = super::parse_polymer(polymer);

            let mut parsed_input = parsed_input.clone();
            super::steps(&mut parsed_input, steps);

            // println!("after {} steps:", steps);
            // super::print_polymer(&parsed_input);
            // println!("expected:");
            // super::print_polymer(&target_polymer);

            // println!("step: {}, expect: {}", steps, polymer);
            assert!(equal_polymers(&parsed_input, &target_polymer))
        });

        assert_eq!(1588, super::solve_p1(&parsed_input));
        assert_eq!(0, super::solve_p2(&parsed_input));


    }
}
