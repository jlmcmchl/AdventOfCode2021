use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use lazy_static::lazy_static;

fn position(score: usize) -> usize {
    (score - 1) % 10 + 1
}

fn roll(rolls: usize) -> usize {
    (rolls - 1) % 100 + 1
}

fn triangle(rolls: usize) -> usize {
    rolls * (rolls + 1) / 2
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Player {
    position: usize,
    score: usize,
    id: u8,
}

impl Player {
    fn new(start: usize, id: u8) -> Self {
        Player {
            position: start,
            id,
            score: 0,
        }
    }

    #[allow(unused)]
    fn print_state(&self, rolls: usize) {
        println!(
            "Player {} rolls {:?} and moves to space {} for a total score of {}",
            self.id,
            (rolls - 2)..=(rolls),
            self.position,
            self.score
        );
    }
}

fn score_players(first: &mut Player, second: &mut Player) -> usize {
    let mut rolls = 0;

    loop {
        let base_rolls_score = triangle(rolls);
        let mid_rolls_score = triangle(rolls + 3);
        let high_rolls_score = triangle(rolls + 6);

        let current_step = first.position + mid_rolls_score - base_rolls_score;
        first.position = position(current_step);
        first.score += position(current_step);

        rolls += 3;

        // first.print_state(rolls);
        if first.score >= 1000 {
            break;
        }

        let current_step = second.position + high_rolls_score - mid_rolls_score;
        second.position = position(current_step);
        second.score += position(current_step);

        rolls += 3;

        // second.print_state(rolls);
        if second.score >= 1000 {
            break;
        }

        // std::thread::sleep(std::time::Duration::from_secs(1));
    }

    rolls
}

fn parse_input(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| line.bytes().last().unwrap() as usize - 48)
        .collect()
}

fn solve_p1(target: &[usize]) -> usize {
    let mut first = Player::new(target[0], 1);
    let mut second = Player::new(target[1], 2);
    let rolls = score_players(&mut first, &mut second);

    println!("{:?} {:?} {}", first, second, rolls);

    if first.score < second.score {
        first.score * rolls
    } else {
        second.score * rolls
    }
}

/*
3 [1, 1, 1]

4 [1, 1, 2]
4 [1, 2, 1]
4 [2, 1, 1]

5 [1, 1, 3]
5 [1, 2, 2]
5 [1, 3, 1]
5 [2, 1, 2]
5 [2, 2, 1]
5 [3, 1, 1]

6 [1, 2, 3]
6 [1, 3, 2]
6 [2, 1, 3]
6 [2, 2, 2]
6 [2, 3, 1]
6 [3, 1, 2]
6 [3, 2, 1]

7 [1, 3, 3]
7 [2, 2, 3]
7 [2, 3, 2]
7 [3, 1, 3]
7 [3, 2, 2]
7 [3, 3, 1]

8 [2, 3, 3]
8 [3, 2, 3]
8 [3, 3, 2]

9 [3, 3, 3]
*/

lazy_static! {
    static ref DIRAC_ROLLS: Vec<usize> = {
        let dice = 1..=3;
        dice.clone()
            .cartesian_product(dice.clone())
            .cartesian_product(dice)
            .map(|vs| vs.0 .0 + vs.0 .1 + vs.1)
            .collect::<Vec<_>>()
    };
}

fn iter_p2(state: &mut HashMap<(Player, Player), usize>, winning_score: usize) -> (HashMap<(Player, Player), usize>, usize, usize) {
    let mut p1_wins = 0;
    let mut p2_wins = 0;

    let mut new_state = HashMap::new();

    state.iter().for_each(|((p1, p2), &count)| {
        // spawn all new universes
        for p1_roll in DIRAC_ROLLS.iter() {
            // iter player 1
            let mut p1 = p1.clone();
            p1.position = position(p1.position + p1_roll);
            p1.score += p1.position;

            if p1.score >= winning_score {
                // don't add entry, add wins
                p1_wins += count;
                continue;
            }

            for p2_roll in DIRAC_ROLLS.iter() {
                // iter player 2
                let mut p2 = p2.clone();
                p2.position = position(p2.position + p2_roll);
                p2.score += p2.position;

                if p2.score >= winning_score {
                    // don't add entry, add wins
                    p2_wins += count;
                    continue;
                }

                // else add entry
                new_state
                    .entry((p1, p2))
                    .and_modify(|v| *v += count)
                    .or_insert(count);
            }
        }
    });

    (new_state, p1_wins, p2_wins)
}

fn solve_p2(target: &[usize], winning_score: usize) -> (usize, usize) {
    let mut state: HashMap<(Player, Player), usize> = HashMap::new();
    let p1 = Player::new(target[0], 1);
    let p2 = Player::new(target[1], 2);

    state.entry((p1, p2)).or_insert(1);

    let mut p1_wins = 0;
    let mut p2_wins = 0;

    while state.iter().map(|(_, count)| *count).sum::<usize>() > 0 {
        let (new_state, p1, p2) = iter_p2(&mut state, winning_score);
        p1_wins += p1;
        p2_wins += p2;
        state = new_state;
    }

    (p1_wins, p2_wins)
}

#[aoc_generator(day21)]
pub fn input_generator(input: &str) -> Vec<usize> {
    parse_input(input)
}

#[aoc(day21, part1)]
pub fn wrapper_p1(input: &[usize]) -> usize {
    solve_p1(input)
}

#[aoc(day21, part2)]
pub fn wrapper_p2(input: &[usize]) -> usize {
    let (p1, p2) = solve_p2(input, 21);
    if p1 > p2 {
        p1
    } else {
        p2
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let input = "Player 1 starting position: 4\nPlayer 2 starting position: 8";

        let parsed_input = super::parse_input(input);
        assert_eq!(739785, super::solve_p1(&parsed_input));
        assert_eq!((27, 0), super::solve_p2(&parsed_input, 1));
        assert_eq!((183, 156), super::solve_p2(&parsed_input, 2));
        assert_eq!((990, 207), super::solve_p2(&parsed_input, 3));
        assert_eq!((2930, 971), super::solve_p2(&parsed_input, 4));
        assert_eq!((7907, 2728), super::solve_p2(&parsed_input, 5));
        assert_eq!((30498, 7203), super::solve_p2(&parsed_input, 6));
        assert_eq!((127019, 152976), super::solve_p2(&parsed_input, 7));
        assert_eq!((655661, 1048978), super::solve_p2(&parsed_input, 8));
        assert_eq!((4008007, 4049420), super::solve_p2(&parsed_input, 9));
        assert_eq!((18973591, 12657100), super::solve_p2(&parsed_input, 10));
    }
}
