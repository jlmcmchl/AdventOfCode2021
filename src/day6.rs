use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

fn incubate(input: &[u8], days: u16) -> usize {
    let mut fish: HashMap<u8, usize> = input.iter().fold(HashMap::new(), |mut map, fish| {
        map.entry(*fish)
            .and_modify(|count| *count += 1)
            .or_insert(1);
        map
    });
    for _ in 0..days {
        fish = fish
            .iter_mut()
            .flat_map(|(k, v)| {
                if *k == 0 {
                    vec![(6, *v), (8, *v)]
                } else {
                    vec![(k - 1, *v)]
                }
            })
            .fold(HashMap::new(), |mut map, (age, count)| {
                map.entry(age)
                    .and_modify(|cnt| *cnt += count)
                    .or_insert(count);
                map
            });
    }

    println!("{:?}", fish);

    fish.iter().map(|(_, v)| v).sum()
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<u8> {
    input.split(',').map(|l| l.parse().unwrap()).collect()
}

#[aoc(day6, part1)]
pub fn wrapper_p1(input: &[u8]) -> usize {
    incubate(input, 80)
}

#[aoc(day6, part2)]
pub fn wrapper_p2(input: &[u8]) -> usize {
    incubate(input, 256)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let input = "3,4,3,1,2";

        let parsed_input = super::input_generator(input);
        assert_eq!(26, super::incubate(&parsed_input, 18));
        assert_eq!(5934, super::incubate(&parsed_input, 80));
        assert_eq!(26984457539, super::incubate(&parsed_input, 256));
    }
}
