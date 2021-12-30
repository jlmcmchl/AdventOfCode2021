use std::{collections::HashMap, fmt::Debug};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use lazy_static::lazy_static;
use nalgebra::{matrix, Matrix3, Vector3};
use ndarray::Array2;
use pathfinding::prelude::kruskal_indices;

#[derive(Default, Clone, PartialEq)]
pub struct Scanner {
    id: usize,
    position: Option<Vector3<isize>>,
    rotation: Option<Matrix3<isize>>,
    beacons: Vec<Beacon>,
    beacon_graph: Array2<isize>,
}

impl Debug for Scanner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Scanner")
            .field("id", &self.id)
            .field("position", &self.position)
            .field("rotation", &self.rotation)
            .field("beacons", &self.beacons)
            .finish()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Beacon {
    offset: Vector3<isize>,
}

lazy_static! {
    static ref ORIENTATIONS: Vec<Matrix3<isize>> = vec![
        matrix![ 1,  0,  0;
                 0,  1,  0;
                 0,  0,  1],
        matrix![-1,  0,  0;
                 0,  -1,  0;
                 0,  0,  1],
        matrix![-1,  0,  0;
                 0,  1,  0;
                 0,  0, -1],
        matrix![ 1,  0,  0;
                 0, -1,  0;
                 0,  0, -1],

        matrix![-1,  0,  0;
                 0,  0,  1;
                 0,  1,  0],
        matrix![ 1,  0,  0;
                 0,  0,  -1;
                 0,  1,  0],
        matrix![ 1,  0,  0;
                 0,  0,  1;
                 0, -1,  0],
        matrix![-1,  0,  0;
                 0,  0, -1;
                 0, -1,  0],
        
        matrix![ 0, -1,  0;
                 1,  0,  0;
                 0,  0,  1],
        matrix![ 0,  1,  0;
                -1,  0,  0;
                 0,  0,  1],
        matrix![ 0,  1,  0;
                 1,  0,  0;
                 0,  0, -1],
        matrix![ 0, -1,  0;
                -1,  0,  0;
                 0,  0, -1],

        matrix![ 0,  1,  0;
                 0,  0,  1;
                 1,  0,  0],
        matrix![ 0, -1,  0;
                 0,  0, -1;
                 1,  0,  0],
        matrix![ 0, -1,  0;
                 0,  0,  1;
                -1,  0,  0],
        matrix![ 0,  1,  0;
                 0,  0, -1;
                -1,  0,  0],

        matrix![ 0,  0,  1;
                 1,  0,  0;
                 0,  1,  0],
        matrix![ 0,  0, -1;
                -1,  0,  0;
                 0,  1,  0],
        matrix![ 0,  0, -1;
                 1,  0,  0;
                 0, -1,  0],
        matrix![ 0,  0,  1;
                -1,  0,  0;
                 0, -1,  0],
        
        matrix![ 0,  0, -1;
                 0,  1,  0;
                 1,  0,  0],
        matrix![ 0,  0,  1;
                 0, -1,  0;
                 1,  0,  0],
        matrix![ 0,  0,  1;
                 0,  1,  0;
                -1,  0,  0],
        matrix![ 0,  0, -1;
                 0, -1,  0;
                -1,  0,  0],

    ];
}

fn counter_intersection(
    first: &HashMap<Vec<isize>, usize>,
    second: &HashMap<Vec<isize>, usize>,
) -> usize {
    first
        .iter()
        .filter_map(|(key, count)| second.get(key).map(|other_count| other_count.min(count)))
        .sum()
}

fn beacon_graph(scanner: &Scanner) -> Array2<Vec<isize>> {
    let beacon_count = scanner.beacons.len();

    Array2::from_shape_fn((beacon_count, beacon_count), |(i, j)| {
        let first = &scanner.beacons[i];
        let second = &scanner.beacons[j];
        taxicab_distance(first, second)
    })
}

fn reposition(scanner: &Scanner, stable: &Scanner, threshold: usize) -> Option<Scanner> {
    // println!(
    //     "beginning reposition search {} based off {} on rotation {}",
    //     scanner.id, stable.id, scanner.rotation.unwrap()
    // );

    let first_beacon_graph = beacon_graph(scanner);

    let second_beacon_graph = beacon_graph(stable);

    for (beacon_id, row) in first_beacon_graph.rows().into_iter().enumerate() {
        let counter = row.into_iter().fold(HashMap::new(), |mut map, val| {
            map.entry(val.clone()).and_modify(|v| *v += 1).or_insert(1);
            map
        });

        for (other_beacon_id, other_row) in second_beacon_graph.rows().into_iter().enumerate() {
            let other_counter = other_row.into_iter().fold(HashMap::new(), |mut map, val| {
                map.entry(val.clone()).and_modify(|v| *v += 1).or_insert(1);
                map
            });

            let intersection = counter_intersection(&counter, &other_counter);
            // println!(
            //     "orientation? {}/{} -> {}",
            //     scanner.id, stable.id, intersection
            // );
        

            if intersection >= threshold {
                // we have found a match
                // scanner.position + beacon.offset = stable.position + other_beacon.offset
                // scanner.position = stable.position + other_beacon.offset - beacon.offset
                let mut scanner = scanner.clone();
                scanner.position = stable.position.map(|position| {
                    position + stable.beacons[other_beacon_id].offset
                        - scanner.beacons[beacon_id].offset
                });
                // println!("  found orientation! {:?}", scanner.position);
                // let beacons = scanner
                //     .beacons
                //     .iter()
                //     .map(|beacon| {
                //         scanner
                //             .position
                //             .map(|position| position + beacon.offset)
                //             .map(|vec| (vec[0], vec[1], vec[2]))
                //             .unwrap_or_default()
                //     })
                //     .collect::<Vec<_>>();
                // println!("  {:?}", beacons);
                return Some(scanner);
            }
        }
    }

    None
}

fn reorient(scanner: &Scanner, orientation: &Matrix3<isize>) -> Scanner {
    Scanner {
        id: scanner.id,
        position: scanner.position,
        rotation: Some(*orientation),
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
    threshold: usize,
) -> Option<Scanner> {
    ORIENTATIONS
        .iter()
        .map(|orient| reorient(scanner, orient))
        .filter_map(|scanner| reposition(&scanner, target, threshold))
        .next()
}

fn generate_beacon_graph(beacons: &[Beacon]) -> Array2<isize> {
    let beacon_count = beacons.len();

    let mut beacon_graph = Array2::from_elem((beacon_count, beacon_count), 0);

    beacons
        .iter()
        .enumerate()
        .combinations(2)
        .map(|pair| (pair[0].0, pair[1].0, distance(pair[0].1, pair[1].1)))
        .for_each(|(i, j, dist)| {
            beacon_graph[(i, j)] = dist;
            beacon_graph[(j, i)] = dist;
        });

    beacon_graph
}

fn parse_input(input: &str) -> Vec<Scanner> {
    input
        .split("\n\n")
        .enumerate()
        .map(|(base_id, scanner)| {
            let mut scanner_lines = scanner.lines();
            let header = scanner_lines.next().unwrap();
            let id: usize = header.split(' ').nth(2).unwrap().parse().unwrap();
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
                id: base_id,
                position: Some(Vector3::new(0, 0, 0)),
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

fn taxicab_distance(first: &Beacon, second: &Beacon) -> Vec<isize> {
    first
        .offset
        .iter()
        .zip(&second.offset)
        .map(|(a, b)| b - a)
        .collect()
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

fn count_beacons(scanners: &[Scanner]) -> usize {
    let mut all_beacons = scanners
        .iter()
        .flat_map(|scanner| {
            scanner.beacons.iter().map(|beacon| {
                scanner
                    .position
                    .map(|position| position + beacon.offset)
                    .map(|vec| (vec[0], vec[1], vec[2]))
                    .unwrap_or_default()
            })
        })
        .collect::<Vec<_>>();

    all_beacons.sort_unstable();
    all_beacons.dedup();
    all_beacons.len()
}

fn graph_to_edges(graph: &Array2<usize>, threshold: usize) -> Vec<(usize, usize, usize)> {
    graph
        .rows()
        .into_iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, val)| **val >= (threshold * (threshold - 1)) / 2)
                .map(move |(j, val)| (i, j, *val))
                .collect::<Vec<_>>()
        })
        .collect()
}

fn reorient_scanners(
    scanners: &[Scanner],
    graph: &[(usize, usize, usize)],
    threshold: usize,
) -> Vec<Scanner> {
    let mut scanners_oriented = scanners.iter().map(|_| None).collect::<Vec<_>>();
    scanners_oriented[0] = Some(scanners[0].clone());

    while scanners_oriented.iter().any(|scanner| scanner.is_none()) {
        graph.iter().for_each(|(first, second, _)| {
            println!(
                "attempting to setup scanners {} and {} based on each other",
                first, second
            );

            let first_scanner = &scanners_oriented[*first];
            let second_scanner = &scanners_oriented[*second];
            match (first_scanner, second_scanner) {
                (Some(_), Some(_)) => {
                    println!("both are oriented, continuing");
                }
                /* find orientation for second */
                (Some(first_scanner), None) => {
                    println!("first has an entry");
                    match find_right_orientation(&scanners[*second], first_scanner, threshold) {
                        Some(scanner) => scanners_oriented[*second] = Some(scanner),
                        None => panic!("could not find orientation for scanner {}", second),
                    }
                }
                /* find orientation for first */
                (None, Some(second_scanner)) => {
                    println!("second has an entry");
                    match find_right_orientation(&scanners[*first], second_scanner, threshold) {
                        Some(scanner) => scanners_oriented[*first] = Some(scanner),
                        None => panic!("could not find orientation for scanner {}", first),
                    }
                }
                /* can't do anything until we get first or second oriented */
                (None, None) => {
                    println!("neither are currently oriented, skipping");
                } // panic!("attempting to match two unmatched scanners {} and {}", first, second),
            }
            // let beacons = first_scanner.beacons.iter().map(|beacon| {
            //     first_scanner
            //         .position
            //         .map(|position| position + beacon.offset)
            //         .map(|vec| (vec[0], vec[1], vec[2]))
            //         .unwrap_or_default()
            // }).collect::<Vec<_>>();
            // println!("looking from {} {:?}", first_scanner.id, beacons);
            // match find_right_orientation(&scanners[*second], &scanners_oriented[*first], threshold) {
            //     Some(scanner) => scanners_oriented[*second] = Some(scanner),
            //     None => panic!("could not find orientation for scanner {}", second),
            // }
        });
    }

    scanners_oriented
        .iter()
        .map(|scanner| scanner.as_ref().unwrap())
        .cloned()
        .collect()
}

fn solve_p1(scanners: &[Scanner], overlap_threshold: usize) -> usize {
    let mut scanner_graph = init_graph(scanners);

    let scanner_maps = scanners
        .iter()
        .map(get_distances)
        .collect::<Vec<_>>();

    build_graph(&mut scanner_graph, &scanner_maps);

    let edges = graph_to_edges(&scanner_graph, overlap_threshold);

    println!("{:?}", edges);

    let mut tree = kruskal_indices(scanners.len(), &edges)
        .map(|(first, second, weight)| (first.min(second), first.max(second), weight))
        .collect::<Vec<_>>();

    // tree.sort_unstable();

    println!("{:?}", tree);

    let scanners = reorient_scanners(scanners, &tree, overlap_threshold);

    // println!("scanners: {:?}", scanners);

    count_beacons(&scanners)
}

fn solve_p2(scanners: &[Scanner], overlap_threshold: usize) -> usize {
    let mut scanner_graph = init_graph(scanners);

    let scanner_maps = scanners
        .iter()
        .map(get_distances)
        .collect::<Vec<_>>();

    build_graph(&mut scanner_graph, &scanner_maps);

    let edges = graph_to_edges(&scanner_graph, overlap_threshold);

    println!("{:?}", edges);

    let mut tree = kruskal_indices(scanners.len(), &edges)
        .map(|(first, second, weight)| (first.min(second), first.max(second), weight))
        .collect::<Vec<_>>();

    // tree.sort_unstable();

    println!("{:?}", tree);

    let scanners = reorient_scanners(scanners, &tree, overlap_threshold);

    // println!("scanners: {:?}", scanners);

    scanners.iter().tuple_combinations().map(|(first, second)| {
        let p1 = first.position.unwrap();
        let p2 = second.position.unwrap();
        p1.iter().zip(&p2).map(|(a, b)|a.abs_diff(*b)).sum()
    }).max().unwrap()
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
    solve_p2(input, 12)
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
        // assert_eq!(0, super::solve_p2(&parsed_input, 3));
    }

    #[test]
    fn one_scanner_many_orientation() {
        let input = "--- scanner 0 ---\n-1,-1,1\n-2,-2,2\n-3,-3,3\n-2,-3,1\n5,6,-4\n8,0,7\n\n--- scanner 0 ---\n1,-1,1\n2,-2,2\n3,-3,3\n2,-1,3\n-5,4,-6\n-8,-7,0\n\n--- scanner 0 ---\n-1,-1,-1\n-2,-2,-2\n-3,-3,-3\n-1,-3,-2\n4,6,5\n-7,0,8\n\n--- scanner 0 ---\n1,1,-1\n2,2,-2\n3,3,-3\n1,3,-2\n-4,-6,5\n7,0,8\n\n--- scanner 0 ---\n1,1,1\n2,2,2\n3,3,3\n3,1,2\n-6,-4,-5\n0,7,-8";
        let parsed_input = super::input_generator(input);

        assert_eq!(6, super::solve_p1(&parsed_input, 6));
        // assert_eq!(0, super::solve_p2(&parsed_input, 6));
    }

    #[test]
    fn large_test() {
        let input = "--- scanner 0 ---\n404,-588,-901\n528,-643,409\n-838,591,734\n390,-675,-793\n-537,-823,-458\n-485,-357,347\n-345,-311,381\n-661,-816,-575\n-876,649,763\n-618,-824,-621\n553,345,-567\n474,580,667\n-447,-329,318\n-584,868,-557\n544,-627,-890\n564,392,-477\n455,729,728\n-892,524,684\n-689,845,-530\n423,-701,434\n7,-33,-71\n630,319,-379\n443,580,662\n-789,900,-551\n459,-707,401\n\n--- scanner 1 ---\n686,422,578\n605,423,415\n515,917,-361\n-336,658,858\n95,138,22\n-476,619,847\n-340,-569,-846\n567,-361,727\n-460,603,-452\n669,-402,600\n729,430,532\n-500,-761,534\n-322,571,750\n-466,-666,-811\n-429,-592,574\n-355,545,-477\n703,-491,-529\n-328,-685,520\n413,935,-424\n-391,539,-444\n586,-435,557\n-364,-763,-893\n807,-499,-711\n755,-354,-619\n553,889,-390\n\n--- scanner 2 ---\n649,640,665\n682,-795,504\n-784,533,-524\n-644,584,-595\n-588,-843,648\n-30,6,44\n-674,560,763\n500,723,-460\n609,671,-379\n-555,-800,653\n-675,-892,-343\n697,-426,-610\n578,704,681\n493,664,-388\n-671,-858,530\n-667,343,800\n571,-461,-707\n-138,-166,112\n-889,563,-600\n646,-828,498\n640,759,510\n-630,509,768\n-681,-892,-333\n673,-379,-804\n-742,-814,-386\n577,-820,562\n\n--- scanner 3 ---\n-589,542,597\n605,-692,669\n-500,565,-823\n-660,373,557\n-458,-679,-417\n-488,449,543\n-626,468,-788\n338,-750,-386\n528,-832,-391\n562,-778,733\n-938,-730,414\n543,643,-506\n-524,371,-870\n407,773,750\n-104,29,83\n378,-903,-323\n-778,-728,485\n426,699,580\n-438,-605,-362\n-469,-447,-387\n509,732,623\n647,635,-688\n-868,-804,481\n614,-800,639\n595,780,-596\n\n--- scanner 4 ---\n727,592,562\n-293,-554,779\n441,611,-461\n-714,465,-776\n-743,427,-804\n-660,-479,-426\n832,-632,460\n927,-485,-438\n408,393,-506\n466,436,-512\n110,16,151\n-258,-428,682\n-393,719,612\n-211,-452,876\n808,-476,-593\n-575,615,604\n-485,667,467\n-680,325,-822\n-627,-443,-432\n872,-547,-609\n833,512,582\n807,604,487\n839,-516,451\n891,-625,532\n-652,-548,-490\n30,-46,-14";
        let parsed_input = super::input_generator(input);

        assert_eq!(79, super::solve_p1(&parsed_input, 12));
        assert_eq!(3621, super::solve_p2(&parsed_input, 12));
    }
}
