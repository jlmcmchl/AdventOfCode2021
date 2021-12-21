use aoc_runner_derive::{aoc, aoc_generator};

fn position(score: usize) -> usize {
    (score - 1) % 10 + 1
}

fn roll(rolls: usize) -> usize {
    (rolls - 1) % 100 + 1
}

fn triangle(rolls: usize) -> usize {
    rolls * (rolls + 1) / 2
}

#[derive(Debug)]
struct Player {
    position: usize,
    id: u8,
    score: usize,
}

impl Player {
    fn new(start: usize, id: u8) -> Self {
        Player { position: start, id, score: 0 }
    }

    #[allow(unused)]
    fn print_state(&self, rolls: usize) {
        println!("Player {} rolls {:?} and moves to space {} for a total score of {}", self.id, (rolls-2)..=(rolls), self.position, self.score);
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
            break
        }

        let current_step = second.position + high_rolls_score - mid_rolls_score;
        second.position = position(current_step);
        second.score += position(current_step);

        rolls += 3;

        // second.print_state(rolls);
        if second.score >= 1000 {
            break
        }

        // std::thread::sleep(std::time::Duration::from_secs(1));
    }

    rolls
}

fn parse_input(input: &str) -> Vec<usize> {
    input.lines().map(|line| line.bytes().last().unwrap() as usize - 48).collect()
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

fn solve_p2(target: &[usize]) -> usize {
    Default::default()
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
    solve_p2(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

        let input = "Player 1 starting position: 4\nPlayer 2 starting position: 8";

        let parsed_input = super::parse_input(input);
        assert_eq!(739785, super::solve_p1(&parsed_input));
        assert_eq!(0, super::solve_p2(&parsed_input));
    }
}
