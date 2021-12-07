use aoc_runner_derive::{aoc, aoc_generator};
use ndarray::{Array2, Array3, Axis, Ix2};

fn parse_bingo_pulls(input: &str) -> Vec<u32> {
    input.split(',').map(|i| i.parse().unwrap()).collect()
}

fn parse_bingo_boards(input: &str) -> Array3<u32> {
    let input = input.replace("  ", " ");

    Array3::<u32>::from(
        input
            .split("\n\n")
            .map::<[[u32; 5]; 5], _>(|board| {
                board
                    .lines()
                    .map(|row| {
                        row.trim()
                            .split(' ')
                            .map(|i| i.parse().unwrap())
                            .collect::<Vec<_>>()
                            .try_into()
                            .unwrap()
                    })
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<_>>(),
    )
}

fn winning_board(board: &Array2<u32>) -> bool {
    for row in board.rows() {
        if row.iter().all(|i| *i == 0) {
            return true;
        }
    }

    for col in board.columns() {
        if col.iter().all(|i| *i == 0) {
            return true;
        }
    }

    false
}

fn any_winning_board(boards: &Array3<u32>) -> Option<(usize, Array2<u32>)> {
    let shape = boards.shape();

    for board_id in 0..shape[0] {
        let board = boards.select(Axis(0), &[board_id]);

        let board = board
            .into_shape(&shape[1..])
            .and_then(|board| board.into_dimensionality::<Ix2>())
            .unwrap();

        if winning_board(&board) {
            return Some((board_id, board));
        }
    }

    None
}

fn score_board(board: &Array2<u32>, final_pull: &u32) -> u32 {
    board.iter().sum::<u32>() * final_pull
}

fn solve_p1((pulls, boards): &(Vec<u32>, Array3<u32>)) -> u32 {
    let mut boards = boards.clone();

    for pull in pulls {
        boards.iter_mut().for_each(|i| {
            if *i == *pull {
                *i = 0;
            }
        });

        if let Some((_, board)) = any_winning_board(&boards) {
            return score_board(&board, pull);
        }
    }

    0
}

fn solve_p2((pulls, boards): &(Vec<u32>, Array3<u32>)) -> u32 {
    let mut boards = boards.clone();

    println!("{:?}", pulls);

    let mut last_score = 0;

    for pull in pulls {
        println!("{}", pull);
        boards.iter_mut().for_each(|i| {
            if *i == *pull {
                *i = 0;
            }
        });

        while let Some((board_id, board)) = any_winning_board(&boards) {
            println!(
                "winning pull: {} = {} {}",
                score_board(&board, pull),
                pull,
                board
            );

            last_score = score_board(&board, pull);

            boards = boards.select(
                Axis(0),
                &(0..boards.shape()[0])
                    .filter(|i| *i != board_id)
                    .collect::<Vec<_>>(),
            )
        }
    }

    last_score
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> (Vec<u32>, Array3<u32>) {
    let (pulls, boards) = input.split_once("\n\n").unwrap();

    (parse_bingo_pulls(pulls), parse_bingo_boards(boards))
}

#[aoc(day4, part1)]
pub fn wrapper_p1(input: &(Vec<u32>, Array3<u32>)) -> u32 {
    solve_p1(input)
}

#[aoc(day4, part2)]
pub fn wrapper_p2(input: &(Vec<u32>, Array3<u32>)) -> u32 {
    solve_p2(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\n22 13 17 11  0\n 8  2 23  4 24\n21  9 14 16  7\n 6 10  3 18  5\n 1 12 20 15 19\n\n 3 15  0  2 22\n 9 18 13 17  5\n19  8  7 25 23\n20 11 10 24  4\n14 21 16 12  6\n\n14 21 17 24  4\n10 16 15  9 19\n18  8 23 26 20\n22 11 13  6  5\n 2  0 12  3  7";

        let parsed_input = super::input_generator(input);
        assert_eq!(4512, super::solve_p1(&parsed_input));
        assert_eq!(1924, super::solve_p2(&parsed_input));
    }
}
