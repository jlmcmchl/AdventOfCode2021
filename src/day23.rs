use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use ndarray::Array2;
use pathfinding::prelude::{astar, build_path, dijkstra_all};
use rayon::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Amphipod {
    A,
    B,
    C,
    D,
}

lazy_static! {
/*
############# | #############
#...........# | #01.2.3.4.56#
###D#D#A#A### | ###7#8#9#A###
  #D#C#B#A#   |   #B#C#D#E#
  #D#B#A#C#   |   #F#G#H#I#
  #C#C#B#B#   |   #J#K#L#M#
  #########   |   #########
*/

        static ref SKIP_GRAPH: Array2<usize> = Array2::from_shape_vec((23, 23), vec![
//       0   1  2  3  4   5   6    7  8  9   A     B   C   D   E     F   G   H   I     J   K   L   M
         0,  0, 0, 0, 0,  0,  0,   3, 5, 7,  9,    4,  6,  8, 10,    5,  7,  9, 11,    6,  8, 10, 12, // 0
         0,  0, 0, 0, 0,  0,  0,   2, 4, 6,  8,    3,  5,  7,  9,    4,  6,  8, 10,    5,  7,  9, 11, // 1
         0,  0, 0, 0, 0,  0,  0,   2, 2, 4,  6,    3,  3,  5,  7,    4,  4,  6,  8,    5,  5,  7,  9, // 2
         0,  0, 0, 0, 0,  0,  0,   4, 2, 2,  4,    5,  3,  3,  5,    6,  4,  4,  6,    7,  5,  5,  7, // 3
         0,  0, 0, 0, 0,  0,  0,   6, 4, 2,  2,    7,  5,  3,  3,    8,  6,  4,  4,    9,  7,  5,  5, // 4
         0,  0, 0, 0, 0,  0,  0,   8, 6, 4,  2,    9,  7,  5,  3,   10,  8,  6,  4,   11,  9,  7,  5, // 5 // TODO
         0,  0, 0, 0, 0,  0,  0,   9, 7, 5,  3,   10,  8,  6,  4,   11,  9,  7,  5,   12, 10,  8,  6, // 6

         3,  2, 2, 4, 6,  8,  9,   0, 4, 6,  8,    1,  5,  7,  9,    2,  6,  8, 10,    3,  7,  9, 11, // 7
         5,  4, 2, 2, 4,  6,  7,   4, 0, 4,  6,    5,  1,  5,  7,    6,  2,  6,  8,    7,  3,  7,  9, // 8
         7,  6, 4, 2, 2,  4,  5,   6, 4, 0,  4,    7,  5,  1,  5,    8,  6,  2,  6,    9,  7,  3,  7, // 9
         9,  8, 6, 4, 2,  2,  3,   8, 6, 4,  0,    9,  7,  5,  1,   10,  8,  6,  2,   11,  9,  7,  3, // A

         4,  3, 3, 5, 7,  9, 10,   1, 5, 7,  9,    0,  6,  8, 10,    1,  7,  9, 11,    2,  8, 10, 12, // B
         6,  5, 3, 3, 5,  7,  8,   5, 1, 5,  7,    6,  0,  6,  8,    7,  1,  7,  9,    8,  2,  8, 10, // C
         8,  7, 5, 3, 3,  5,  6,   7, 5, 1,  5,    8,  6,  0,  6,    9,  7,  1,  7,   10,  8,  2,  8, // D
        10,  9, 7, 5, 3,  3,  4,   9, 7, 5,  1,   10,  8,  6,  0,   11,  9,  7,  1,   12, 10,  8,  2, // E

         5,  4, 4, 6, 8, 10, 11,   2, 6, 8, 10,    1,  7,  9, 11,    0,  8, 10, 12,    1,  9, 11, 13, // F
         7,  6, 4, 4, 6,  8,  9,   6, 2, 6,  8,    7,  1,  7,  9,    8,  0,  8, 10,    9,  1,  9, 11, // G
         9,  8, 6, 4, 4,  6,  7,   8, 6, 2,  6,    9,  7,  1,  7,   10,  8,  0,  8,   11,  9,  1,  9, // H
        11, 10, 8, 6, 4,  4,  5,  10, 8, 6,  2,   11,  9,  7,  1,   12, 10,  8,  0,   13, 11,  9,  1, // I

         6,  5, 5, 7, 9, 11, 12,   3, 7, 9, 11,    2,  8, 10, 12,    1,  9, 11, 13,    0, 10, 12, 14, // J
         8,  7, 5, 5, 7,  9, 10,   7, 3, 7,  9,    8,  2,  8, 10,    9,  1,  9, 11,   10,  0, 10, 12, // K
        10,  9, 7, 5, 5,  7,  8,   9, 7, 3,  7,   10,  8,  2,  8,   11,  9,  1,  9,   12, 10,  0, 10, // L
        12, 11, 9, 7, 5,  5,  6,  11, 9, 7,  3,   12, 10,  8,  2,   13, 11,  9,  1,   14, 12, 10,  0, // M
    ]).unwrap();

    static ref A_SKIP_GRAPH: Array2<usize> = Array2::from_shape_vec((23, 23), vec![
    //   0   1  2  3  4   5   6    7  8  9  A     B   C   D   E     F   G   H   I     J   K   L   M
         0,  0, 0, 0, 0,  0,  0,   3, 0, 0, 0,    4,  0,  0,  0,    5,  0,  0,  0,    6,  0,  0,  0, // 0
         0,  0, 0, 0, 0,  0,  0,   2, 0, 0, 0,    3,  0,  0,  0,    4,  0,  0,  0,    5,  0,  0,  0, // 1
         0,  0, 0, 0, 0,  0,  0,   2, 0, 0, 0,    3,  0,  0,  0,    4,  0,  0,  0,    5,  0,  0,  0, // 2
         0,  0, 0, 0, 0,  0,  0,   4, 0, 0, 0,    5,  0,  0,  0,    6,  0,  0,  0,    7,  0,  0,  0, // 3
         0,  0, 0, 0, 0,  0,  0,   6, 0, 0, 0,    7,  0,  0,  0,    8,  0,  0,  0,    9,  0,  0,  0, // 4
         0,  0, 0, 0, 0,  0,  0,   8, 0, 0, 0,    9,  0,  0,  0,   10,  0,  0,  0,   11,  0,  0,  0, // 5 // TODO
         0,  0, 0, 0, 0,  0,  0,   9, 0, 0, 0,   10,  0,  0,  0,   11,  0,  0,  0,   12,  0,  0,  0, // 6
         3,  2, 2, 4, 6,  8,  9,   0, 0, 0, 0,    1,  0,  0,  0,    2,  0,  0,  0,    3,  0,  0,  0, // 7
         5,  4, 2, 2, 4,  6,  7,   4, 0, 0, 0,    5,  0,  0,  0,    6,  0,  0,  0,    7,  0,  0,  0, // 8
         7,  6, 4, 2, 2,  4,  5,   6, 0, 0, 0,    7,  0,  0,  0,    8,  0,  0,  0,    9,  0,  0,  0, // 9
         9,  8, 6, 4, 2,  2,  3,   8, 0, 0, 0,    9,  0,  0,  0,   10,  0,  0,  0,   11,  0,  0,  0, // A
         4,  3, 3, 5, 7,  9, 10,   1, 0, 0, 0,    0,  0,  0,  0,    1,  0,  0,  0,    2,  0,  0,  0, // B
         6,  5, 3, 3, 5,  7,  8,   5, 0, 0, 0,    6,  0,  0,  0,    7,  0,  0,  0,    8,  0,  0,  0, // C
         8,  7, 5, 3, 3,  5,  6,   7, 0, 0, 0,    8,  0,  0,  0,    9,  0,  0,  0,   10,  0,  0,  0, // D
        10,  9, 7, 5, 3,  3,  4,   9, 0, 0, 0,   10,  0,  0,  0,   11,  0,  0,  0,   12,  0,  0,  0, // E
         5,  4, 4, 6, 8, 10, 11,   2, 0, 0, 0,    1,  0,  0,  0,    0,  0,  0,  0,    1,  0,  0,  0, // F
         7,  6, 4, 4, 6,  8,  9,   6, 0, 0, 0,    7,  0,  0,  0,    8,  0,  0,  0,    9,  0,  0,  0, // G
         9,  8, 6, 4, 4,  6,  7,   8, 0, 0, 0,    9,  0,  0,  0,   10,  0,  0,  0,   11,  0,  0,  0, // H
        11, 10, 8, 6, 4,  4,  5,  10, 0, 0, 0,   11,  0,  0,  0,   12,  0,  0,  0,   13,  0,  0,  0, // I
         6,  5, 5, 7, 9, 11, 12,   3, 0, 0, 0,    2,  0,  0,  0,    1,  0,  0,  0,    0,  0,  0,  0, // J
         8,  7, 5, 5, 7,  9, 10,   7, 0, 0, 0,    8,  0,  0,  0,    9,  0,  0,  0,   10,  0,  0,  0, // K
        10,  9, 7, 5, 5,  7,  8,   9, 0, 0, 0,   10,  0,  0,  0,   11,  0,  0,  0,   12,  0,  0,  0, // L
        12, 11, 9, 7, 5,  5,  6,  11, 0, 0, 0,   12,  0,  0,  0,   13,  0,  0,  0,   14,  0,  0,  0, // M
    ]).unwrap();

    static ref B_SKIP_GRAPH: Array2<usize> = Array2::from_shape_vec((23, 23), vec![
    //       0   1  2  3  4   5   6    7  8  9   A     B   C   D   E     F   G   H   I     J   K   L   M
             0,  0, 0, 0, 0,  0,  0,   0, 5, 0,  0,    0,  6,  0,  0,    0,  7,  0,  0,    0,  8,  0,  0, // 0
             0,  0, 0, 0, 0,  0,  0,   0, 4, 0,  0,    0,  5,  0,  0,    0,  6,  0,  0,    0,  7,  0,  0, // 1
             0,  0, 0, 0, 0,  0,  0,   0, 2, 0,  0,    0,  3,  0,  0,    0,  4,  0,  0,    0,  5,  0,  0, // 2
             0,  0, 0, 0, 0,  0,  0,   0, 2, 0,  0,    0,  3,  0,  0,    0,  4,  0,  0,    0,  5,  0,  0, // 3
             0,  0, 0, 0, 0,  0,  0,   0, 4, 0,  0,    0,  5,  0,  0,    0,  6,  0,  0,    0,  7,  0,  0, // 4
             0,  0, 0, 0, 0,  0,  0,   0, 6, 0,  0,    0,  7,  0,  0,    0,  8,  0,  0,    0,  9,  0,  0, // 5 // TODO
             0,  0, 0, 0, 0,  0,  0,   0, 7, 0,  0,    0,  8,  0,  0,    0,  9,  0,  0,    0, 10,  0,  0, // 6
             3,  2, 2, 4, 6,  8,  9,   0, 4, 0,  0,    0,  5,  0,  0,    0,  6,  0,  0,    0,  7,  0,  0, // 7
             5,  4, 2, 2, 4,  6,  7,   0, 0, 0,  0,    0,  1,  0,  0,    0,  2,  0,  0,    0,  3,  0,  0, // 8
             7,  6, 4, 2, 2,  4,  5,   0, 4, 0,  0,    0,  5,  0,  0,    0,  6,  0,  0,    0,  7,  0,  0, // 9
             9,  8, 6, 4, 2,  2,  3,   0, 6, 0,  0,    0,  7,  0,  0,    0,  8,  0,  0,    0,  9,  0,  0, // A
             4,  3, 3, 5, 7,  9, 10,   0, 5, 0,  0,    0,  6,  0,  0,    0,  7,  0,  0,    0,  8,  0,  0, // B
             6,  5, 3, 3, 5,  7,  8,   0, 1, 0,  0,    0,  0,  0,  0,    0,  1,  0,  0,    0,  2,  0,  0, // C
             8,  7, 5, 3, 3,  5,  6,   0, 5, 0,  0,    0,  6,  0,  0,    0,  7,  0,  0,    0,  8,  0,  0, // D
            10,  9, 7, 5, 3,  3,  4,   0, 7, 0,  0,    0,  8,  0,  0,    0,  9,  0,  0,    0, 10,  0,  0, // E
             5,  4, 4, 6, 8, 10, 11,   0, 6, 0,  0,    0,  7,  0,  0,    0,  8,  0,  0,    0,  9,  0,  0, // F
             7,  6, 4, 4, 6,  8,  9,   0, 2, 0,  0,    0,  1,  0,  0,    0,  0,  0,  0,    0,  1,  0,  0, // G
             9,  8, 6, 4, 4,  6,  7,   0, 6, 0,  0,    0,  7,  0,  0,    0,  8,  0,  0,    0,  9,  0,  0, // H
            11, 10, 8, 6, 4,  4,  5,   0, 8, 0,  0,    0,  9,  0,  0,    0, 10,  0,  0,    0, 11,  0,  0, // I
             6,  5, 5, 7, 9, 11, 12,   0, 7, 0,  0,    0,  8,  0,  0,    0,  9,  0,  0,    0, 10,  0,  0, // J
             8,  7, 5, 5, 7,  9, 10,   0, 3, 0,  0,    0,  2,  0,  0,    0,  1,  0,  0,    0,  0,  0,  0, // K
            10,  9, 7, 5, 5,  7,  8,   0, 7, 0,  0,    0,  8,  0,  0,    0,  9,  0,  0,    0, 10,  0,  0, // L
            12, 11, 9, 7, 5,  5,  6,   0, 9, 0,  0,    0, 10,  0,  0,    0, 11,  0,  0,    0, 12,  0,  0, // M
        ]).unwrap();

    static ref C_SKIP_GRAPH: Array2<usize> = Array2::from_shape_vec((23, 23), vec![
        //   0   1  2  3  4   5   6    7  8  9   A     B   C   D   E     F   G   H   I     J   K   L   M
             0,  0, 0, 0, 0,  0,  0,   0, 0, 7,  0,    0,  0,  8,  0,    0,  0,  9,  0,    0,  0, 10,  0, // 0
             0,  0, 0, 0, 0,  0,  0,   0, 0, 6,  0,    0,  0,  7,  0,    0,  0,  8,  0,    0,  0,  9,  0, // 1
             0,  0, 0, 0, 0,  0,  0,   0, 0, 4,  0,    0,  0,  5,  0,    0,  0,  6,  0,    0,  0,  7,  0, // 2
             0,  0, 0, 0, 0,  0,  0,   0, 0, 2,  0,    0,  0,  3,  0,    0,  0,  4,  0,    0,  0,  5,  0, // 3
             0,  0, 0, 0, 0,  0,  0,   0, 0, 2,  0,    0,  0,  3,  0,    0,  0,  4,  0,    0,  0,  5,  0, // 4
             0,  0, 0, 0, 0,  0,  0,   0, 0, 4,  0,    0,  0,  5,  0,    0,  0,  6,  0,    0,  0,  7,  0, // 5 // TODO
             0,  0, 0, 0, 0,  0,  0,   0, 0, 5,  0,    0,  0,  6,  0,    0,  0,  7,  0,    0,  0,  8,  0, // 6
             3,  2, 2, 4, 6,  8,  9,   0, 0, 6,  0,    0,  0,  7,  0,    0,  0,  8,  0,    0,  0,  9,  0, // 7
             5,  4, 2, 2, 4,  6,  7,   0, 0, 4,  0,    0,  0,  5,  0,    0,  0,  6,  0,    0,  0,  7,  0, // 8
             7,  6, 4, 2, 2,  4,  5,   0, 0, 0,  0,    0,  0,  1,  0,    0,  0,  2,  0,    0,  0,  3,  0, // 9
             9,  8, 6, 4, 2,  2,  3,   0, 0, 4,  0,    0,  0,  5,  0,    0,  0,  6,  0,    0,  0,  7,  0, // A
             4,  3, 3, 5, 7,  9, 10,   0, 0, 7,  0,    0,  0,  8,  0,    0,  0,  9,  0,    0,  0, 10,  0, // B
             6,  5, 3, 3, 5,  7,  8,   0, 0, 5,  0,    0,  0,  6,  0,    0,  0,  7,  0,    0,  0,  8,  0, // C
             8,  7, 5, 3, 3,  5,  6,   0, 0, 1,  0,    0,  0,  0,  0,    0,  0,  1,  0,    0,  0,  2,  0, // D
            10,  9, 7, 5, 3,  3,  4,   0, 0, 5,  0,    0,  0,  6,  0,    0,  0,  7,  0,    0,  0,  8,  0, // E
             5,  4, 4, 6, 8, 10, 11,   0, 0, 8,  0,    0,  0,  9,  0,    0,  0, 10,  0,    0,  0, 11,  0, // F
             7,  6, 4, 4, 6,  8,  9,   0, 0, 6,  0,    0,  0,  7,  0,    0,  0,  8,  0,    0,  0,  9,  0, // G
             9,  8, 6, 4, 4,  6,  7,   0, 0, 2,  0,    0,  0,  1,  0,    0,  0,  0,  0,    0,  0,  1,  0, // H
            11, 10, 8, 6, 4,  4,  5,   0, 0, 6,  0,    0,  0,  7,  0,    0,  0,  8,  0,    0,  0,  9,  0, // I
             6,  5, 5, 7, 9, 11, 12,   0, 0, 9,  0,    0,  0, 10,  0,    0,  0, 11,  0,    0,  0, 12,  0, // J
             8,  7, 5, 5, 7,  9, 10,   0, 0, 7,  0,    0,  0,  8,  0,    0,  0,  9,  0,    0,  0, 10,  0, // K
            10,  9, 7, 5, 5,  7,  8,   0, 0, 3,  0,    0,  0,  2,  0,    0,  0,  1,  0,    0,  0,  0,  0, // L
            12, 11, 9, 7, 5,  5,  6,   0, 0, 7,  0,    0,  0,  8,  0,    0,  0,  9,  0,    0,  0, 10,  0, // M
        ]).unwrap();

    static ref D_SKIP_GRAPH: Array2<usize> = Array2::from_shape_vec((23, 23), vec![
            //  0   1  2  3  4   5   6    7  8  9   A     B   C   D   E     F   G   H   I     J   K   L   M
                0,  0, 0, 0, 0,  0,  0,   0, 0, 0,  9,    0,  0,  0, 10,    0,  0,  0, 11,    0,  0,  0, 12, // 0
                0,  0, 0, 0, 0,  0,  0,   0, 0, 0,  8,    0,  0,  0,  9,    0,  0,  0, 10,    0,  0,  0, 11, // 1
                0,  0, 0, 0, 0,  0,  0,   0, 0, 0,  6,    0,  0,  0,  7,    0,  0,  0,  8,    0,  0,  0,  9, // 2
                0,  0, 0, 0, 0,  0,  0,   0, 0, 0,  4,    0,  0,  0,  5,    0,  0,  0,  6,    0,  0,  0,  7, // 3
                0,  0, 0, 0, 0,  0,  0,   0, 0, 0,  2,    0,  0,  0,  3,    0,  0,  0,  4,    0,  0,  0,  5, // 4
                0,  0, 0, 0, 0,  0,  0,   0, 0, 0,  2,    0,  0,  0,  3,    0,  0,  0,  4,    0,  0,  0,  5, // 5 // TODO
                0,  0, 0, 0, 0,  0,  0,   0, 0, 0,  3,    0,  0,  0,  4,    0,  0,  0,  5,    0,  0,  0,  6, // 6
                3,  2, 2, 4, 6,  8,  9,   0, 0, 0,  8,    0,  0,  0,  9,    0,  0,  0, 10,    0,  0,  0, 11, // 7
                5,  4, 2, 2, 4,  6,  7,   0, 0, 0,  6,    0,  0,  0,  7,    0,  0,  0,  8,    0,  0,  0,  9, // 8
                7,  6, 4, 2, 2,  4,  5,   0, 0, 0,  4,    0,  0,  0,  5,    0,  0,  0,  6,    0,  0,  0,  7, // 9
                9,  8, 6, 4, 2,  2,  3,   0, 0, 0,  0,    0,  0,  0,  1,    0,  0,  0,  2,    0,  0,  0,  3, // A
                4,  3, 3, 5, 7,  9, 10,   0, 0, 0,  9,    0,  0,  0, 10,    0,  0,  0, 11,    0,  0,  0, 12, // B
                6,  5, 3, 3, 5,  7,  8,   0, 0, 0,  7,    0,  0,  0,  8,    0,  0,  0,  9,    0,  0,  0, 10, // C
                8,  7, 5, 3, 3,  5,  6,   0, 0, 0,  5,    0,  0,  0,  6,    0,  0,  0,  7,    0,  0,  0,  8, // D
               10,  9, 7, 5, 3,  3,  4,   0, 0, 0,  1,    0,  0,  0,  0,    0,  0,  0,  1,    0,  0,  0,  2, // E
                5,  4, 4, 6, 8, 10, 11,   0, 0, 0, 10,    0,  0,  0, 11,    0,  0,  0, 12,    0,  0,  0, 13, // F
                7,  6, 4, 4, 6,  8,  9,   0, 0, 0,  8,    0,  0,  0,  9,    0,  0,  0, 10,    0,  0,  0, 11, // G
                9,  8, 6, 4, 4,  6,  7,   0, 0, 0,  6,    0,  0,  0,  7,    0,  0,  0,  8,    0,  0,  0,  9, // H
               11, 10, 8, 6, 4,  4,  5,   0, 0, 0,  2,    0,  0,  0,  1,    0,  0,  0,  0,    0,  0,  0,  1, // I
                6,  5, 5, 7, 9, 11, 12,   0, 0, 0, 11,    0,  0,  0, 12,    0,  0,  0, 13,    0,  0,  0, 14, // J
                8,  7, 5, 5, 7,  9, 10,   0, 0, 0,  9,    0,  0,  0, 10,    0,  0,  0, 11,    0,  0,  0, 12, // K
               10,  9, 7, 5, 5,  7,  8,   0, 0, 0,  7,    0,  0,  0,  8,    0,  0,  0,  9,    0,  0,  0, 10, // L
               12, 11, 9, 7, 5,  5,  6,   0, 0, 0,  3,    0,  0,  0,  2,    0,  0,  0,  1,    0,  0,  0,  0, // M
        ]).unwrap();

    static ref GRAPH: Array2<usize> = Array2::from_shape_vec((23, 23), vec![
//      0  1  2  3  4  5  6  7  8  9  A  B  C  D  E  F  G  H  I  J  K  L  M
        0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0
        1, 0, 2, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 1
        0, 2, 0, 2, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 2
        0, 0, 2, 0, 2, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 3
        0, 0, 0, 2, 0, 2, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 4
        0, 0, 0, 0, 2, 0, 1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 5
        0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 6
        0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 7
        0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 8
        0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 9
        0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, // A
        0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, // B
        0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, // C
        0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, // D
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, // E
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, // F
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, // G
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, // H
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, // I
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, // J
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, // K
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, // L
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, // M
    ]).unwrap();

    static ref NODE_PATHS: Vec<Vec<Vec<usize>>> =
        (0..23)
        .map(|node| {
            let parent_map = dijkstra_all(&node, |node| {
                GRAPH
                    .row(*node)
                    .iter()
                    .enumerate()
                    .filter(|(_, weight)| **weight != 0)
                    .map(|(id, weight)| (id, *weight))
                    .collect::<Vec<_>>()
            });
            (0..23)
                .filter(|target| *target != node)
                .map(|target| {
                    let mut path = build_path(&target, &parent_map).iter().skip(1).cloned().collect::<Vec<_>>();
                    path.reverse();
                    path
                })
                .collect()
        })
        .collect();

    static ref GOAL_STATE: Vec<Option<Amphipod>> = vec![
        None,              // 0
        None,              // 1
        None,              // 2
        None,              // 3
        None,              // 4
        None,              // 5
        None,              // 6
        Some(Amphipod::A), // 7
        Some(Amphipod::B), // 8
        Some(Amphipod::C), // 9
        Some(Amphipod::D), // A
        Some(Amphipod::A), // B
        Some(Amphipod::B), // C
        Some(Amphipod::C), // D
        Some(Amphipod::D), // E
        Some(Amphipod::A), // F
        Some(Amphipod::B), // G
        Some(Amphipod::C), // H
        Some(Amphipod::D), // I
        Some(Amphipod::A), // J
        Some(Amphipod::B), // K
        Some(Amphipod::C), // L
        Some(Amphipod::D), // M
    ];

    static ref A_WEIGHT: Vec<usize> = vec![
        3, 2, 2, 4, 6, 8, 9, 0, 4, 6, 8, 0, 5, 7, 9, 0, 6, 8, 10, 0, 7, 9, 11,
    ];

    static ref B_WEIGHT: Vec<usize> = vec![
        5, 4, 2, 2, 4, 6, 7, 4, 0, 4, 6, 5, 0, 5, 7, 6, 0, 6, 8, 7, 0, 7, 9,
    ];

    static ref C_WEIGHT: Vec<usize> = vec![
        7, 6, 4, 2, 2, 4, 5, 6, 4, 0, 4, 7, 5, 0, 5, 8, 6, 0, 6, 9, 7, 0, 7,
    ];

    static ref D_WEIGHT: Vec<usize> = vec![
        9, 8, 6, 4, 2, 2, 3, 8, 6, 4, 0, 9, 7, 5, 0, 11, 9, 7, 0, 10, 6, 8, 0,
    ];
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Graph {
    node_content: Vec<Option<Amphipod>>,
}

impl Graph {
    fn in_place(&self, node: usize, p2: bool) -> bool {
        match self.node_content[node] {
            Some(Amphipod::A) => {
                match node {
                    7 | 11 | 15 | 19 => {
                        // ensure none of 7/11/15/19 contain non-A nodes
                        matches!(self.node_content[7], Some(Amphipod::A) | None)
                            && matches!(self.node_content[11], Some(Amphipod::A) | None)
                            && (!p2
                                || (matches!(self.node_content[15], Some(Amphipod::A) | None)
                                    && matches!(self.node_content[19], Some(Amphipod::A) | None)))
                    }
                    _ => false,
                }
            }
            Some(Amphipod::B) => {
                match node {
                    8 | 12 | 16 | 20 => {
                        // ensure none of 8/12/16/20 contain non-B nodes
                        matches!(self.node_content[8], Some(Amphipod::B) | None)
                            && matches!(self.node_content[12], Some(Amphipod::B) | None)
                            && (!p2
                                || (matches!(self.node_content[16], Some(Amphipod::B) | None)
                                    && matches!(self.node_content[20], Some(Amphipod::B) | None)))
                    }
                    _ => false,
                }
            }
            Some(Amphipod::C) => {
                match node {
                    9 | 13 | 17 | 21 => {
                        // ensure none of 9/13/17/21 contain non-C nodes
                        matches!(self.node_content[9], Some(Amphipod::C) | None)
                            && matches!(self.node_content[13], Some(Amphipod::C) | None)
                            && (!p2
                                || (matches!(self.node_content[17], Some(Amphipod::C) | None)
                                    && matches!(self.node_content[21], Some(Amphipod::C) | None)))
                    }
                    _ => false,
                }
            }
            Some(Amphipod::D) => {
                match node {
                    10 | 14 | 18 | 22 => {
                        // ensure none of 10/14/18/22 contain non-D nodes
                        matches!(self.node_content[10], Some(Amphipod::D) | None)
                            && matches!(self.node_content[14], Some(Amphipod::D) | None)
                            && (!p2
                                || (matches!(self.node_content[18], Some(Amphipod::D) | None)
                                    && matches!(self.node_content[22], Some(Amphipod::D) | None)))
                    }
                    _ => false,
                }
            }
            None => false,
        }
    }

    // TODO: This can search more deeply in order to guarantee we get as close to dest as possible
    fn moves_from(&self, node: usize, deepest: [usize; 4], p2: bool) -> Vec<(Self, usize)> {
        // _seen_states.insert(self.clone());

        if !p2 {
            assert!(node < 15);
        }

        // check if node is in place
        if self.in_place(node, p2) {
            return Vec::new();
        }

        // for all accessible nodes from this node
        NODE_PATHS[node]
            .iter()
            // p1 filter
            .filter(|path| p2 || path[0] < 15)
            // must be unblocked
            .filter(|path| path.iter().all(|node| self.node_content[*node].is_none()))
            // only care about target
            .map(|path| path[0])
            // filter to only nodes we can skip to
            .filter(|id| match self.node_content[node] {
                Some(Amphipod::A) => A_SKIP_GRAPH[(node, *id)] != 0,
                Some(Amphipod::B) => B_SKIP_GRAPH[(node, *id)] != 0,
                Some(Amphipod::C) => C_SKIP_GRAPH[(node, *id)] != 0,
                Some(Amphipod::D) => D_SKIP_GRAPH[(node, *id)] != 0,
                None => false,
            })
            // if we are targeting a hole, filter to only try and place into the deepest spot
            .filter(|id| match id {
                7.. => deepest.contains(id),
                _ => true,
            })
            .map(|target| {
                let weight = SKIP_GRAPH[(node, target)];
                let mut new_state = self.clone();
                new_state.node_content[target] = self.node_content[node];
                new_state.node_content[node] = None;

                let cost = match self.node_content[node] {
                    Some(Amphipod::A) => weight * 1,
                    Some(Amphipod::B) => weight * 10,
                    Some(Amphipod::C) => weight * 100,
                    Some(Amphipod::D) => weight * 1000,
                    None => 0,
                };

                (new_state, cost)
            })
            //.filter(|(state, _)| !_seen_states.contains(state))
            .collect()
    }
}

fn node_str(node: Option<Amphipod>) -> String {
    match node {
        Some(Amphipod::A) => "A".to_owned(),
        Some(Amphipod::B) => "B".to_owned(),
        Some(Amphipod::C) => "C".to_owned(),
        Some(Amphipod::D) => "D".to_owned(),
        None => ".".to_owned(),
    }
}

fn print_graph(state: &Graph, p2: bool) {
    let inner = if p2 {
        format!(
            "\n  #{}#{}#{}#{}#\n  #{}#{}#{}#{}#",
            node_str(state.node_content[15]),
            node_str(state.node_content[16]),
            node_str(state.node_content[17]),
            node_str(state.node_content[18]),
            node_str(state.node_content[19]),
            node_str(state.node_content[20]),
            node_str(state.node_content[21]),
            node_str(state.node_content[22])
        )
    } else {
        "".to_string()
    };

    println!(
        "#############\n#{}{}.{}.{}.{}.{}{}#\n###{}#{}#{}#{}###\n  #{}#{}#{}#{}#{}\n  #########",
        node_str(state.node_content[0]),
        node_str(state.node_content[1]),
        node_str(state.node_content[2]),
        node_str(state.node_content[3]),
        node_str(state.node_content[4]),
        node_str(state.node_content[5]),
        node_str(state.node_content[6]),
        node_str(state.node_content[7]),
        node_str(state.node_content[8]),
        node_str(state.node_content[9]),
        node_str(state.node_content[10]),
        node_str(state.node_content[11]),
        node_str(state.node_content[12]),
        node_str(state.node_content[13]),
        node_str(state.node_content[14]),
        inner
    );
}

fn parse_input(_input: &str) -> Graph {
    let node_content = vec![
        None,              // 0
        None,              // 1
        None,              // 2
        None,              // 3
        None,              // 4
        None,              // 5
        None,              // 6
        Some(Amphipod::D), // 7
        Some(Amphipod::D), // 8
        Some(Amphipod::A), // 9
        Some(Amphipod::A), // A
        Some(Amphipod::C), // B
        Some(Amphipod::C), // C
        Some(Amphipod::B), // D
        Some(Amphipod::B), // E
    ];

    Graph { node_content }
}

fn success(state: &Graph) -> bool {
    GOAL_STATE
        .iter()
        .zip(&state.node_content)
        .all(|(a, b)| *a == *b)
}

fn astar_successors(state: &Graph, p2: bool) -> Vec<(Graph, usize)> {
    // println!("checking state:");
    // print_graph(state, p2);

    let deepest_a = {
        let a_tunnel = [19, 15, 11, 7];
        if a_tunnel
            .iter()
            .filter(|id| p2 || **id < 15)
            .any(|id| !matches!(state.node_content[*id], Some(Amphipod::A) | None))
        {
            0
        } else {
            *a_tunnel
                .iter()
                .filter(|id| p2 || **id < 15)
                .find(|id| matches!(state.node_content[**id], None))
                .unwrap_or(&0)
        }
    };

    let deepest_b = {
        let b_tunnel = [20, 16, 12, 8];

        if b_tunnel
            .iter()
            .filter(|id| p2 || **id < 15)
            .any(|id| !matches!(state.node_content[*id], Some(Amphipod::B) | None))
        {
            0
        } else {
            *b_tunnel
                .iter()
                .filter(|id| p2 || **id < 15)
                .find(|id| matches!(state.node_content[**id], None))
                .unwrap_or(&0)
        }
    };

    let deepest_c = {
        let c_tunnel = [21, 17, 13, 9];
        if c_tunnel
            .iter()
            .filter(|id| p2 || **id < 15)
            .any(|id| !matches!(state.node_content[*id], Some(Amphipod::C) | None))
        {
            0
        } else {
            *c_tunnel
                .iter()
                .filter(|id| p2 || **id < 15)
                .find(|id| matches!(state.node_content[**id], None))
                .unwrap_or(&0)
        }
    };

    let deepest_d = {
        let d_tunnel = [22, 18, 14, 10];
        if d_tunnel
            .iter()
            .filter(|id| p2 || **id < 15)
            .any(|id| !matches!(state.node_content[*id], Some(Amphipod::D) | None))
        {
            0
        } else {
            *d_tunnel
                .iter()
                .filter(|id| p2 || **id < 15)
                .find(|id| matches!(state.node_content[**id], None))
                .unwrap_or(&0)
        }
    };

    let deepest = [deepest_a, deepest_b, deepest_c, deepest_d];

    state
        .node_content
        .iter()
        .enumerate()
        .filter(|(_, v)| v.is_some())
        .flat_map(|(id, _)| state.moves_from(id, deepest, p2))
        .collect::<Vec<_>>()
}

fn heuristic(state: &Graph) -> usize {
    let mut a_score = 0;
    let mut a_adds = 0;
    let mut b_score = 0;
    let mut b_adds = 0;
    let mut c_score = 0;
    let mut c_adds = 0;
    let mut d_score = 0;
    let mut d_adds = 0;

    for (node, content) in state.node_content.iter().enumerate() {
        match content {
            Some(Amphipod::A) => {
                a_score += A_WEIGHT[node];
                a_adds += 1;
            }
            Some(Amphipod::B) => {
                b_score += B_WEIGHT[node];
                b_adds += 1;
            }
            Some(Amphipod::C) => {
                c_score += C_WEIGHT[node];
                c_adds += 1;
            }
            Some(Amphipod::D) => {
                d_score += D_WEIGHT[node];
                d_adds += 1;
            }
            None => {}
        }
    }

    a_score
        + b_score
        + c_score
        + d_score
        + a_adds * (a_adds - 1) / 2
        + b_adds * (b_adds - 1) / 2
        + c_adds * (c_adds - 1) / 2
        + d_adds * (d_adds - 1) / 2
}

fn solve_p1(target: &Graph) -> usize {
    let soln = solve(target, false);
    match soln {
        Some((soln, score)) => {
            for (id, state) in soln.iter().enumerate() {
                println!("{}.", id,);
                print_graph(state, false);
            }
            score
        }
        None => 0,
    }
}

fn solve(target: &Graph, p2: bool) -> Option<(Vec<Graph>, usize)> {
    astar(
        target,
        |state| astar_successors(state, p2),
        heuristic,
        success,
    )
}

/*
############# | #############
#...........# | #01.2.3.4.56#
###D#D#A#A### | ###7#8#9#A###
  #D#C#B#A#   |   #B#C#D#E#
  #D#B#A#C#   |   #F#G#H#I#
  #C#C#B#B#   |   #J#K#L#M#
  #########   |   #########
*/

fn adj_input_p2(target: &Graph) -> Graph {
    let mut vec = target.node_content.clone();
    vec.insert(11, Some(Amphipod::D));
    vec.insert(12, Some(Amphipod::C));
    vec.insert(13, Some(Amphipod::B));
    vec.insert(14, Some(Amphipod::A));
    vec.insert(15, Some(Amphipod::D));
    vec.insert(16, Some(Amphipod::B));
    vec.insert(17, Some(Amphipod::A));
    vec.insert(18, Some(Amphipod::C));

    Graph { node_content: vec }
}

fn solve_p2(target: &Graph) -> usize {
    std::thread::sleep(std::time::Duration::from_secs(5));
    let target = &adj_input_p2(target);
    let soln = solve(target, true);
    match soln {
        Some((soln, score)) => {
            for (id, state) in soln.iter().enumerate() {
                println!("{}. ", id);
                print_graph(state, true);
            }
            score
        }
        None => 0,
    }
}

#[aoc_generator(day23)]
pub fn input_generator(input: &str) -> Graph {
    parse_input(input)
}

#[aoc(day23, part1)]
pub fn wrapper_p1(input: &Graph) -> usize {
    solve_p1(input)
}

#[aoc(day23, part2)]
pub fn wrapper_p2(input: &Graph) -> usize {
    solve_p2(input)
}

#[cfg(test)]
mod tests {
    use ndarray::Array2;

    fn test_input() -> super::Graph {
        let node_content = vec![
            None,                     // 0
            None,                     // 1
            None,                     // 2
            None,                     // 3
            None,                     // 4
            None,                     // 5
            None,                     // 6
            Some(super::Amphipod::B), // 7
            Some(super::Amphipod::C), // 8
            Some(super::Amphipod::B), // 9
            Some(super::Amphipod::D), // A
            Some(super::Amphipod::A), // B
            Some(super::Amphipod::D), // C
            Some(super::Amphipod::C), // D
            Some(super::Amphipod::A), // E
        ];
        /*
        ############# | #############
        #...........# | #01.2.3.4.56#
        ###D#D#A#A### | ###7#8#9#A###
          #C#C#B#B#   |   #B#C#D#E#
          #########   |   #########
        */

        super::Graph { node_content }
    }

    #[test]
    fn test_matmul() {
        let mut test_mat = Array2::<usize>::zeros((23, 23));

        ndarray::linalg::general_mat_mul(1, &super::GRAPH, &super::GRAPH, 0, &mut test_mat);

        println!("{}", super::A_SKIP_GRAPH.row(0));
    }

    #[test]
    fn test_p1() {
        let test_input = test_input();

        assert_eq!(12521, super::solve_p1(&test_input));
    }

    #[test]
    fn test_p2() {
        let test_input = test_input();

        assert_eq!(44169, super::solve_p2(&test_input));
    }
}
