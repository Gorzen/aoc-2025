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

fn solve_puzzle(puzzle: &Puzzle) -> usize {
    // The first battery takes precedence, it needs to be the largest digit in the bank (excluding the last digit, as the second battery needs to have a digit available)
    puzzle
        .banks
        .iter()
        .map(|bank| {
            let first_battery = find_first_battery(&bank.batteries);
            let second_battery = find_second_battery(&bank.batteries, first_battery.0);
            first_battery.1 * 10 + second_battery.1
        })
        .sum()
}

/// Returns (index, value) of first battery, which is the first max value - excluding the last index
fn find_first_battery(batteries: &[usize]) -> (usize, usize) {
    if batteries.len() < 2 {
        panic!(
            "Batteries should always have at least length 2, found {:?}",
            batteries
        );
    }

    let mut max = None;

    // Always runs at least once because batteries.len() >= 2
    for (i, value) in batteries.iter().enumerate().take(batteries.len() - 1) {
        match max {
            None => max = Some((i, batteries[i])),
            Some((_, current_max)) => {
                if *value > current_max {
                    max = Some((i, *value))
                }
            }
        }
    }

    // We can safely unwrap, thanks to the condition and comment above.
    max.unwrap()
}

/// Returns (index, value) of second battery, which is the max value after the index of the first_battery
fn find_second_battery(batteries: &[usize], first_battery_index: usize) -> (usize, usize) {
    if first_battery_index >= batteries.len() - 1 {
        panic!(
            "first_battery_index should always leave room for second batter, found {}, for {:?}",
            first_battery_index, batteries
        );
    }

    let mut max = None;

    // Always runs at least once because first_battery_index < batteries.len() - 1
    for (i, value) in batteries.iter().enumerate().skip(first_battery_index + 1) {
        match max {
            None => max = Some((i, batteries[i])),
            Some((_, current_max)) => {
                if *value > current_max {
                    max = Some((i, *value))
                }
            }
        }
    }

    // We can safely unwrap, thanks to the condition and comment above.
    max.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_puzzle() {
        let input = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
        let puzzle = parse_puzzle(input).unwrap();
        let solution = solve_puzzle(&puzzle);
        assert_eq!(solution, 357);
    }

    #[test]
    fn test_finding_batteries_1() {
        let batteries = vec![3, 2, 3, 3, 9, 1, 1];
        let first_battery = find_first_battery(&batteries);
        assert_eq!(first_battery, (4, 9));
        let second_battery = find_second_battery(&batteries, first_battery.0);
        assert_eq!(second_battery, (5, 1));
    }

    #[test]
    fn test_finding_batteries_2() {
        let batteries = vec![
            3, 2, 3, 3, 4, 3, 4, 2, 2, 3, 3, 5, 2, 2, 5, 3, 3, 2, 2, 2, 4, 4, 3, 2, 3, 5, 6, 2, 4,
            1, 3, 2, 2, 2, 3, 2, 2, 4, 2, 2, 5, 2, 2, 6, 2, 2, 3, 1, 2, 4, 2, 2, 3, 3, 2, 1, 2, 3,
            2, 2, 3, 1, 2, 3, 2, 3, 5, 4, 2, 2, 2, 1, 2, 1, 9, 6, 3, 2, 3, 2, 2, 2, 3, 3, 2, 2, 3,
            2, 3, 3, 2, 2, 4, 2, 2, 2, 2, 2, 1, 1,
        ];
        let first_battery = find_first_battery(&batteries);
        assert_eq!(first_battery, (74, 9));
        let second_battery = find_second_battery(&batteries, first_battery.0);
        assert_eq!(second_battery, (75, 6));
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
        assert_eq!(solution, 17109);
    }
}
