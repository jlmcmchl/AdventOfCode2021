use aoc_runner_derive::{aoc, aoc_generator};
use nalgebra::SMatrix;

fn surrounding((i, j): (usize, usize), (rows, cols): (usize, usize)) -> Vec<(usize, usize)> {
    let mut vec = Vec::new();
    if i > 0 {
        if j > 0 {
            vec.push((i - 1, j - 1));
        }
        vec.push((i - 1, j));
        if j < cols - 1 {
            vec.push((i - 1, j + 1));
        }
    }

    if j > 0 {
        vec.push((i, j - 1));
    }

    if j < cols - 1 {
        vec.push((i, j + 1));
    }

    if i < rows - 1 {
        if j > 0 {
            vec.push((i + 1, j - 1));
        }
        vec.push((i + 1, j));
        if j < cols - 1 {
            vec.push((i + 1, j + 1));
        }
    }

    vec
}

fn step(input: &SMatrix<u8, 10, 10>) -> (SMatrix<u8, 10, 10>, usize) {
    let (rows, cols) = input.shape();
    let mut flash_mat = input.add_scalar(1);

    let mut last_flashes = 1;
    let mut flashes = 0;
    let mut flashers = Vec::with_capacity(100);

    while flashes != last_flashes || (last_flashes == 0 && flashes != 0) {
        last_flashes = flashes;
        for i in 0..rows {
            for j in 0..cols {
                let val = flash_mat[(i, j)];
                if val > 9 && !flashers.contains(&(i, j)) {
                    // add 1 to surrounding
                    for coord in surrounding((i, j), (rows, cols)) {
                        flash_mat[coord] += 1;
                    }
                    flashes += 1;
                    flashers.push((i, j));
                }
            }
        }
    }

    for i in 0..rows {
        for j in 0..cols {
            let val = flash_mat[(i, j)];
            if val > 9 {
                // add 1 to surrounding
                flash_mat[(i, j)] = 0;
            }
        }
    }

    (flash_mat, last_flashes)
}

fn steps(input: &SMatrix<u8, 10, 10>, steps: usize) -> (SMatrix<u8, 10, 10>, usize) {
    (0..steps).fold((*input, 0), |(mat, flashcnt), _| {
        let (next_mat, flashes) = step(&mat);
        (next_mat, flashcnt + flashes)
    })
}

fn flash_count(input: &SMatrix<u8, 10, 10>, stepcnt: usize) -> usize {
    steps(input, stepcnt).1
}

fn parse_input(input: &str) -> SMatrix<u8, 10, 10> {
    let mut mat = SMatrix::<u8, 10, 10>::zeros();

    input.lines().enumerate().for_each(|(i, row)| {
        row.trim()
            .as_bytes()
            .iter()
            .enumerate()
            .for_each(|(j, chr)| {
                mat[(i, j)] = *chr - 48;
            });
    });

    mat
}

fn solve_p1(input: &SMatrix<u8, 10, 10>) -> usize {
    flash_count(input, 100)
}

fn solve_p2(input: &SMatrix<u8, 10, 10>) -> usize {
    let mut current_mat = *input;

    for iter in 0.. {
        let (next_mat, _) = step(&current_mat);
        if next_mat.iter().fold(0, |agg, i| agg + *i as usize) == 0 {
            return iter + 1;
        }
        current_mat = next_mat;
    }

    0
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> SMatrix<u8, 10, 10> {
    parse_input(input)
}

#[aoc(day11, part1)]
pub fn wrapper_p1(input: &SMatrix<u8, 10, 10>) -> usize {
    solve_p1(input)
}

#[aoc(day11, part2)]
pub fn wrapper_p2(input: &SMatrix<u8, 10, 10>) -> usize {
    solve_p2(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let input = "5483143223\n2745854711\n5264556173\n6141336146\n6357385478\n4167524645\n2176841721\n6882881134\n4846848554\n5283751526";
        let parsed_input = super::parse_input(input);

        let results = vec![
        ("6594254334\n3856965822\n6375667284\n7252447257\n7468496589\n5278635756\n3287952832\n7993992245\n5957959665\n6394862637", 1),
        ("8807476555\n5089087054\n8597889608\n8485769600\n8700908800\n6600088989\n6800005943\n0000007456\n9000000876\n8700006848", 2),
        ("0050900866\n8500800575\n9900000039\n9700000041\n9935080063\n7712300000\n7911250009\n2211130000\n0421125000\n0021119000", 3),
        ("2263031977\n0923031697\n0032221150\n0041111163\n0076191174\n0053411122\n0042361120\n5532241122\n1532247211\n1132230211", 4),
        ("4484144000\n2044144000\n2253333493\n1152333274\n1187303285\n1164633233\n1153472231\n6643352233\n2643358322\n2243341322", 5),
        ("5595255111\n3155255222\n3364444605\n2263444496\n2298414396\n2275744344\n2264583342\n7754463344\n3754469433\n3354452433", 6),
        ("6707366222\n4377366333\n4475555827\n3496655709\n3500625609\n3509955566\n3486694453\n8865585555\n4865580644\n4465574644", 7),
        ("7818477333\n5488477444\n5697666949\n4608766830\n4734946730\n4740097688\n6900007564\n0000009666\n8000004755\n6800007755", 8),
        ("9060000644\n7800000976\n6900000080\n5840000082\n5858000093\n6962400000\n8021250009\n2221130009\n9111128097\n7911119976", 9),
        ("0481112976\n0031112009\n0041112504\n0081111406\n0099111306\n0093511233\n0442361130\n5532252350\n0532250600\n0032240000", 10),
        ("3936556452\n5686556806\n4496555690\n4448655580\n4456865570\n5680086577\n7000009896\n0000000344\n6000000364\n4600009543", 20),
        ("0643334118\n4253334611\n3374333458\n2225333337\n2229333338\n2276733333\n2754574565\n5544458511\n9444447111\n7944446119", 30),
        ("6211111981\n0421111119\n0042111115\n0003111115\n0003111116\n0065611111\n0532351111\n3322234597\n2222222976\n2222222762", 40),
        ("9655556447\n4865556805\n4486555690\n4458655580\n4574865570\n5700086566\n6000009887\n8000000533\n6800000633\n5680000538", 50),
        ("2533334200\n2743334640\n2264333458\n2225333337\n2225333338\n2287833333\n3854573455\n1854458611\n1175447111\n1115446111", 60),
        ("8211111164\n0421111166\n0042111114\n0004211115\n0000211116\n0065611111\n0532351111\n7322235117\n5722223475\n4572222754", 70),
        ("1755555697\n5965555609\n4486555680\n4458655580\n4570865570\n5700086566\n7000008666\n0000000990\n0000000800\n0000000000", 80),
        ("7433333522\n2643333522\n2264333458\n2226433337\n2222433338\n2287833333\n2854573333\n4854458333\n3387779333\n3333333333", 90),
        ("0397666866\n0749766918\n0053976933\n0004297822\n0004229892\n0053222877\n0532222966\n9322228966\n7922286866\n6789998766", 100)];

        for (expect, rounds) in results {
            let expected_mat = super::parse_input(expect);
            assert_eq!(super::steps(&parsed_input, rounds).0, expected_mat);
        }

        assert_eq!(204, super::flash_count(&parsed_input, 10));
        assert_eq!(1656, super::flash_count(&parsed_input, 100));

        assert_eq!(195, super::solve_p2(&parsed_input));
    }
}
