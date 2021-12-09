use aoc_runner_derive::{aoc, aoc_generator};
use nalgebra::DMatrix;

fn parse_input(input: &str) -> DMatrix<u8> {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();

    let mut mat = DMatrix::<u8>::zeros(rows + 2, cols + 2);
    mat.fill(9);

    input.lines().enumerate().for_each(|(i, row)| {
        row.as_bytes().iter().enumerate().for_each(|(j, chr)| {
            mat[(i + 1, j + 1)] = *chr - 48;
        });
    });

    mat
}

fn surrounding((x, y): (usize, usize)) -> Vec<(usize, usize)> {
    vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
}

fn low_point(matrix: &DMatrix<u8>, coord: (usize, usize)) -> bool {
    let val = matrix[coord];

    surrounding(coord)
        .into_iter()
        .all(|coord| val < matrix[coord])
}

fn get_low_points(input: &DMatrix<u8>) -> Vec<(usize, usize)> {
    let (rowcnt, colcnt) = input.shape();

    let mut bottoms = Vec::new();
    for i in 1..rowcnt - 1 {
        for j in 1..colcnt - 1 {
            if low_point(input, (i, j)) {
                bottoms.push((i, j));
            }
        }
    }
    bottoms
}

fn solve_p1(input: &DMatrix<u8>) -> usize {
    let bottoms = get_low_points(input);
    bottoms
        .into_iter()
        .map(|coord| input[coord] as usize + 1)
        .sum()
}

fn basin_size(map: &DMatrix<u8>, source: (usize, usize)) -> usize {
    let mut seen = Vec::new();
    let mut last_layer = vec![source];

    while !last_layer.is_empty() {
        let mut next_layer = last_layer
            .iter()
            .flat_map(|coord| {
                surrounding(*coord)
                    .into_iter()
                    .filter(|coord| !seen.contains(coord) && map[*coord] < 9)
            })
            .collect();

        seen.append(&mut last_layer);
        last_layer.append(&mut next_layer);
    }

    seen.sort_unstable();
    seen.dedup();

    seen.len()
}

fn solve_p2(input: &DMatrix<u8>) -> usize {
    let mut basins: Vec<usize> = get_low_points(input)
        .into_iter()
        .map(|coord| basin_size(input, coord))
        .collect();

    basins.sort_unstable();
    basins.reverse();
    basins.iter().take(3).product()
}

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> DMatrix<u8> {
    parse_input(input)
}

#[aoc(day9, part1)]
pub fn wrapper_p1(input: &DMatrix<u8>) -> usize {
    solve_p1(input)
}

#[aoc(day9, part2)]
pub fn wrapper_p2(input: &DMatrix<u8>) -> usize {
    solve_p2(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let input = "2199943210\n3987894921\n9856789892\n8767896789\n9899965678";

        let parsed_input = super::input_generator(input);
        assert_eq!(15, super::solve_p1(&parsed_input));
        assert_eq!(1134, super::solve_p2(&parsed_input));
    }
}
