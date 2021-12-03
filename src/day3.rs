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
    let (gamma, epsilon) = input.transpose().row_iter().enumerate().fold(
        (0u32, 0u32),
        |(gamma, epsilon), (id, row)| {
            let ones: usize = row.iter().sum();
            let zeros = row.len() - ones;

            if ones > zeros {
                (gamma << 1 | 1, epsilon << 1)
            } else {
                (gamma << 1, epsilon << 1 | 1)
            }
        },
    );

    gamma as u64 * epsilon as u64
}

// #[aoc(day3, part2)]
pub fn solve_p2(input: &DMatrix<usize>) -> u128 {
    0
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
