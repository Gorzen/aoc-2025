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

fn solve_puzzle(puzzle: &Puzzle) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_puzzle() {
        let input = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
        let puzzle = parse_puzzle(input).unwrap();
        let solution = solve_puzzle(&puzzle);
        assert_eq!(solution, 1227775554);
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
        let solution = solve_puzzle(&puzzle);
        assert_eq!(solution, 1145);
    }
}
