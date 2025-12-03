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

    let solution = solve_puzzle(&puzzle);
    println!("Solution: {}", solution);
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

fn solve_puzzle(puzzle: &Puzzle) -> usize {
    // Collect all invalid IDs
    let invalid_ids = puzzle.ranges.iter().flat_map(|range| {
        (range.start..=range.end).flat_map(|id| if !is_id_valid(id) { Some(id) } else { None })
    });

    invalid_ids.sum()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_puzzle() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let puzzle = parse_puzzle(input).unwrap();
        let solution = solve_puzzle(&puzzle);
        assert_eq!(solution, 1227775554);
    }

    #[test]
    fn test_empty_puzzle() {
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
        let solution = solve_puzzle(&puzzle);
        assert_eq!(solution, 26255179562);
    }
}
