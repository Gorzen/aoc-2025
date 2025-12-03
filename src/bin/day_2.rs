use anyhow::{Context, Result, anyhow};
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

    let solution_1 = task_1::solve_puzzle(&puzzle);
    let solution_2 = task_2::solve_puzzle(&puzzle);
    println!(
        "Solution task 1: {}\nSolution task 2: {}",
        solution_1, solution_2
    );
}

#[derive(Debug)]
struct Puzzle {
    ranges: Vec<Range>,
}

/// Inclusive range from start to end
#[derive(Debug)]
struct Range {
    start: usize,
    end: usize,
}

fn parse_puzzle(input: &str) -> Result<Puzzle> {
    let ranges = input
        .split(',')
        .map(|range| {
            if range.is_empty() {
                return Err(anyhow!("Empty range found"));
            }

            let x: Vec<&str> = range.split('-').collect();
            if x.len() != 2 {
                return Err(anyhow!(
                    "Invalid range format, expected x-y, found '{}'",
                    range
                ));
            }
            let start: usize = x[0]
                .parse()
                .with_context(|| format!("Failed to parse number: '{}'", x[0]))?;
            let end: usize = x[1]
                .parse()
                .with_context(|| format!("Failed to parse number: '{}'", x[1]))?;
            Ok(Range { start, end })
        })
        .collect::<Result<Vec<Range>>>()?;

    Ok(Puzzle { ranges })
}

/// Get digits of a number as a vector
fn get_digits(mut id: usize) -> Vec<u8> {
    if id == 0 {
        return vec![0];
    }

    let mut digits = Vec::new();
    while id > 0 {
        digits.insert(0, (id % 10) as u8);
        id /= 10;
    }
    digits
}

fn solve_puzzle_generic(puzzle: &Puzzle, is_id_valid: fn(usize) -> bool) -> usize {
    // Collect all invalid IDs
    let invalid_ids = puzzle.ranges.iter().flat_map(|range| {
        (range.start..=range.end).flat_map(|id| if is_id_valid(id) { None } else { Some(id) })
    });

    invalid_ids.sum()
}

/// Module for solving task 1 (to keep things organized)
mod task_1 {
    use super::*;

    pub fn solve_puzzle(puzzle: &Puzzle) -> usize {
        solve_puzzle_generic(puzzle, is_id_valid)
    }

    fn is_id_valid(id: usize) -> bool {
        let digits: Vec<u8> = get_digits(id);

        if !digits.len().is_multiple_of(2) {
            return true;
        }

        let first_half = &digits[..digits.len() / 2];
        let second_half = &digits[digits.len() / 2..];

        first_half != second_half
    }
}

/// Module for solving task 2 (to keep things organized)
mod task_2 {
    use super::*;

    pub fn solve_puzzle(puzzle: &Puzzle) -> usize {
        solve_puzzle_generic(puzzle, is_id_valid)
    }

    fn is_id_valid(id: usize) -> bool {
        let digits: Vec<u8> = get_digits(id);

        for split in 1..digits.len() {
            let mut chunks = digits.chunks(split);

            let num_chunks = chunks.len();

            let first_chunk = chunks.next().unwrap();

            // Check if the rest of the chunks are equal to the first
            if num_chunks > 1 && chunks.all(|chunk| chunk == first_chunk) {
                // Match found -> invalid ID
                return false;
            }
        }

        // Did not find a matching split -> valid ID
        true
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_is_id_valid() {
            assert_eq!(is_id_valid(11), false);
            assert_eq!(is_id_valid(112), true);
            assert_eq!(is_id_valid(1212), false);
            assert_eq!(is_id_valid(12123), true);
            assert_eq!(is_id_valid(123123), false);
            assert_eq!(is_id_valid(12341234), false);
            assert_eq!(is_id_valid(123123123), false);
            assert_eq!(is_id_valid(1212121212), false);
            assert_eq!(is_id_valid(1111111), false);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_puzzle() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let puzzle = parse_puzzle(input).unwrap();
        let solution_1 = task_1::solve_puzzle(&puzzle);
        let solution_2 = task_2::solve_puzzle(&puzzle);
        assert_eq!(solution_1, 1227775554);
        assert_eq!(solution_2, 4174379265);
    }

    #[test]
    fn test_parse_empty_puzzle() {
        let input = "";
        let result = parse_puzzle(input);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Empty range found")
        );
    }

    #[test]
    fn test_real_input() {
        let input = include_str!("../../inputs/day-2");
        let puzzle = parse_puzzle(input).unwrap();
        let solution_1 = task_1::solve_puzzle(&puzzle);
        let solution_2 = task_2::solve_puzzle(&puzzle);
        assert_eq!(solution_1, 26255179562);
        assert_eq!(solution_2, 31680313976);
    }
}
