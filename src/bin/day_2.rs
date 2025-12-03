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
    ids: Vec<Id>,
}

struct Id(usize);

impl Id {
    fn is_valid(&self) -> bool {
        todo!()
    }
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
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let puzzle = parse_puzzle(input).unwrap();
        let solution = solve_puzzle(&puzzle);
        assert_eq!(solution, 1227775554);
    }

    #[test]
    fn test_real_input() {
        let input = include_str!("../../inputs/day-2");
        let puzzle = parse_puzzle(input).unwrap();
        let solution = solve_puzzle(&puzzle);
        assert_eq!(solution, 1145);
    }
}
