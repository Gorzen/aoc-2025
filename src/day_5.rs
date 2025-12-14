use crate::{common::Solution, range::Range, range::merge_ranges};
use anyhow::{Context, Result};

pub struct Puzzle {
    fresh_ingredients: Vec<Range>,
    available_ingredients: Vec<usize>,
}

pub fn parse_puzzle(input: &str) -> Result<Puzzle> {
    let mut reading_fresh_ingredients = true;

    let mut fresh_ingredients = Vec::new();
    let mut available_ingredients = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            // Empty line, we are now reading the available ingredients
            reading_fresh_ingredients = false;
        } else if reading_fresh_ingredients {
            let range = Range::parse(line)?;
            fresh_ingredients.push(range);
        } else {
            let id: usize = line
                .parse()
                .with_context(|| format!("Failed to parse available ingredient: '{}'", line))?;

            available_ingredients.push(id);
        }
    }

    Ok(Puzzle {
        fresh_ingredients,
        available_ingredients,
    })
}

pub fn solve_puzzle(puzzle: Puzzle) -> Solution {
    // Merge ranges to have non-overlapping ranges
    let fresh_ingredients = merge_ranges(puzzle.fresh_ingredients);

    // Count number of available ingredients that are fresh
    let task_1 = puzzle
        .available_ingredients
        .iter()
        .filter(|id| is_fresh(&fresh_ingredients, **id))
        .count();

    // Compute number of fresh ingredients assuming ranges do not overlap
    let mut task_2 = 0;
    for range in fresh_ingredients {
        task_2 += range.count();
    }

    Solution { task_1, task_2 }
}

fn is_fresh(fresh_ingredients: &[Range], id: usize) -> bool {
    fresh_ingredients.iter().any(|range| range.is_included(id))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_puzzle() {
        let input = "
3-5
10-14
16-20
12-18

1
5
8
11
17
32
        "
        .trim();
        let puzzle = parse_puzzle(input).unwrap();
        let solution = solve_puzzle(puzzle);
        assert_eq!(solution.task_1, 3);
        assert_eq!(solution.task_2, 14);
    }

    #[test]
    fn test_real_input() {
        let input = include_str!("../inputs/day_5");
        let puzzle = parse_puzzle(input).unwrap();
        let solution = solve_puzzle(puzzle);
        assert_eq!(solution.task_1, 577);
        assert_eq!(solution.task_2, 350513176552950);
    }
}
