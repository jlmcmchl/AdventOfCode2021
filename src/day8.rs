use std::{collections::HashMap, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};
use bit_iter::BitIter;
use std::convert::TryInto;

use lazy_static::lazy_static;

#[derive(Default, Debug)]
pub struct Seg7 {
    inputs: u8,
}

impl FromStr for Seg7 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Seg7 {
            inputs: s.bytes().fold(0u8, |agg, c| agg | 1 << (c - 97)),
        })
    }
}

/*
displayed number -> segment count
0: 6
1: 2
2: 5
3: 5
4: 4
5: 5
6: 6
7: 3
8: 7
9: 6
 ->
segment count -> displayed number
2: 1
3: 7
4: 4
5: 2, 3, 5
6: 0, 6, 9
7: 8

displayed number -> segments
_  0, 1, 2, 3, 4, 5, 6
0: 0, 1, 2,    4, 5, 6
1:       2,       5
2: 0,    2, 3, 4,    6
3: 0,    2, 3,    5, 6
4:    1, 2, 3,    5,
5: 0, 1,    3,    5, 6
6: 0, 1,    3, 4, 5, 6
7: 0,    2,       5
8: 0, 1, 2, 3, 4, 5, 6
9: 0, 1, 2, 3,    5, 6
->
segment -> candidate number
_  0, 1, 2, 3, 4, 5, 6, 7, 8, 9
0: 0,    2, 3,    5, 6, 7, 8, 9
1: 0,          4, 5, 6,    8, 9
2: 0, 1, 2, 3, 4,       7, 8, 9
3:       2, 3, 4, 5, 6,    8, 9
4: 0,    2,          6,    8
5: 0, 1,    3, 4, 5, 6, 7, 8, 9
6: 0,    2, 3,    5, 6,    8, 9

*/

lazy_static! {
    static ref BITCOUNT_TO_CANDIDATE: HashMap<u32, Vec<u8>> = {
        let mut map = HashMap::new();
        map.insert(2, vec![1]);
        map.insert(3, vec![7]);
        map.insert(4, vec![4]);
        map.insert(5, vec![2, 3, 5]);
        map.insert(6, vec![0, 6, 9]);
        map.insert(7, vec![8]);

        map
    };

    static ref SEGMENT_TO_CANDIDATE: Vec<Vec<u8>> = vec![
        /* 0 */ vec![0, 2, 3, 5, 6, 7, 8, 9],
        /* 1 */ vec![0, 4, 5, 6, 8, 9],
        /* 2 */ vec![0, 1, 2, 3, 4, 7, 8, 9],
        /* 3 */ vec![2, 3, 4, 5, 6, 8, 9],
        /* 4 */ vec![0, 2, 6, 8],
        /* 5 */ vec![0, 1, 3, 4, 5, 6, 7, 8, 9],
        /* 6 */ vec![0, 2, 3, 5, 6, 8, 9],
    ];

    static ref CANDIDATE_TO_OBSERVED: Vec<u8> = vec![
        /* 0 */ 0b1110111,
        /* 1 */ 0b0100100,
        /* 2 */ 0b1011101,
        /* 3 */ 0b1101101,
        /* 4 */ 0b0101110,
        /* 5 */ 0b1101011,
        /* 6 */ 0b1111011,
        /* 7 */ 0b0100101,
        /* 8 */ 0b1111111,
        /* 9 */ 0b1101111,
    ];

    static ref OBSERVED_TO_CANDIDATE: HashMap<u8, u8> = {
        let mut map = HashMap::new();

        map.insert(0b1110111, 0);
        map.insert(0b0100100, 1);
        map.insert(0b1011101, 2);
        map.insert(0b1101101, 3);
        map.insert(0b0101110, 4);
        map.insert(0b1101011, 5);
        map.insert(0b1111011, 6);
        map.insert(0b0100101, 7);
        map.insert(0b1111111, 8);
        map.insert(0b1101111, 9);

        map
    };

}

/*
id | possible mapping    | count
0: | 0, 1, 0, 0, 1, 0, 0 | 2
1: | 0, 1, 0, 0, 1, 0, 0 | 2 
2: | 1, 1, 1, 1, 1, 1, 1 | 7 
3: | 0, 1, 0, 0, 1, 0, 1 | 3
4: | 0, 1, 0, 1, 1, 1, 0 | 4
5: | 0, 1, 0, 1, 1, 1, 0 | 4
6: | 1, 1, 1, 1, 1, 1, 1 | 7

sorted by count
-> 
reduce candidates

id | possible mapping    | count
0: | 0, 1, 0, 0, 1, 0, 0 | 2
1: | 0, 1, 0, 0, 1, 0, 0 | 2 
3: | 0, 0, 0, 0, 0, 0, 1 | 1
4: | 0, 0, 0, 1, 0, 1, 0 | 2
5: | 0, 0, 0, 1, 0, 1, 0 | 2
2: | 1, 0, 1, 1, 0, 1, 1 | 5 
6: | 1, 0, 1, 1, 0, 1, 1 | 5

sorted by count
-> 
reduce candidates

id | possible mapping    | count | letter
3: | 0, 0, 0, 0, 0, 0, 1 | 1     | d
0: | 0, 1, 0, 0, 1, 0, 0 | 2     | a
1: | 0, 1, 0, 0, 1, 0, 0 | 2     | b
4: | 0, 0, 0, 1, 0, 1, 0 | 2     | e
5: | 0, 0, 0, 1, 0, 1, 0 | 2     | f
2: | 1, 0, 1, 0, 0, 0, 0 | 2     | c
6: | 1, 0, 1, 0, 0, 0, 0 | 2     | g

  0:      1:      2:      3:      4:
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
 ....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....

  5:      6:      7:      8:      9:
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg

acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf
  8       5               7                 4      0/6   1

 --d--
e    a
f    b
 eeff
c    a
g    b
 ccgg
*/

fn deduce(display: &[Seg7; 14]) -> usize {
    // map[input segment] -> output segment
    let mut signal_map = [127u8; 7];

    display.iter().for_each(|mystery| {
        let candidates = &BITCOUNT_TO_CANDIDATE[&mystery.inputs.count_ones()];

        let candidate_mask = candidates.iter().fold(0u8, |agg, number| {
            agg | CANDIDATE_TO_OBSERVED[*number as usize]
        });
        BitIter::from(mystery.inputs).for_each(|i| signal_map[i] &= candidate_mask);
    });

    signal_map.iter().for_each(|i| print!("{:#b} ", i));

    let mut signal_map_enumerated = signal_map.into_iter().enumerate().collect::<Vec<_>>();
    signal_map_enumerated.sort_by_cached_key(|(_, mask)| mask.count_ones());

    println!("");

    Default::default()
}

fn parse_input(input: &str) -> Vec<[Seg7; 14]> {
    input
        .lines()
        .map(|line| {
            line.replace("| ", "")
                .split(' ')
                .map(|segment| segment.parse::<Seg7>().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect()
}

fn solve_p1(input: &[[Seg7; 14]]) -> usize {
    let unique_segment_counts = vec![2, 3, 4, 7];

    input
        .iter()
        .map(|display| {
            display
                .iter()
                .skip(10)
                .filter(|number| unique_segment_counts.contains(&number.inputs.count_ones()))
                .count()
        })
        .sum()
}

fn solve_p2(input: &[[Seg7; 14]]) -> usize {
    input.iter().map(|display| deduce(display)).sum()
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<[Seg7; 14]> {
    parse_input(input)
}

#[aoc(day8, part1)]
pub fn wrapper_p1(input: &[[Seg7; 14]]) -> usize {
    solve_p1(input)
}

#[aoc(day8, part2)]
pub fn wrapper_p2(input: &[[Seg7; 14]]) -> usize {
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
