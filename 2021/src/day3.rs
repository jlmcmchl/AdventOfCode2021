use aoc_runner_derive::{aoc, aoc_generator};
use nalgebra::DMatrix;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> DMatrix<usize> {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();

    let mut mat = DMatrix::<usize>::zeros(rows, cols);

    input.lines().enumerate().for_each(|(i, row)| {
        row.as_bytes().iter().enumerate().for_each(|(j, chr)| {
            mat[(i, j)] = *chr as usize - 48;
        });
    });

    mat
}

#[aoc(day3, part1)]
pub fn solve_p1(input: &DMatrix<usize>) -> u64 {
    let (gamma, epsilon) =
        input
            .transpose()
            .row_iter()
            .fold((0u32, 0u32), |(gamma, epsilon), row| {
                let ones: usize = row.iter().sum();
                let zeros = row.len() - ones;

                if ones > zeros {
                    (gamma << 1 | 1, epsilon << 1)
                } else {
                    (gamma << 1, epsilon << 1 | 1)
                }
            });

    gamma as u64 * epsilon as u64
}

fn row_to_number(row: DMatrix<usize>) -> usize {
    row.iter().fold(0, |agg, i| agg << 1 | i)
}

fn get_rating(input: &DMatrix<usize>, inverted: bool) -> usize {
    let (_, cols) = input.shape();

    let mut input = input.clone();

    for i in 0..cols {
        let col = input.column(i);
        let ones: usize = col.iter().sum();
        let zeros = col.shape().0 - ones;

        let take_zeros = (zeros > ones) ^ inverted;

        input = DMatrix::from_rows(
            &input
                .row_iter()
                .filter(|row| (row[(0, i)] == 0 && take_zeros) || (row[(0, i)] == 1 && !take_zeros))
                .collect::<Vec<_>>(),
        );

        if input.shape().0 == 1 {
            break;
        }
    }

    // turn input into number again
    row_to_number(input)
}

#[aoc(day3, part2)]
pub fn solve_p2(input: &DMatrix<usize>) -> usize {
    let o2 = get_rating(input, false);
    let co2 = get_rating(input, true);

    o2 * co2
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let input =
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";

        let parsed_input = super::input_generator(input);
        println!("{}", parsed_input);
        assert_eq!(198, super::solve_p1(&parsed_input));
        assert_eq!(230, super::solve_p2(&parsed_input));
    }
}
