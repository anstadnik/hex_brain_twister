use std::iter::{empty, once};

use indicatif::{ProgressBar, ProgressIterator};
use itertools::Itertools;

const H: usize = 5;
const W: usize = 9;

const ACTIVE_TILES: [[u8; W]; H] = [
    [0, 0, 1, 0, 1, 0, 1, 0, 0],
    [0, 1, 0, 1, 0, 1, 0, 1, 0],
    [1, 0, 1, 0, 1, 0, 1, 0, 1],
    [0, 1, 0, 1, 0, 1, 0, 1, 0],
    [0, 0, 1, 0, 1, 0, 1, 0, 0],
];

fn get_active_tiles_coords() -> Vec<(usize, usize)> {
    (0..ACTIVE_TILES.len())
        .cartesian_product(0..ACTIVE_TILES[0].len())
        .filter(|&(y, x)| ACTIVE_TILES[y][x] == 1)
        .collect()
}

#[derive(Debug, Default, Clone, Copy)]
struct Puzzle<'a> {
    tiles: [[Option<u8>; W]; H],
    used: [bool; 19],
    coords: &'a [(usize, usize)],
}

impl<'a> Puzzle<'a> {
    fn gen_all_puzzles(self) -> Box<dyn Iterator<Item = Self> + 'a> {
        if self.used.iter().all(|&used| used) {
            return Box::new(once(self));
        }
        if !self.check_solution() {
            return Box::new(empty());
        }
        Box::new(
            self.coords
                .iter()
                .filter(move |&&(y, x)| self.coords.contains(&(y, x)) && self.tiles[y][x].is_none())
                .flat_map(move |&(y, x)| {
                    (0..self.used.len())
                        .filter(move |&i| !self.used[i])
                        .map(move |i| (y, x, i))
                })
                .flat_map(move |(y, x, i)| {
                    let mut new_puzzle = self;
                    new_puzzle.tiles[y][x] = Some(i as u8 + 1);
                    new_puzzle.used[i] = true;
                    let ret = new_puzzle.gen_all_puzzles();
                    if !self.used.iter().any(|&used| used) {
                        println!(".");
                    }
                    ret
                }),
        )
    }

    /// Visual representation of the puzzle grid:
    /// ```
    ///     x x x
    ///    x x x x
    ///   x x x x x
    ///    x x x x
    ///     x x x
    /// ```
    /// Where 'x' represents active tile positions

    fn check_solution(self) -> bool {
        fn get_sum(p: &Puzzle, (y, x): (usize, usize), (dy, dx): (isize, isize)) -> Option<u8> {
            let get_available = |y: usize, x: usize| ACTIVE_TILES.get(y)?.get(x);
            (0..)
                .map(|i| ((y as isize + i * dy) as _, (x as isize + i * dx) as _))
                .map_while(|(y, x)| {
                    get_available(y, x)
                        .inspect(|v| assert!(**v == 1))
                        .map(|_| p.tiles[y][x])
                })
                .try_fold(0, |acc, v| v.map(|v| acc + v))
        }

        let left_lines = ([(0, 2), (0, 4), (0, 6), (1, 7), (2, 8)], (1, -1));
        let right_lines = ([(2, 0), (1, 1), (0, 2), (0, 4), (0, 6)], (1, 1));
        let horizontal_lines = ([(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)], (0, 2));
        [left_lines, right_lines, horizontal_lines]
            .iter()
            .flat_map(|&(start, step)| {
                start
                    .into_iter()
                    .filter_map(move |(y, x)| get_sum(&self, (y, x), step))
            })
            .all(|sum| sum == 38)
    }
}

fn main() {
    let active_tiles_coords = get_active_tiles_coords();
    let puzzle = Puzzle {
        tiles: [[None; W]; H],
        used: [false; 19],
        coords: &active_tiles_coords,
    };
    let all_puzzles = puzzle.gen_all_puzzles();
    println!(
        "{:?}",
        all_puzzles
            .progress_with(ProgressBar::new_spinner())
            .count()
    );
}
