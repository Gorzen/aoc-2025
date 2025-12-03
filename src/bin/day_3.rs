use anyhow::{Context, Result};
use std::io::{self, Read};

fn main() {
    let mut input_buffer = String::new();

    println!("Reading input from stdin...");
    if let Err(e) = io::stdin().read_to_string(&mut input_buffer) {
        eprintln!("Error reading input: {}", e);
        return;
    }

    let puzzle = match parse_puzzle(&input_buffer) {
        Ok(puzzle) => puzzle,
        Err(e) => {
            eprintln!("Error parsing puzzle: {}", e);
            return;
        }
    };

    if puzzle.banks.iter().any(|bank| bank.batteries.len() < 12) {
        eprintln!("Expected a puzzle where all banks (lines) have at least 12 batteries");
        return;
    }

    let solution_2 = solve_puzzle(&puzzle, 2);
    let solution_12 = solve_puzzle(&puzzle, 12);
    println!("Solution with 2 batteries on: {}", solution_2);
    println!("Solution with 12 batteries on: {}", solution_12);
}

#[derive(Debug, PartialEq)]
struct Puzzle {
    banks: Vec<Bank>,
}

#[derive(Debug, PartialEq)]
struct Bank {
    batteries: Vec<usize>,
}

fn parse_puzzle(input: &str) -> Result<Puzzle> {
    let banks = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| {
                    ch.to_digit(10)
                        .with_context(|| format!("Failed to parse digit: '{}'", ch))
                        .map(|d| d as usize)
                })
                .collect::<Result<Vec<usize>>>()
                .map(|batteries| Bank { batteries })
        })
        .collect::<Result<Vec<Bank>>>()?;

    Ok(Puzzle { banks })
}

/// Solve puzzle (turns on `num_on_batteries` batteries)
fn solve_puzzle(puzzle: &Puzzle, num_on_batteries: usize) -> usize {
    if num_on_batteries == 0 {
        panic!("Invalid argument: 0 batteries to turn on");
    }

    puzzle
        .banks
        .iter()
        .map(|bank| find_max_batteries(&bank.batteries, num_on_batteries))
        .sum()
}

fn find_max_batteries(batteries: &[usize], num_on_batteries: usize) -> usize {
    let num_batteries = batteries.len();

    if num_batteries < num_on_batteries {
        panic!(
            "`num_on_batteries = {}`, but only have {} batteries: {:?}",
            num_on_batteries, num_batteries, batteries
        );
    }

    let mut to_remove = num_batteries - num_on_batteries;

    let mut res: Vec<usize> = vec![];

    // Greedily find largest numbers by removing all previous numbers with lower value
    // Adds between [num_batteries-to_remove = num_on_batteries, num_batteries] numbers
    for &value in batteries {
        // If we find a number greater than last seen value, remove stored values until either
        // - there is nothing to remove
        // - the last seen value is greater
        // This makes sense because the first numbers in `res` have the most importance, if they can have a greater value, they must.
        while to_remove > 0 && !res.is_empty() && *res.last().unwrap() < value {
            // We remove a number here, so increase count
            res.pop();
            to_remove -= 1;
        }

        // Add value to res
        res.push(value);
    }

    // Trim what is taken in excess (happens if nothing was removed for example - list sorted in descending order)
    res.truncate(num_on_batteries);

    res.iter().fold(0, |acc, value| acc * 10 + value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_puzzle() {
        let input = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
        let puzzle = parse_puzzle(input).unwrap();
        let solution_2 = solve_puzzle(&puzzle, 2);
        assert_eq!(solution_2, 357);
        let solution_12 = solve_puzzle(&puzzle, 12);
        assert_eq!(solution_12, 3121910778619);
    }

    #[test]
    fn test_solve_puzzle_1() {
        let input = "987654321111111";
        let puzzle = parse_puzzle(input).unwrap();
        let solution_2 = solve_puzzle(&puzzle, 2);
        assert_eq!(solution_2, 98);
        let solution_12 = solve_puzzle(&puzzle, 12);
        assert_eq!(solution_12, 987654321111);
    }

    #[test]
    fn test_solve_puzzle_2() {
        let input = "811111111111119";
        let puzzle = parse_puzzle(input).unwrap();
        let solution_2 = solve_puzzle(&puzzle, 2);
        assert_eq!(solution_2, 89);
        let solution_12 = solve_puzzle(&puzzle, 12);
        assert_eq!(solution_12, 811111111119);
    }

    #[test]
    fn test_solve_puzzle_3() {
        let input = "234234234234278";
        let puzzle = parse_puzzle(input).unwrap();
        let solution_2 = solve_puzzle(&puzzle, 2);
        assert_eq!(solution_2, 78);
        let solution_12 = solve_puzzle(&puzzle, 12);
        assert_eq!(solution_12, 434234234278);
    }

    #[test]
    fn test_solve_puzzle_4() {
        let input = "818181911112111";
        let puzzle = parse_puzzle(input).unwrap();
        let solution_2 = solve_puzzle(&puzzle, 2);
        assert_eq!(solution_2, 92);
        let solution_12 = solve_puzzle(&puzzle, 12);
        assert_eq!(solution_12, 888911112111);
    }

    #[test]
    fn test_solve_puzzle_5() {
        let input = "24352342";
        let puzzle = parse_puzzle(input).unwrap();
        let solution_4 = solve_puzzle(&puzzle, 4);
        assert_eq!(solution_4, 5342);
    }

    #[test]
    fn test_solve_puzzle_6() {
        let input = "987654321";
        let puzzle = parse_puzzle(input).unwrap();
        let solution_4 = solve_puzzle(&puzzle, 4);
        assert_eq!(solution_4, 9876);
    }

    #[test]
    fn test_parse_empty_puzzle() {
        let input = "";
        let result = parse_puzzle(input).unwrap();
        assert_eq!(result, Puzzle { banks: vec![] });
    }

    #[test]
    fn test_real_input() {
        let input = include_str!("../../inputs/day-3");
        let puzzle = parse_puzzle(input).unwrap();
        let solution_2 = solve_puzzle(&puzzle, 2);
        assert_eq!(solution_2, 17109);
        let solution_12 = solve_puzzle(&puzzle, 12);
        assert_eq!(solution_12, 169347417057382);
    }
}
