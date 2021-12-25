use aoc_runner_derive::{aoc, aoc_generator};
use ndarray::Array2;

fn parse_input(input: &str) -> ([u8; 512], Array2<u8>) {
    let mut algorithm = [0; 512];
    let (alg, image) = input.split_once("\n\n").unwrap();
    alg.bytes()
        .enumerate()
        .filter(|(_, chr)| *chr == b'#')
        .for_each(|(ind, _)| {
            algorithm[ind] = 1;
        });

    let image_rows = image.lines().count();
    let image_cols = image.lines().next().unwrap().len();

    let image_vec = image
        .bytes()
        .filter(|chr| *chr != b'\n')
        .map(|chr| if chr == b'#' { 1 } else { 0 })
        .collect();

    let image = Array2::<u8>::from_shape_vec((image_cols, image_rows), image_vec).unwrap();

    (algorithm, image)
}

fn print_image(image: &Array2<u8>) {
    for row in image.rows() {
        for cell in row.iter() {
            if *cell == 1 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn around(row: usize, col: usize) -> Vec<(usize, usize)> {
    vec![
        (row - 1, col - 1),
        (row - 1, col),
        (row - 1, col + 1),
        (row, col - 1),
        (row, col),
        (row, col + 1),
        (row + 1, col - 1),
        (row + 1, col),
        (row + 1, col + 1),
    ]
}

fn kernel(image: &Array2<u8>, row: usize, col: usize) -> usize {
    around(row, col)
        .iter()
        .fold(0, |agg, coord| agg << 1 | image[*coord] as usize)
}

fn expand(image: &Array2<u8>, unseen_state: u8) -> Array2<u8> {
    let image_shape = image.shape();
    Array2::from_shape_fn((image_shape[0] + 4, image_shape[1] + 4), |(row, col)| {
        if row < 2 || row > image_shape[0] + 1 || col < 2 || col > image_shape[1] + 1 {
            unseen_state
        } else if row > 1 && row < image_shape[0] + 2 && col > 1 && col < image_shape[1] + 2 {
            image[(row - 2, col - 2)]
        } else {
            0
        }
    })
}

fn contract(image: &Array2<u8>) -> Array2<u8> {
    let image_shape = image.shape();
    Array2::from_shape_fn((image_shape[0] - 2, image_shape[1] - 2), |(row, col)| {
        image[(row + 1, col + 1)]
    })
}

fn enhance(algorithm: &[u8; 512], image: &Array2<u8>, unseen_state: u8) -> Array2<u8> {
    let ref_image = expand(image, unseen_state);
    let shape = ref_image.shape();

    let mut new_image = Array2::from_shape_simple_fn((shape[0], shape[1]), || 0);

    // inside rows
    for row in 1..(shape[0] - 1) {
        // inside cols
        for col in 1..(shape[1] - 1) {
            new_image[(row, col)] = algorithm[kernel(&ref_image, row, col)];
        }
    }

    // keep just inside rows, cols
    contract(&new_image)
}

fn steps(algorithm: &[u8; 512], image: &Array2<u8>, steps: usize, debug: bool) -> Array2<u8> {
    let mut infinite_state = 0;
    let mut image = image.clone();

    for _step in 0..steps {
        // println!("step {}: infinite @ {}", _step, infinite_state);
        image = enhance(algorithm, &image, infinite_state);
        infinite_state = algorithm[(infinite_state..=infinite_state)
            .cycle()
            .take(9)
            .fold(0usize, |agg, v| agg << 1 | v as usize)];

        if debug {
            print_image(&image);
        }
    }

    image
}

fn solve_p1((algorithm, image): &([u8; 512], Array2<u8>)) -> usize {
    // println!("{:?}", image.shape());
    // print_image(image);

    let image = steps(algorithm, image, 2, false);

    // print_image(&image);

    image.iter().map(|i| *i as usize).sum()
}

fn solve_p2((algorithm, image): &([u8; 512], Array2<u8>)) -> usize {
    let image = steps(algorithm, image, 50, false);

    // print_image(&image);

    image.iter().map(|i| *i as usize).sum()
}

#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> ([u8; 512], Array2<u8>) {
    parse_input(input)
}

#[aoc(day20 part1)]
pub fn wrapper_p1(input: &([u8; 512], Array2<u8>)) -> usize {
    solve_p1(input)
}

#[aoc(day20, part2)]
pub fn wrapper_p2(input: &([u8; 512], Array2<u8>)) -> usize {
    solve_p2(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let input = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#\n\n#..#.\n#....\n##..#\n..#..\n..###";

        let parsed_input = super::input_generator(input);
        assert_eq!(35, super::solve_p1(&parsed_input));
        assert_eq!(3351, super::solve_p2(&parsed_input));
    }
}
