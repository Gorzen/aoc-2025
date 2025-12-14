use crate::common::Solution;
use anyhow::{Result, anyhow};

pub struct Puzzle {
    diagram: Vec<Vec<Position>>,
}

#[derive(PartialEq, Clone)]
enum Position {
    Empty,
    Paper,
}

fn parse_position(c: char) -> Result<Position> {
    match c {
        '.' => Ok(Position::Empty),
        '@' => Ok(Position::Paper),
        other => Err(anyhow!(
            "Invalid Position character {}. Expected '.' or '@'.",
            other
        )),
    }
}

pub fn parse_puzzle(input: &str) -> Result<Puzzle> {
    let diagram = input
        .lines()
        .map(|line| {
            line.chars()
                .map(parse_position)
                .collect::<Result<Vec<Position>>>()
        })
        .collect::<Result<Vec<Vec<Position>>>>()?;

    Ok(Puzzle { diagram })
}

/// Pass `puzzle` as value and not reference to consume it (to avoid the caller reusing a modified puzzle afterwards).
/// Returns (number of papers removed in first pass, number of papers removed in total)
pub fn solve_puzzle(mut puzzle: Puzzle) -> Solution {
    let mut removed_rounds: Vec<usize> = Vec::new();

    let mut done = false;

    while !done {
        let removed = remove_accessible_papers(&mut puzzle);
        removed_rounds.push(removed);
        done = removed == 0;
    }

    let total_removed = removed_rounds.iter().sum();
    let first_pass = removed_rounds.first().unwrap_or(&0);

    Solution {
        task_1: *first_pass,
        task_2: total_removed,
    }
}

/// Returns the number of papers removed
fn remove_accessible_papers(puzzle: &mut Puzzle) -> usize {
    let mut positions_to_remove: Vec<(usize, usize)> = Vec::new();

    for (x, row) in puzzle.diagram.iter().enumerate() {
        for (y, pos) in row.iter().enumerate() {
            // If pos is accessible paper, mark it as 'to be removed'
            if pos == &Position::Paper && is_accessible(puzzle, x, y) {
                positions_to_remove.push((x, y));
            }
        }
    }

    // Remove positions
    // Note: Can't do in 1 loop as otherwise the borrow checker complains because
    // there is an immutable borrow (to call `is_accessible`) inside the mutable borrow of the loop
    for (x, y) in positions_to_remove.iter() {
        puzzle.diagram[*x][*y] = Position::Empty;
    }

    positions_to_remove.len()
}

/// Less than 4 papers in adjacent positions
fn is_accessible(puzzle: &Puzzle, x: usize, y: usize) -> bool {
    let mut adjacent_papers = 0;

    let x_i32 = x as i32;
    let y_i32 = y as i32;

    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                // Ignore self
                continue;
            }

            let x = x_i32 + i;
            let y = y_i32 + j;

            if x >= 0
                && let Some(row) = puzzle.diagram.get(x as usize)
                && y >= 0
                && let Some(pos) = row.get(y as usize)
                && pos == &Position::Paper
            {
                adjacent_papers += 1;
            }
        }
    }

    adjacent_papers < 4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_puzzle() {
        let input = "
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
        "
        .trim();

        let puzzle = parse_puzzle(input).unwrap();
        let solution = solve_puzzle(puzzle);
        assert_eq!(solution.task_1, 13);
        assert_eq!(solution.task_2, 43);
    }

    #[test]
    fn test_is_accessible() {
        let input = "
.@@
@@.
        "
        .trim();

        let puzzle = parse_puzzle(input).unwrap();
        let solution = solve_puzzle(puzzle);
        assert_eq!(solution.task_1, 4);
        assert_eq!(solution.task_2, 4);
    }

    #[test]
    fn test_real_input() {
        let input = include_str!("../inputs/day_4");
        let puzzle = parse_puzzle(input).unwrap();
        let solution = solve_puzzle(puzzle);
        assert_eq!(solution.task_1, 1349);
        assert_eq!(solution.task_2, 8277);
    }
}
