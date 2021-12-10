use aoc_runner_derive::{aoc, aoc_generator};
use std::convert::TryInto;

fn parse_segment(input: &str) -> u8 {
    input.bytes().fold(0u8, |agg, c| agg | 1 << (c - 97))
}

fn deduce(display: &[u8; 14]) -> usize {
    // map[display id] -> displayed number;
    let mut display_map = [0usize; 14];
    // map[displayed number] -> input signal
    let mut inverse_map = [0u8; 10];

    // find 1, 4, 7, 8
    display
        .iter()
        .enumerate()
        .for_each(|(id, mystery)| match mystery.count_ones() {
            2 => {
                inverse_map[1] = *mystery;
                display_map[id] = 1;
            }
            3 => {
                inverse_map[7] = *mystery;
                display_map[id] = 7;
            }
            4 => {
                inverse_map[4] = *mystery;
                display_map[id] = 4;
            }
            7 => {
                inverse_map[8] = *mystery;
                display_map[id] = 8;
            }
            _ => {}
        });

    // the rest
    display.iter().enumerate().for_each(|(id, mystery)| {
        match mystery.count_ones() {
            // 2, 3, 5
            5 => {
                if mystery & inverse_map[7] == inverse_map[7] {
                    inverse_map[3] = *mystery;
                    display_map[id] = 3;
                } else if (mystery & inverse_map[4]).count_ones() == 2 {
                    inverse_map[2] = *mystery;
                    display_map[id] = 2;
                } else {
                    inverse_map[5] = *mystery;
                    display_map[id] = 5;
                }
            }
            // 0, 6, 9
            6 => {
                if mystery & inverse_map[4] == inverse_map[4] {
                    inverse_map[9] = *mystery;
                    display_map[id] = 9;
                } else if mystery & inverse_map[7] == inverse_map[7] {
                    inverse_map[0] = *mystery;
                    display_map[id] = 0;
                } else {
                    inverse_map[6] = *mystery;
                    display_map[id] = 6;
                }
            }
            _ => {}
        }
    });

    display_map[10] * 1000 + display_map[11] * 100 + display_map[12] * 10 + display_map[13]
}

fn parse_input(input: &str) -> Vec<[u8; 14]> {
    input
        .lines()
        .map(|line| {
            line.replace("| ", "")
                .split(' ')
                .map(|segment| parse_segment(segment))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect()
}

fn solve_p1(input: &[[u8; 14]]) -> usize {
    let unique_segment_counts = vec![2, 3, 4, 7];

    input
        .iter()
        .map(|display| {
            display
                .iter()
                .skip(10)
                .filter(|number| unique_segment_counts.contains(&number.count_ones()))
                .count()
        })
        .sum()
}

fn solve_p2(input: &[[u8; 14]]) -> usize {
    input.iter().map(|display| deduce(display)).sum()
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<[u8; 14]> {
    parse_input(input)
}

#[aoc(day8, part1)]
pub fn wrapper_p1(input: &[[u8; 14]]) -> usize {
    solve_p1(input)
}

#[aoc(day8, part2)]
pub fn wrapper_p2(input: &[[u8; 14]]) -> usize {
    solve_p2(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let input_1 =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let larger_input = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\nedbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc\nfgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg\nfbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb\naecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea\nfgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb\ndbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe\nbdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef\negadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb\ngcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

        let parsed_input = super::input_generator(input_1);
        assert_eq!(0, super::solve_p1(&parsed_input));
        assert_eq!(5353, super::solve_p2(&parsed_input));

        let parsed_input = super::input_generator(larger_input);
        assert_eq!(26, super::solve_p1(&parsed_input));
        assert_eq!(61229, super::solve_p2(&parsed_input));
    }
}
