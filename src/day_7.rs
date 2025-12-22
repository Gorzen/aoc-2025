use anyhow::Result;
use std::fmt::Display;

use crate::common::Solution;

pub struct Puzzle {
    manifold: Vec<Vec<Cell>>,
}

impl Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.manifold {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

enum Cell {
    Empty,
    Beam,
    Splitter,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::Beam => write!(f, "|"),
            Cell::Splitter => write!(f, "^"),
        }
    }
}

impl TryFrom<char> for Cell {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            '.' => Ok(Cell::Empty),
            'S' => Ok(Cell::Beam), // Start of beam
            '^' => Ok(Cell::Splitter),
            _ => Err(anyhow::anyhow!("Invalid cell character: '{}'", value)),
        }
    }
}

pub fn parse_puzzle(input: &str) -> Result<Puzzle> {
    let manifold = input
        .lines()
        .map(|line| {
            line.chars()
                .map(Cell::try_from)
                .collect::<Result<Vec<Cell>>>()
        })
        .collect::<Result<Vec<Vec<Cell>>>>()?;

    Ok(Puzzle { manifold })
}

pub fn solve_puzzle(mut puzzle: Puzzle) -> Solution {
    let mut task_1 = 0;

    let set_beam = |i: usize, j: usize, manifold: &mut Vec<Vec<Cell>>| {
        if let Some(Cell::Empty) = manifold.get(i).and_then(|row| row.get(j)) {
            manifold[i][j] = Cell::Beam;
        }
    };

    // Loop for beam propagation
    for i in 0..puzzle.manifold.len() {
        for j in 0..puzzle.manifold[i].len() {
            let cell = &puzzle.manifold[i][j];
            match cell {
                Cell::Empty => (),
                Cell::Splitter => (),
                Cell::Beam => {
                    // If beam, propagate downwards
                    // Need to check what is below: empty or splitter
                    if let Some(cell_below) = puzzle
                        .manifold
                        .get_mut(i + 1)
                        .and_then(|row| row.get_mut(j))
                    {
                        match cell_below {
                            Cell::Empty => {
                                // Empty below, continue beam downwards
                                *cell_below = Cell::Beam;
                            }
                            Cell::Splitter => {
                                // Splitter below, split beam
                                task_1 += 1;
                                set_beam(i + 1, j - 1, &mut puzzle.manifold);
                                set_beam(i + 1, j + 1, &mut puzzle.manifold);
                            }
                            Cell::Beam => (), // Beam below, do nothing
                        }
                    }
                }
            }
        }
    }

    // Can print final manifold for visualisation and debugging
    // println!("Final manifold:\n{}", puzzle);

    Solution { task_1, task_2: 0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_puzzle() {
        let input = include_str!("../inputs/examples/day_7");
        let puzzle = parse_puzzle(input).unwrap();
        let solution = solve_puzzle(puzzle);
        assert_eq!(solution.task_1, 21);
        assert_eq!(solution.task_2, 0);
    }

    #[test]
    fn test_real_input() {
        let input = include_str!("../inputs/day_7");
        let puzzle = parse_puzzle(input).unwrap();
        let solution = solve_puzzle(puzzle);
        assert_eq!(solution.task_1, 1626);
        assert_eq!(solution.task_2, 0);
    }
}
