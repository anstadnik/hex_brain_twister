use std::iter::once;

use indicatif::{ProgressBar, ProgressIterator};
use itertools::Itertools;

const H: usize = 9;
const W: usize = 5;

const ACTIVE_TILES: [[u8; W]; H] = [
    [0, 0, 1, 0, 0],
    [0, 1, 0, 1, 0],
    [1, 0, 1, 0, 1],
    [0, 1, 0, 1, 0],
    [1, 0, 1, 0, 1],
    [0, 1, 0, 1, 0],
    [1, 0, 1, 0, 1],
    [0, 1, 0, 1, 0],
    [0, 0, 1, 0, 0],
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
                    new_puzzle.gen_all_puzzles()
                }),
        )
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
