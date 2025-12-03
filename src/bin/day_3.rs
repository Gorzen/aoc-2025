use std::io::{self, Read};
use anyhow::{Result, Context, anyhow}

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

struct Puzzle {
}

fn parse_puzzle(input: &str) -> Result<Puzzle> {
    todo!()
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
    fn test_real_input() {
        let input = include_str!("../../inputs/day-3");
        let puzzle = parse_puzzle(input).unwrap();
        let solution = solve_puzzle(&puzzle);
        assert_eq!(solution, 1145);
    }
}
