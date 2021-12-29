use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use lazy_static::lazy_static;
use nalgebra::{matrix, Matrix3, Vector3};
use ndarray::Array2;

#[derive(Debug)]
pub struct Scanner {
    id: usize,
    position: Option<Vector3<isize>>,
    rotation: Option<Matrix3<isize>>,
    beacons: Vec<Beacon>,
    beacon_graph: Array2<f64>,
}

#[derive(Debug)]
pub struct Beacon {
    offset: Vector3<isize>,
}

lazy_static! {
    static ref ORIENTATIONS: Vec<Matrix3<isize>> = vec![
        //      +x  +y  +z
        matrix![ 1,  0,  0;
                 0,  1,  0;
                 0,  0,  1],
        //      +x  +z  -y
        matrix![ 1,  0,  0;
                 0,  0,  1;
                 0, -1,  0],
        //      +x  -y  -z
        matrix![ 1,  0,  0;
                 0, -1,  0;
                 0,  0, -1],
        //      +x  -z  +y
        matrix![ 1,  0,  0;
                 0,  0, -1;
                 0,  1,  0],

        //      -x  +y  -z
        matrix![-1,  0,  0;
                 0,  1,  0;
                 0,  0, -1],
        //      -x  -z  -y
        matrix![-1,  0,  0;
                 0,  0, -1;
                 0, -1,  0],
        //      -x  -y  +z
        matrix![-1,  0,  0;
                 0, -1,  0;
                 0,  0,  1],
        //      -x  +z  +y
        matrix![-1,  0,  0;
                 0,  0,  1;
                 0,  1,  0],

        //      +y  -x  +z
        matrix![ 0,  1,  0;
                -1,  0,  0;
                 0,  0,  1],
        //      +y  +z  +x
        matrix![ 0,  1,  0;
                 0,  0,  1;
                 1,  0,  0],
        //      +y  +x  -z
        matrix![ 0,  1,  0;
                 1,  0,  0;
                 0,  0, -1],
        //      +y  -z  -x
        matrix![ 0,  1,  0;
                 0,  0, -1;
                 1,  0,  0],

        //      -y  +x  +z
        matrix![ 0, -1,  0;
                 1,  0,  0;
                 0,  0,  1],
        //      -y  +z  -x
        matrix![ 0, -1,  0;
                 0,  0,  1;
                -1,  0,  0],
        //      -y  -x  -z
        matrix![ 0, -1,  0;
                -1,  0,  0;
                 0,  0, -1],
        //      -y  -z  +x
        matrix![ 0, -1,  0;
                 0,  0, -1;
                 1,  0,  0],

        //      +z  +y  -x
        matrix![ 0,  0,  1;
                 0,  1,  0;
                -1,  0,  0],
        //      +z  -x  -y
        matrix![ 0,  0,  1;
                -1,  0,  0;
                 0, -1,  0],
        //      +z  -y  +x
        matrix![ 0,  0,  1;
                 0, -1,  0;
                 1,  0,  0],
        //      +z  +x  +y
        matrix![ 0,  0,  1;
                 1,  0,  0;
                 0,  1,  0],

        //      -z  -x  +y
        matrix![ 0,  0, -1;
                -1,  0,  0;
                 0,  1,  0],
        //      -z  +y  +x
        matrix![ 0,  0, -1;
                 0,  1,  0;
                 1,  0,  0],
        //      -z  +x  -y
        matrix![ 0,  0, -1;
                 1,  0,  0;
                 0, -1,  0],
        //      -z  -y  -x
        matrix![ 0,  0, -1;
                 0, -1,  0;
                -1,  0,  0],

    ];
}

fn prim(beacons: &Array2<f64>) -> Array2<f64> {
    let shape = beacons.shape();
    let mut vertices = vec![0];
    let mut mst = Array2::zeros((shape[0], shape[1]));

    while vertices.len() < shape[0] {
        vertices.iter().map(|vertex| {
            beacons
                .row(*vertex)
                .columns()
                .into_iter()
                .enumerate()
                .filter(|(id, dist)| vertex != id)
                .map(|(id, dist)| (vertex, id, dist.first()))
                .min_by_key(|(_, _, dist)| dist)
        });
    }

    mst
}

fn reorient(scanner: &Scanner, orientation: &Matrix3<isize>) -> Scanner {
    Scanner {
        id: scanner.id,
        position: scanner.position,
        rotation: Some(orientation.clone()),
        beacons: scanner
            .beacons
            .iter()
            .map(|beacon| {
                let mut new_position = Vector3::zeros();
                new_position.gemv(1, orientation, &beacon.offset, 0);
                Beacon {
                    offset: new_position,
                }
            })
            .collect(),
        beacon_graph: scanner.beacon_graph.clone(),
    }
}

fn find_right_orientation(
    scanner: &Scanner,
    target: &Scanner,
    test: impl Fn(&Scanner, &Scanner) -> bool,
) -> Option<Scanner> {
    ORIENTATIONS
        .iter()
        .map(|orient| reorient(scanner, orient))
        .filter(|scanner| test(scanner, target))
        .next()
}

fn are_linked(
    scanner: &Scanner,
    target: &Scanner,
    test: impl Fn(&Scanner, &Scanner) -> bool,
) -> Option<Scanner> {
    ORIENTATIONS
        .iter()
        .map(|orient| reorient(scanner, orient))
        .filter(|scanner| test(scanner, target))
        .next()
}

fn generate_beacon_graph(beacons: &[Beacon]) -> Array2<f64> {
    let beacon_count = beacons.len();

    let mut beacon_graph = Array2::from_elem((beacon_count, beacon_count), 0.);

    beacons
        .iter()
        .enumerate()
        .combinations(2)
        .map(|pair| (pair[0].0, pair[1].0, distance(pair[0].1, pair[1].1)))
        .for_each(|(i, j, dist)| {
            let dist = (dist as f64).sqrt();
            beacon_graph[(i, j)] = dist;
            beacon_graph[(j, i)] = dist;
        });

    beacon_graph
}

fn parse_input(input: &str) -> Vec<Scanner> {
    input
        .split("\n\n")
        .map(|scanner| {
            let mut scanner_lines = scanner.lines();
            let header = scanner_lines.next().unwrap();
            let id = header.split(' ').nth(2).unwrap().parse().unwrap();
            let beacons = scanner_lines
                .map(|line| {
                    let position = line
                        .split(',')
                        .map(|s| s.parse().unwrap())
                        .collect::<Vec<_>>();
                    let point = Vector3::new(position[0], position[1], position[2]);
                    Beacon { offset: point }
                })
                .collect::<Vec<_>>();

            let beacon_graph = generate_beacon_graph(&beacons);

            Scanner {
                id,
                position: None,
                rotation: None,
                beacons,
                beacon_graph,
            }
        })
        .collect()
}

fn distance(first: &Beacon, second: &Beacon) -> isize {
    first
        .offset
        .iter()
        .zip(&second.offset)
        .map(|(a, b)| (b - a).pow(2))
        .sum()
}

fn get_distances(scanner: &Scanner) -> HashMap<isize, usize> {
    scanner
        .beacons
        .iter()
        .combinations(2)
        .map(|pair| distance(pair[0], pair[1]))
        .fold(HashMap::new(), |mut counts, dist| {
            counts.entry(dist).and_modify(|v| *v += 1usize).or_insert(1);
            counts
        })
}

fn init_graph(scanners: &[Scanner]) -> Array2<usize> {
    let scanner_count = scanners.len();

    Array2::from_elem((scanner_count, scanner_count), 0)
}

fn build_graph(scanner_graph: &mut Array2<usize>, scanner_maps: &[HashMap<isize, usize>]) {
    scanner_maps
        .iter()
        .enumerate()
        .combinations(2)
        .for_each(|combo| {
            let (first_id, first_dists) = combo[0];
            let (second_id, second_dists) = combo[1];

            let overlap: usize = first_dists
                .iter()
                .map(|(dist, first_count)| {
                    let second_count = if second_dists.contains_key(dist) {
                        second_dists[dist]
                    } else {
                        0
                    };

                    (*first_count).min(second_count)
                })
                .sum();

            scanner_graph[(first_id, second_id)] = overlap;
            scanner_graph[(second_id, first_id)] = overlap;
        });
}

fn count_beacons(graph: &Array2<usize>, scanners: &[Scanner], overlap_threshold: usize) -> usize {
    let mut all_beacon_records: usize = scanners.iter().map(|scanner| scanner.beacons.len()).sum();

    for i in 0..scanners.len() {
        for j in (i + 1)..scanners.len() {
            if graph[(i, j)] >= (overlap_threshold * (overlap_threshold - 1)) / 2 {
                all_beacon_records -= overlap_threshold;
            }
        }
    }

    all_beacon_records
}

fn solve_p1(scanners: &[Scanner], overlap_threshold: usize) -> usize {
    let mut scanner_graph = init_graph(scanners);

    let scanner_maps = scanners
        .iter()
        .map(|scanner| get_distances(scanner))
        .collect::<Vec<_>>();

    build_graph(&mut scanner_graph, &scanner_maps);

    for i in 0..scanners.len() {
        for j in 0..scanners.len() {
            print!("{}\t", scanner_graph[(i, j)]);
        }
        println!()
    }
    // println!("{}", scanner_graph);

    count_beacons(&scanner_graph, scanners, overlap_threshold)
}

fn solve_p2(target: &[Scanner]) -> usize {
    Default::default()
}

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> Vec<Scanner> {
    parse_input(input)
}

#[aoc(day19, part1)]
pub fn wrapper_p1(input: &[Scanner]) -> usize {
    solve_p1(input, 12)
}

#[aoc(day19, part2)]
pub fn wrapper_p2(input: &[Scanner]) -> usize {
    solve_p2(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn two_scanners() {
        let input =
            "--- scanner 0 ---\n0,2,0\n4,1,0\n3,3,0\n\n--- scanner 1 ---\n-1,-1,0\n-5,0,0\n-2,1,0";
        let parsed_input = super::input_generator(input);

        // println!("{:?}", parsed_input);

        assert_eq!(3, super::solve_p1(&parsed_input, 3));
        assert_eq!(0, super::solve_p2(&parsed_input));
    }

    #[test]
    fn one_scanner_many_orientation() {
        let input = "--- scanner 0 ---\n-1,-1,1\n-2,-2,2\n-3,-3,3\n-2,-3,1\n5,6,-4\n8,0,7\n\n--- scanner 0 ---\n1,-1,1\n2,-2,2\n3,-3,3\n2,-1,3\n-5,4,-6\n-8,-7,0\n\n--- scanner 0 ---\n-1,-1,-1\n-2,-2,-2\n-3,-3,-3\n-1,-3,-2\n4,6,5\n-7,0,8\n\n--- scanner 0 ---\n1,1,-1\n2,2,-2\n3,3,-3\n1,3,-2\n-4,-6,5\n7,0,8\n\n--- scanner 0 ---\n1,1,1\n2,2,2\n3,3,3\n3,1,2\n-6,-4,-5\n0,7,-8";
        let parsed_input = super::input_generator(input);

        assert_eq!(6, super::solve_p1(&parsed_input, 6));
        assert_eq!(0, super::solve_p2(&parsed_input));
    }

    #[test]
    fn large_test() {
        let input = "--- scanner 0 ---\n404,-588,-901\n528,-643,409\n-838,591,734\n390,-675,-793\n-537,-823,-458\n-485,-357,347\n-345,-311,381\n-661,-816,-575\n-876,649,763\n-618,-824,-621\n553,345,-567\n474,580,667\n-447,-329,318\n-584,868,-557\n544,-627,-890\n564,392,-477\n455,729,728\n-892,524,684\n-689,845,-530\n423,-701,434\n7,-33,-71\n630,319,-379\n443,580,662\n-789,900,-551\n459,-707,401\n\n--- scanner 1 ---\n686,422,578\n605,423,415\n515,917,-361\n-336,658,858\n95,138,22\n-476,619,847\n-340,-569,-846\n567,-361,727\n-460,603,-452\n669,-402,600\n729,430,532\n-500,-761,534\n-322,571,750\n-466,-666,-811\n-429,-592,574\n-355,545,-477\n703,-491,-529\n-328,-685,520\n413,935,-424\n-391,539,-444\n586,-435,557\n-364,-763,-893\n807,-499,-711\n755,-354,-619\n553,889,-390\n\n--- scanner 2 ---\n649,640,665\n682,-795,504\n-784,533,-524\n-644,584,-595\n-588,-843,648\n-30,6,44\n-674,560,763\n500,723,-460\n609,671,-379\n-555,-800,653\n-675,-892,-343\n697,-426,-610\n578,704,681\n493,664,-388\n-671,-858,530\n-667,343,800\n571,-461,-707\n-138,-166,112\n-889,563,-600\n646,-828,498\n640,759,510\n-630,509,768\n-681,-892,-333\n673,-379,-804\n-742,-814,-386\n577,-820,562\n\n--- scanner 3 ---\n-589,542,597\n605,-692,669\n-500,565,-823\n-660,373,557\n-458,-679,-417\n-488,449,543\n-626,468,-788\n338,-750,-386\n528,-832,-391\n562,-778,733\n-938,-730,414\n543,643,-506\n-524,371,-870\n407,773,750\n-104,29,83\n378,-903,-323\n-778,-728,485\n426,699,580\n-438,-605,-362\n-469,-447,-387\n509,732,623\n647,635,-688\n-868,-804,481\n614,-800,639\n595,780,-596\n\n--- scanner 4 ---\n727,592,562\n-293,-554,779\n441,611,-461\n-714,465,-776\n-743,427,-804\n-660,-479,-426\n832,-632,460\n927,-485,-438\n408,393,-506\n466,436,-512\n110,16,151\n-258,-428,682\n-393,719,612\n-211,-452,876\n808,-476,-593\n-575,615,604\n-485,667,467\n-680,325,-822\n-627,-443,-432\n872,-547,-609\n833,512,582\n807,604,487\n839,-516,451\n891,-625,532\n-652,-548,-490\n30,-46,-14";
        let parsed_input = super::input_generator(input);

        assert_eq!(79, super::solve_p1(&parsed_input, 12));
        assert_eq!(0, super::solve_p2(&parsed_input));
    }
}
