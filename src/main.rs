use itertools::Itertools;
use std::fmt::Display;
use std::iter::{empty, once, successors};

const H: usize = 5;
const W: usize = 9;

const ACTIVE_TILES: [[u8; W]; H] = [
    [0, 0, 1, 0, 1, 0, 1, 0, 0],
    [0, 1, 0, 1, 0, 1, 0, 1, 0],
    [1, 0, 1, 0, 1, 0, 1, 0, 1],
    [0, 1, 0, 1, 0, 1, 0, 1, 0],
    [0, 0, 1, 0, 1, 0, 1, 0, 0],
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Unused,
    Used(u8),
}
use Tile::*;

#[derive(Debug, Clone, Copy)]
struct Puzzle {
    tiles: [[Tile; W]; H],
    used: [bool; 19],
}

impl Default for Puzzle {
    fn default() -> Self {
        Self {
            tiles: ACTIVE_TILES.map(|row| row.map(|v| if v == 1 { Unused } else { Empty })),
            used: [false; 19],
        }
    }
}

impl Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.tiles {
            for tile in row {
                match tile {
                    Empty => write!(f, "  ")?,
                    Unused => write!(f, "··")?,
                    Used(n) => write!(f, "{:2}", n)?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Puzzle {
    fn gen_all_puzzles(self) -> Box<dyn Iterator<Item = Self>> {
        if !self.check_solution() {
            return Box::new(empty());
        }
        if let Some((y, x)) = (0..H)
            .cartesian_product(0..W)
            .find(|&(y, x)| self.tiles[y][x] == Unused)
        {
            Box::new(
                self.used
                    .into_iter()
                    .enumerate()
                    .filter(|&(_, used)| !used)
                    .flat_map(move |(i, _)| {
                        let mut new_puzzle = self;
                        new_puzzle.tiles[y][x] = Used(i as u8 + 1);
                        new_puzzle.used[i] = true;
                        new_puzzle.gen_all_puzzles()
                    }),
            )
        } else {
            Box::new(once(self))
        }
    }

    fn check_solution(self) -> bool {
        let left_lines = ([(0, 2), (0, 4), (0, 6), (1, 7), (2, 8)], (1, -1));
        let right_lines = ([(2, 0), (1, 1), (0, 2), (0, 4), (0, 6)], (1, 1));
        let horizontal_lines = ([(0, 2), (1, 1), (2, 0), (3, 1), (4, 2)], (0, 2));
        [horizontal_lines, left_lines, right_lines]
            .iter()
            .flat_map(|(starts, step)| {
                starts
                    .iter()
                    .filter_map(|&start| self.try_sum(start, *step))
            })
            .all(|sum| sum == 38)
    }

    fn try_sum(self, (y, x): (usize, usize), (dy, dx): (isize, isize)) -> Option<u8> {
        successors(Some((y, x)), |&(y, x)| {
            Some((y.checked_add_signed(dy)?, x.checked_add_signed(dx)?))
        })
        .map_while(|(y, x)| self.tiles.get(y)?.get(x))
        .try_fold(0, |acc, &tile| match tile {
            Used(v) => Some(acc + v),
            Unused => None,
            Empty => Some(acc),
        })
    }
}

fn main() {
    Puzzle::default()
        .gen_all_puzzles()
        .for_each(|solution| println!("{solution}"));
}
