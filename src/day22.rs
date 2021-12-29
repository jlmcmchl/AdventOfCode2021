use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

trait Volume {
    fn volume(&self) -> isize;
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Range {
    low: isize,
    high: isize,
}

impl Range {
    fn len(&self) -> isize {
        self.high - self.low + 1
    }

    fn encloses(&self, other: &Self) -> bool {
        self.low <= other.low && self.high >= other.high
    }

    fn envelopes(&self, other: &Self) -> bool {
        self.low < other.low && self.high > other.high
    }

    fn intersects(&self, other: &Self) -> bool {
        self.low <= other.high && self.high >= other.low
    }

    fn share_endpoint(&self, other: &Self) -> bool {
        self.low == other.low || self.high == other.high
    }

    fn union(&self, other: &Self) -> Option<Self> {
        if self.intersects(other) {
            Some(Range {
                low: self.low.min(other.low),
                high: self.high.max(other.high),
            })
        } else {
            None
        }
    }

    fn intersection(&self, other: &Self) -> Option<Self> {
        if self.intersects(other) {
            Some(Range {
                low: self.low.max(other.low),
                high: self.high.min(other.high),
            })
        } else {
            None
        }
    }

    fn left_difference(&self, other: &Self) -> Option<Self> {
        if other.encloses(self) || !self.intersects(other) {
            None
        } else if self.low < other.low {
            Some(Range {
                low: self.low,
                high: other.low - 1,
            })
        } else if self.high > other.high {
            Some(Range {
                low: other.high + 1,
                high: self.high,
            })
        } else {
            None
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cube {
    x: Range,
    y: Range,
    z: Range,
}

impl Cube {
    fn envelopes(&self, other: &Self) -> bool {
        self.x.envelopes(&other.x) && self.y.envelopes(&other.y) && self.z.envelopes(&other.z)
    }

    fn encloses(&self, other: &Self) -> bool {
        self.x.encloses(&other.x) && self.y.encloses(&other.y) && self.z.encloses(&other.z)
    }

    fn intersects(&self, other: &Self) -> bool {
        self.x.intersects(&other.x) && self.y.intersects(&other.y) && self.z.intersects(&other.z)
    }

    fn intersection(&self, other: &Self) -> Option<Self> {
        if self.intersects(other) {
            let x = self.x.intersection(&other.x).unwrap();
            let y = self.y.intersection(&other.y).unwrap();
            let z = self.z.intersection(&other.z).unwrap();
            Some(Cube { x, y, z })
        } else {
            None
        }
    }

    fn share_corner(&self, other: &Self) -> bool {
        self.x.share_endpoint(&other.x)
            && self.y.share_endpoint(&other.y)
            && self.z.share_endpoint(&other.z)
    }

    fn remove(&self, other: &Self) -> Vec<Self> {
        let mut other = *other;
        // println!("{} {:?} {:?}", self.intersects(&other), self, other);
        let mut cubes = if !self.encloses(&other) {
            Vec::new()
        } else if !self.share_corner(&other) {
            // println!("these don't share a corner: {:?} {:?}", self, other);
            // special case: other is not anchored to a corner
            //      create inner cube > other
            //      collect cubes
            //      is this a recursive case?
            //          expand other to one corner and then remove
            //          remove other from expanded case
            let x = Range {
                low: self.x.low,
                high: other.x.high,
            };
            let y = Range {
                low: self.y.low,
                high: other.y.high,
            };
            let z = Range {
                low: self.z.low,
                high: other.z.high,
            };
            let inner_cube = Cube { x, y, z };

            let cubes = inner_cube.remove(&other);

            other = inner_cube;

            // println!("cube within cube detected -\ninner: {:?}\nouter: {:?}\ninner_cube: {:?}\nresult: {:?}", other, self, inner_cube, cubes);

            cubes
        } else {
            Vec::new()
        };

        // split into 7 cubes
        //      intersecting on not x and not y and not z
        //      intersecting on x and not y and not z
        //      intersecting on y and not z and not x
        //      intersecting on z and not x and not y
        //      intersecting on x and y and not z
        //      intersecting on y and z and not x
        //      intersecting on z and x and not y
        let x_intersection = self.x.intersection(&other.x).unwrap();
        let y_intersection = self.y.intersection(&other.y).unwrap();
        let z_intersection = self.z.intersection(&other.z).unwrap();

        let x_diff = self.x.left_difference(&other.x);
        let y_diff = self.y.left_difference(&other.y);
        let z_diff = self.z.left_difference(&other.z);

        if let Some(x) = x_diff {
            cubes.push(Cube {
                x,
                y: y_intersection,
                z: z_intersection,
            });
        }

        if let Some(y) = y_diff {
            cubes.push(Cube {
                x: x_intersection,
                y,
                z: z_intersection,
            });
        }

        if let Some(z) = z_diff {
            cubes.push(Cube {
                x: x_intersection,
                y: y_intersection,
                z,
            });
        }

        if let (Some(x), Some(y)) = (x_diff, y_diff) {
            cubes.push(Cube {
                x,
                y,
                z: z_intersection,
            });
        }

        if let (Some(x), Some(z)) = (x_diff, z_diff) {
            cubes.push(Cube {
                x,
                y: y_intersection,
                z,
            });
        }

        if let (Some(y), Some(z)) = (y_diff, z_diff) {
            cubes.push(Cube {
                x: x_intersection,
                y,
                z,
            });
        }

        if let (Some(x), Some(y), Some(z)) = (x_diff, y_diff, z_diff) {
            cubes.push(Cube { x, y, z });
        }

        cubes
    }
}

impl Volume for Cube {
    fn volume(&self) -> isize {
        self.x.len() * self.y.len() * self.z.len()
    }
}

impl Volume for Vec<Cube> {
    fn volume(&self) -> isize {
        self.iter().map(|cube| cube.volume()).sum()
    }
}

fn parse_range(input: &str) -> Range {
    let (start, end) = input.split_once("..").unwrap();
    Range {
        low: start.parse().unwrap(),
        high: end.parse().unwrap(),
    }
}

fn parse_cube(input: &str) -> Cube {
    let mut cube: Cube = Default::default();
    input.split(',').for_each(|tagged_range| {
        let (tag, range) = tagged_range.split_once('=').unwrap();
        let range = parse_range(range);
        match tag {
            "x" => cube.x = range,
            "y" => cube.y = range,
            "z" => cube.z = range,
            _ => unreachable!(),
        }
    });

    cube
}

fn parse_input(input: &str) -> Vec<(bool, Cube)> {
    input
        .lines()
        .map(|line| {
            let (state, cube) = line.split_once(' ').unwrap();
            let state = matches!(state, "on");
            let cube = parse_cube(cube);
            (state, cube)
        })
        .collect()
}

fn count_cubes(input: &[(bool, Cube)]) -> isize {
    input
        .iter()
        .fold(Vec::<Cube>::new(), |agg, (state, new_cube)| {
            // println!(
            //     "{} {:?}",
            //     if *state { "adding" } else { "removing" },
            //     new_cube
            // );
            // println!("state: {}; volume: {}", agg.len(), agg.volume());
            // println!("new: {:?}", new_cube);

            // we guarantee each iteration that all cubes are distinct.
            // agg.iter()
            //     .enumerate()
            //     .tuple_combinations()
            //     .for_each(|(a, b)| {
            //         // println!("{:?}", a.0);
            //         // println!("{:?}", b.0);

            //         let intersection = a.1.intersection(b.1);
            //         // println!("{:?}", intersection);
            //         assert!(intersection.is_none(), "agg error: {:?} {:?}", a, b);
            //     });

            // with that assumption, all remove operations are distinct
            let mut agg: Vec<Cube> = agg
                .iter()
                .flat_map(|cube| {
                    if let Some(intersection) = cube.intersection(new_cube) {
                        // println!("{:?} {:?}", cube, intersection);
                        let mut removed1 = cube.remove(&intersection);

                        if *state {
                            removed1.push(intersection);
                        }

                        removed1
                    } else {
                        vec![*cube]
                    }
                })
                .collect();

            // if this is an `on`, then add the new cube minus all intersections with other cubes
            if *state {
                // println!("reducing: {:?}", new_cube);

                let mut new_cubes = agg.iter().fold(vec![*new_cube], |new_cubes, cube| {
                    // new_cubes
                    //     .iter()
                    //     .enumerate()
                    //     .tuple_combinations()
                    //     .for_each(|(a, b)| {
                    //         let intersection = a.1.intersection(b.1);
                    //         // println!("{:?}", intersection);
                    //         assert!(intersection.is_none(), "new_cube error: {:?} {:?}", a, b);
                    //     });

                    let new_cube = new_cubes
                        .iter()
                        .flat_map(|new_cube| {
                            // assert_eq!(cube.intersection(new_cube), new_cube.intersection(&cube));
                            if let Some(intersection) = new_cube.intersection(cube) {
                                let mut removed = new_cube.remove(&intersection);
                                // removed.sort_unstable();
                                // println!(
                                //     "intersection detected between {:?} and {:?}, intersection: {:?}, new: {:?}",
                                //     new_cube, cube, intersection, removed
                                // );
                                // assert_eq!(
                                //     new_cube.volume(),
                                //     removed.volume() + intersection.volume()
                                // );
                                removed
                            } else {
                                vec![*new_cube]
                            }
                        })
                        .collect::<Vec<_>>();
                    if new_cube.len() != new_cubes.len() {
                        // println!("after removing {:?}:\n{:?}", cube, new_cube);
                    }
                    new_cube
                });

                // println!("added cubes w/o dedup: {}", new_cubes.len());

                // new_cubes.sort_unstable();
                // new_cubes.dedup();

                // println!("added cubes: {}", new_cubes.len());

                agg.extend(new_cubes);
            }

            agg
        })
        .volume()
}

fn solve_p1(input: &[(bool, Cube)]) -> isize {
    let view = Cube {
        x: Range { low: -50, high: 50 },
        y: Range { low: -50, high: 50 },
        z: Range { low: -50, high: 50 },
    };

    let input = input
        .iter()
        .filter_map(|(state, cube)| view.intersection(cube).map(|cube| (*state, cube)))
        .collect::<Vec<_>>();
    count_cubes(&input)
}

fn solve_p2(input: &[(bool, Cube)]) -> isize {
    count_cubes(input)
}

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> Vec<(bool, Cube)> {
    parse_input(input)
}

#[aoc(day22, part1)]
pub fn wrapper_p1(input: &[(bool, Cube)]) -> isize {
    solve_p1(input)
}

#[aoc(day22, part2)]
pub fn wrapper_p2(input: &[(bool, Cube)]) -> isize {
    solve_p2(input)
}

#[cfg(test)]
mod tests {
    use crate::day22::{Cube, Range, Volume};

    #[test]
    fn test_range() {
        let range1 = Range { low: 10, high: 12 };
        let range2 = Range { low: 11, high: 13 };

        assert_eq!(Some(Range { low: 10, high: 13 }), range1.union(&range2));
        assert_eq!(
            Some(Range { low: 11, high: 12 }),
            range1.intersection(&range2)
        );
        assert_eq!(
            Some(Range { low: 10, high: 10 }),
            range1.left_difference(&range2)
        );
    }

    #[test]
    fn test_cube() {
        let cube1 = Cube {
            x: Range { low: 11, high: 12 },
            y: Range { low: 10, high: 10 },
            z: Range { low: 10, high: 12 },
        };

        let cube2 = Cube {
            x: Range { low: 11, high: 12 },
            y: Range { low: 11, high: 12 },
            z: Range { low: 11, high: 12 },
        };

        assert_eq!(6, cube1.volume());
        assert_eq!(8, cube2.volume());

        let intersection = cube1.intersection(&cube2);
        let removed1 = intersection.map(|cube| cube1.remove(&cube));
        let removed2 = intersection.map(|cube| cube2.remove(&cube));

        println!("{:?}", cube1);
        println!("{:?}", cube2);
        println!("{:?}", intersection);
        println!("{:?}", removed1);
        println!("{:?}", removed2);
        println!("{:?}", intersection.map(|cube| cube.volume()));
        println!("{:?}", removed1.map(|cube| cube.volume()));
        println!("{:?}", removed2.map(|cube| cube.volume()));
    }

    #[test]
    fn test_p1() {
        let inputs = vec![
            ("on x=10..12,y=10..12,z=10..12\non x=11..13,y=11..13,z=11..13\noff x=9..11,y=9..11,z=9..11\non x=10..10,y=10..10,z=10..10", 39),
            ("on x=-20..26,y=-36..17,z=-47..7\non x=-20..33,y=-21..23,z=-26..28\non x=-22..28,y=-29..23,z=-38..16\non x=-46..7,y=-6..46,z=-50..-1\non x=-49..1,y=-3..46,z=-24..28\non x=2..47,y=-22..22,z=-23..27\non x=-27..23,y=-28..26,z=-21..29\non x=-39..5,y=-6..47,z=-3..44\non x=-30..21,y=-8..43,z=-13..34\non x=-22..26,y=-27..20,z=-29..19\noff x=-48..-32,y=26..41,z=-47..-37\non x=-12..35,y=6..50,z=-50..-2\noff x=-48..-32,y=-32..-16,z=-15..-5\non x=-18..26,y=-33..15,z=-7..46\noff x=-40..-22,y=-38..-28,z=23..41\non x=-16..35,y=-41..10,z=-47..6\noff x=-32..-23,y=11..30,z=-14..3\non x=-49..-5,y=-3..45,z=-29..18\noff x=18..30,y=-20..-8,z=-3..13\non x=-41..9,y=-7..43,z=-33..15\non x=-54112..-39298,y=-85059..-49293,z=-27449..7877\non x=967..23432,y=45373..81175,z=27513..53682", 590784),
            ("on x=-5..47,y=-31..22,z=-19..33\non x=-44..5,y=-27..21,z=-14..35\non x=-49..-1,y=-11..42,z=-10..38\non x=-20..34,y=-40..6,z=-44..1\noff x=26..39,y=40..50,z=-2..11\non x=-41..5,y=-41..6,z=-36..8\noff x=-43..-33,y=-45..-28,z=7..25\non x=-33..15,y=-32..19,z=-34..11\noff x=35..47,y=-46..-34,z=-11..5\non x=-14..36,y=-6..44,z=-16..29\non x=-57795..-6158,y=29564..72030,z=20435..90618\non x=36731..105352,y=-21140..28532,z=16094..90401\non x=30999..107136,y=-53464..15513,z=8553..71215\non x=13528..83982,y=-99403..-27377,z=-24141..23996\non x=-72682..-12347,y=18159..111354,z=7391..80950\non x=-1060..80757,y=-65301..-20884,z=-103788..-16709\non x=-83015..-9461,y=-72160..-8347,z=-81239..-26856\non x=-52752..22273,y=-49450..9096,z=54442..119054\non x=-29982..40483,y=-108474..-28371,z=-24328..38471\non x=-4958..62750,y=40422..118853,z=-7672..65583\non x=55694..108686,y=-43367..46958,z=-26781..48729\non x=-98497..-18186,y=-63569..3412,z=1232..88485\non x=-726..56291,y=-62629..13224,z=18033..85226\non x=-110886..-34664,y=-81338..-8658,z=8914..63723\non x=-55829..24974,y=-16897..54165,z=-121762..-28058\non x=-65152..-11147,y=22489..91432,z=-58782..1780\non x=-120100..-32970,y=-46592..27473,z=-11695..61039\non x=-18631..37533,y=-124565..-50804,z=-35667..28308\non x=-57817..18248,y=49321..117703,z=5745..55881\non x=14781..98692,y=-1341..70827,z=15753..70151\non x=-34419..55919,y=-19626..40991,z=39015..114138\non x=-60785..11593,y=-56135..2999,z=-95368..-26915\non x=-32178..58085,y=17647..101866,z=-91405..-8878\non x=-53655..12091,y=50097..105568,z=-75335..-4862\non x=-111166..-40997,y=-71714..2688,z=5609..50954\non x=-16602..70118,y=-98693..-44401,z=5197..76897\non x=16383..101554,y=4615..83635,z=-44907..18747\noff x=-95822..-15171,y=-19987..48940,z=10804..104439\non x=-89813..-14614,y=16069..88491,z=-3297..45228\non x=41075..99376,y=-20427..49978,z=-52012..13762\non x=-21330..50085,y=-17944..62733,z=-112280..-30197\non x=-16478..35915,y=36008..118594,z=-7885..47086\noff x=-98156..-27851,y=-49952..43171,z=-99005..-8456\noff x=2032..69770,y=-71013..4824,z=7471..94418\non x=43670..120875,y=-42068..12382,z=-24787..38892\noff x=37514..111226,y=-45862..25743,z=-16714..54663\noff x=25699..97951,y=-30668..59918,z=-15349..69697\noff x=-44271..17935,y=-9516..60759,z=49131..112598\non x=-61695..-5813,y=40978..94975,z=8655..80240\noff x=-101086..-9439,y=-7088..67543,z=33935..83858\noff x=18020..114017,y=-48931..32606,z=21474..89843\noff x=-77139..10506,y=-89994..-18797,z=-80..59318\noff x=8476..79288,y=-75520..11602,z=-96624..-24783\non x=-47488..-1262,y=24338..100707,z=16292..72967\noff x=-84341..13987,y=2429..92914,z=-90671..-1318\noff x=-37810..49457,y=-71013..-7894,z=-105357..-13188\noff x=-27365..46395,y=31009..98017,z=15428..76570\noff x=-70369..-16548,y=22648..78696,z=-1892..86821\non x=-53470..21291,y=-120233..-33476,z=-44150..38147\noff x=-93533..-4276,y=-16170..68771,z=-104985..-24507", 474140)
        ];

        for (input, expect) in inputs {
            let parsed_input = super::input_generator(input);
            assert_eq!(expect, super::solve_p1(&parsed_input));
        }
    }

    #[test]
    fn test_p2() {
        let inputs = vec![
            ("on x=-5..47,y=-31..22,z=-19..33\non x=-44..5,y=-27..21,z=-14..35\non x=-49..-1,y=-11..42,z=-10..38\non x=-20..34,y=-40..6,z=-44..1\noff x=26..39,y=40..50,z=-2..11\non x=-41..5,y=-41..6,z=-36..8\noff x=-43..-33,y=-45..-28,z=7..25\non x=-33..15,y=-32..19,z=-34..11\noff x=35..47,y=-46..-34,z=-11..5\non x=-14..36,y=-6..44,z=-16..29\non x=-57795..-6158,y=29564..72030,z=20435..90618\non x=36731..105352,y=-21140..28532,z=16094..90401\non x=30999..107136,y=-53464..15513,z=8553..71215\non x=13528..83982,y=-99403..-27377,z=-24141..23996\non x=-72682..-12347,y=18159..111354,z=7391..80950\non x=-1060..80757,y=-65301..-20884,z=-103788..-16709\non x=-83015..-9461,y=-72160..-8347,z=-81239..-26856\non x=-52752..22273,y=-49450..9096,z=54442..119054\non x=-29982..40483,y=-108474..-28371,z=-24328..38471\non x=-4958..62750,y=40422..118853,z=-7672..65583\non x=55694..108686,y=-43367..46958,z=-26781..48729\non x=-98497..-18186,y=-63569..3412,z=1232..88485\non x=-726..56291,y=-62629..13224,z=18033..85226\non x=-110886..-34664,y=-81338..-8658,z=8914..63723\non x=-55829..24974,y=-16897..54165,z=-121762..-28058\non x=-65152..-11147,y=22489..91432,z=-58782..1780\non x=-120100..-32970,y=-46592..27473,z=-11695..61039\non x=-18631..37533,y=-124565..-50804,z=-35667..28308\non x=-57817..18248,y=49321..117703,z=5745..55881\non x=14781..98692,y=-1341..70827,z=15753..70151\non x=-34419..55919,y=-19626..40991,z=39015..114138\non x=-60785..11593,y=-56135..2999,z=-95368..-26915\non x=-32178..58085,y=17647..101866,z=-91405..-8878\non x=-53655..12091,y=50097..105568,z=-75335..-4862\non x=-111166..-40997,y=-71714..2688,z=5609..50954\non x=-16602..70118,y=-98693..-44401,z=5197..76897\non x=16383..101554,y=4615..83635,z=-44907..18747\noff x=-95822..-15171,y=-19987..48940,z=10804..104439\non x=-89813..-14614,y=16069..88491,z=-3297..45228\non x=41075..99376,y=-20427..49978,z=-52012..13762\non x=-21330..50085,y=-17944..62733,z=-112280..-30197\non x=-16478..35915,y=36008..118594,z=-7885..47086\noff x=-98156..-27851,y=-49952..43171,z=-99005..-8456\noff x=2032..69770,y=-71013..4824,z=7471..94418\non x=43670..120875,y=-42068..12382,z=-24787..38892\noff x=37514..111226,y=-45862..25743,z=-16714..54663\noff x=25699..97951,y=-30668..59918,z=-15349..69697\noff x=-44271..17935,y=-9516..60759,z=49131..112598\non x=-61695..-5813,y=40978..94975,z=8655..80240\noff x=-101086..-9439,y=-7088..67543,z=33935..83858\noff x=18020..114017,y=-48931..32606,z=21474..89843\noff x=-77139..10506,y=-89994..-18797,z=-80..59318\noff x=8476..79288,y=-75520..11602,z=-96624..-24783\non x=-47488..-1262,y=24338..100707,z=16292..72967\noff x=-84341..13987,y=2429..92914,z=-90671..-1318\noff x=-37810..49457,y=-71013..-7894,z=-105357..-13188\noff x=-27365..46395,y=31009..98017,z=15428..76570\noff x=-70369..-16548,y=22648..78696,z=-1892..86821\non x=-53470..21291,y=-120233..-33476,z=-44150..38147\noff x=-93533..-4276,y=-16170..68771,z=-104985..-24507", 2758514936282235)
        ];

        for (input, expect) in inputs {
            let parsed_input = super::input_generator(input);
            assert_eq!(expect, super::solve_p2(&parsed_input));
        }
    }
}
