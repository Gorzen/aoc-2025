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
    // There are always at least 2 batteries, by construction
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
                .and_then(|batteries| {
                    if batteries.len() < 2 {
                        return Err(anyhow!("Less than 2 batteries in one bank: '{}'", line));
                    }

                    Ok(Bank { batteries })
                })
        })
        .collect::<Result<Vec<Bank>>>()?;

    Ok(Puzzle { banks })
}

/// Solve puzzle (turns on `num_on_batteries` batteries)
fn solve_puzzle(puzzle: &Puzzle, num_on_batteries: usize) -> usize {
    if num_on_batteries == 0 {
        panic!("Invalid argument: 0 batteries to turn on");
    }

    // The first battery takes precedence, it needs to be the largest digit in the bank (excluding the last digit, as the second battery needs to have a digit available)
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
            num_batteries,
            batteries.len(),
            batteries
        );
    }

    let mut to_remove = num_batteries - num_on_batteries;

    let mut res: Vec<usize> = vec![];

    let mut i = 0;

    // Greedily find largest numbers by removing all previous numbers with lower value
    while to_remove > 0 && i < batteries.len() {
        let value = batteries[i];

        if res.is_empty() {
            // Add value to res
            res.push(value);
            // Move to next value
            i += 1;
        } else {
            // Check if value is better than what was already seen (previous values take precedence)
            if value > *res.last().unwrap() {
                // Value is better -> remove last
                res.pop();
                // We removed one more number
                to_remove -= 1;
            } else {
                // Value is not better, take it for now
                res.push(value);
                i += 1;
            }
        }
    }

    // If we removed all values before reaching the end (i < batteries.len()-1), add the rest
    for value in batteries.iter().skip(i) {
        res.push(*value);
    }

    // Trim what is taken in excess (happens if nothing was removed for example - list sorted in descending order)
    while res.len() > num_on_batteries {
        res.pop();
    }

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
