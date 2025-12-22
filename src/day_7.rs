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

#[derive(PartialEq)]
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
    let mut task_2_counts: Vec<Vec<usize>> = Vec::new();

    // If cell is in bounds and empty, set to beam
    let set_beam = |i: usize,
                    j: usize,
                    manifold: &mut Vec<Vec<Cell>>,
                    task_2: &mut Vec<Vec<usize>>,
                    task_2_parent_count: usize| {
        if let Some(cell) = manifold.get_mut(i).and_then(|row| row.get_mut(j)) {
            match cell {
                Cell::Empty => {
                    *cell = Cell::Beam;
                    task_2[i][j] += task_2_parent_count;
                }
                Cell::Beam => {
                    // Already a beam, just increment count
                    task_2[i][j] += task_2_parent_count;
                }
                Cell::Splitter => (), // This case is quite unexpected, ignore it and don't propagate beam
            }
        }
    };

    // Initialise task 2 counts structure
    for i in 0..puzzle.manifold.len() {
        task_2_counts.push(vec![0; puzzle.manifold[i].len()]);

        if i == 0 {
            // Initialise first counters at first row
            for (j, cell) in puzzle.manifold[i].iter().enumerate() {
                if cell == &Cell::Beam {
                    task_2_counts[0][j] = 1;
                }
            }
        }
    }

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
                                task_2_counts[i + 1][j] += task_2_counts[i][j];
                            }
                            Cell::Splitter => {
                                // Splitter below, split beam
                                task_1 += 1;
                                let task_2_count = task_2_counts[i][j];
                                set_beam(
                                    i + 1,
                                    j - 1,
                                    &mut puzzle.manifold,
                                    &mut task_2_counts,
                                    task_2_count,
                                );
                                set_beam(
                                    i + 1,
                                    j + 1,
                                    &mut puzzle.manifold,
                                    &mut task_2_counts,
                                    task_2_count,
                                );
                            }
                            Cell::Beam => {
                                // Already a beam below, just propagate count
                                task_2_counts[i + 1][j] += task_2_counts[i][j];
                            }
                        }
                    }
                }
            }
        }
    }

    // Can print final manifold for visualisation and debugging
    // println!("Final manifold:\n{}", puzzle);
    // pretty_print_task_2_counts(&puzzle.manifold, &task_2_counts);

    let task_2 = task_2_counts.last().map(|c| c.iter().sum()).unwrap_or(0);

    Solution { task_1, task_2 }
}

fn _pretty_print_task_2_counts(final_manifold: &[Vec<Cell>], counts: &[Vec<usize>]) {
    for i in 0..final_manifold.len() {
        for j in 0..final_manifold[i].len() {
            let cell = &final_manifold[i][j];
            match cell {
                Cell::Beam => print!("{}", counts[i][j]),
                other => print!("{}", other),
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Task 2 beam count visualisation:
    // .......S.......
    // .......1....... sum -> 1
    // ......1^1...... sum -> 2
    // ......1.1...... sum -> 2
    // .....1^2^1..... sum -> 4
    // .....1.2.1..... sum -> 4
    // ....1^3^3^1.... sum -> 8
    // ....1.3.3.1.... sum -> 8
    // ...1^4^331^1... sum -> 13
    // ...1.4.331.1... sum -> 13
    // ..1^5^434^2^1.. sum -> 20
    // ..1.5.434.2.1.. sum -> 20
    // .1^154^74.21^1. sum -> 26
    // .1.154.74.21.1. sum -> 26
    // 1^2^a^b^b^211^1 sum -> 40
    // ...............

    #[test]
    fn test_increment_example_1() {
        let input = "
.......S.......
...............
"
        .trim();
        let puzzle = parse_puzzle(input).unwrap();
        let solution = solve_puzzle(puzzle);
        assert_eq!(solution.task_1, 0);
        assert_eq!(solution.task_2, 1);
    }

    #[test]
    fn test_increment_example_2() {
        let input = "
.......S.......
...............
.......^.......
...............
"
        .trim();
        let puzzle = parse_puzzle(input).unwrap();
        let solution = solve_puzzle(puzzle);
        assert_eq!(solution.task_1, 1);
        assert_eq!(solution.task_2, 2);
    }

    #[test]
    fn test_increment_example_3() {
        let input = "
.......S.......
...............
.......^.......
...............
......^.^......
...............
"
        .trim();
        let puzzle = parse_puzzle(input).unwrap();
        let solution = solve_puzzle(puzzle);
        assert_eq!(solution.task_1, 3);
        assert_eq!(solution.task_2, 4);
    }

    #[test]
    fn test_increment_example_4() {
        let input = "
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
"
        .trim();
        let puzzle = parse_puzzle(input).unwrap();
        let solution = solve_puzzle(puzzle);
        assert_eq!(solution.task_1, 6);
        assert_eq!(solution.task_2, 8);
    }

    #[test]
    fn test_increment_example_5() {
        let input = "
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
"
        .trim();
        let puzzle = parse_puzzle(input).unwrap();
        let solution = solve_puzzle(puzzle);
        assert_eq!(solution.task_1, 9);
        assert_eq!(solution.task_2, 13);
    }

    #[test]
    fn test_increment_example_6() {
        let input = "
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
"
        .trim();
        let puzzle = parse_puzzle(input).unwrap();
        let solution = solve_puzzle(puzzle);
        assert_eq!(solution.task_1, 13);
        assert_eq!(solution.task_2, 20);
    }

    #[test]
    fn test_increment_example_7() {
        let input = "
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
"
        .trim();
        let puzzle = parse_puzzle(input).unwrap();
        let solution = solve_puzzle(puzzle);
        assert_eq!(solution.task_1, 16);
        assert_eq!(solution.task_2, 26);
    }

    #[test]
    fn test_example_puzzle() {
        let input = include_str!("../inputs/examples/day_7");
        let puzzle = parse_puzzle(input).unwrap();
        let solution = solve_puzzle(puzzle);
        assert_eq!(solution.task_1, 21);
        assert_eq!(solution.task_2, 40);
    }

    #[test]
    fn test_real_input() {
        let input = include_str!("../inputs/day_7");
        let puzzle = parse_puzzle(input).unwrap();
        let solution = solve_puzzle(puzzle);
        assert_eq!(solution.task_1, 1626);
        assert_eq!(solution.task_2, 48989920237096);
    }
}
