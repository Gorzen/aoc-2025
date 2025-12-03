use anyhow::{Context, Result, anyhow};
use std::{
    fmt::Display,
    io::{self, Read},
};

const START_POSITION: usize = 50;

const MAX_POSITION: usize = 100;

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

    let (times_finish_at_zero, times_pass_zero) = solve_puzzle(&puzzle);
    println!(
        "Solution:\n- times_finish_at_zero: {}\n- times_pass_zero {}",
        times_finish_at_zero, times_pass_zero
    );
}

struct Puzzle {
    instructions: Vec<Instruction>,
}

enum Instruction {
    Left(usize),
    Right(usize),
}

impl Instruction {
    fn steps(&self) -> usize {
        match self {
            Instruction::Left(steps) => *steps,
            Instruction::Right(steps) => *steps,
        }
    }

    fn delta(&self) -> i64 {
        match self {
            Instruction::Left(steps) => -(*steps as i64),
            Instruction::Right(steps) => *steps as i64,
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Left(steps) => write!(f, "L{}", steps),
            Instruction::Right(steps) => write!(f, "R{}", steps),
        }
    }
}

/// Consumes the input string and produces a Puzzle representation.
fn parse_puzzle(input: &str) -> Result<Puzzle> {
    let instructions: Vec<Instruction> = input
        .lines()
        .map(|line| {
            if line.is_empty() {
                return Err(anyhow!("Empty line found"));
            }

            let (dir, value) = line.split_at(1);
            let value: usize = value
                .parse()
                .with_context(|| format!("Failed to parse number in line: '{}'", line))?;

            match dir {
                "L" => Ok(Instruction::Left(value)),
                "R" => Ok(Instruction::Right(value)),
                _ => Err(anyhow::anyhow!(
                    "Invalid direction. Expected 'L' or 'R', found '{}'",
                    dir
                )),
            }
        })
        // Use collect to gather results and propagate errors if any
        .collect::<Result<Vec<Instruction>>>()?;

    Ok(Puzzle { instructions })
}

fn solve_puzzle(puzzle: &Puzzle) -> (usize, usize) {
    let mut times_finish_at_zero: usize = 0;
    let mut times_pass_zero: usize = 0;

    let mut position = START_POSITION as i64;

    for instruction in &puzzle.instructions {
        let old_position = position;
        let new_position = (old_position + instruction.delta()).rem_euclid(MAX_POSITION as i64);
        position = new_position;

        // Number of time we go full circle
        times_pass_zero += instruction.steps() / MAX_POSITION;

        // Check if we passed zero in this move (not counting full circles)
        match instruction {
            Instruction::Left(_) => {
                if (new_position > old_position || new_position == 0) && old_position != 0 {
                    times_pass_zero += 1;
                }
            }
            Instruction::Right(_) => {
                if new_position < old_position {
                    times_pass_zero += 1;
                }
            }
        }

        // Check if we finish at zero
        if new_position == 0 {
            times_finish_at_zero += 1;
        }
    }

    (times_finish_at_zero, times_pass_zero)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_puzzle() {
        let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
        let puzzle = parse_puzzle(input).unwrap();
        let (times_finish_at_zero, times_pass_zero) = solve_puzzle(&puzzle);
        assert_eq!(times_finish_at_zero, 3);
        assert_eq!(times_pass_zero, 6);
    }

    #[test]
    fn test_empty_puzzle() {
        let input = "";
        let puzzle = parse_puzzle(input).unwrap();
        let (times_finish_at_zero, times_pass_zero) = solve_puzzle(&puzzle);
        assert_eq!(times_finish_at_zero, 0);
        assert_eq!(times_pass_zero, 0);
    }

    #[test]
    fn test_pass_zero_right() {
        let input = "R1000";
        let puzzle = parse_puzzle(input).unwrap();
        let (times_finish_at_zero, times_pass_zero) = solve_puzzle(&puzzle);
        assert_eq!(times_finish_at_zero, 0);
        assert_eq!(times_pass_zero, 10);
    }

    #[test]
    fn test_pass_zero_left() {
        let input = "L1000";
        let puzzle = parse_puzzle(input).unwrap();
        let (_, pass) = solve_puzzle(&puzzle);
        assert_eq!(pass, 10);
    }

    #[test]
    fn test_pass_zero_exactly_left() {
        let input = "L50";
        let puzzle = parse_puzzle(input).unwrap();
        let (finish, pass) = solve_puzzle(&puzzle);
        assert_eq!(finish, 1);
        assert_eq!(pass, 1);
    }

    #[test]
    fn test_pass_zero_exactly_right() {
        let input = "R50";
        let puzzle = parse_puzzle(input).unwrap();
        let (finish, pass) = solve_puzzle(&puzzle);
        assert_eq!(finish, 1);
        assert_eq!(pass, 1);
    }

    #[test]
    fn test_move_left_from_zero() {
        let input = "L50\nL1";
        let puzzle = parse_puzzle(input).unwrap();
        let (finish, pass) = solve_puzzle(&puzzle);
        assert_eq!(finish, 1);
        assert_eq!(pass, 1); // Passed zero only once, the L1 goes from 0 to 99, it does not pass or finish on zero.
    }

    #[test]
    fn test_move_99_left() {
        let input = "L99";
        let puzzle = parse_puzzle(input).unwrap();
        let (finish, pass) = solve_puzzle(&puzzle);
        assert_eq!(finish, 0);
        assert_eq!(pass, 1);
    }

    #[test]
    fn test_move_99_right() {
        let input = "R99";
        let puzzle = parse_puzzle(input).unwrap();
        let (finish, pass) = solve_puzzle(&puzzle);
        assert_eq!(finish, 0);
        assert_eq!(pass, 1);
    }

    #[test]
    fn test_move_101_left() {
        let input = "L101";
        let puzzle = parse_puzzle(input).unwrap();
        let (finish, pass) = solve_puzzle(&puzzle);
        assert_eq!(finish, 0);
        assert_eq!(pass, 1);
    }

    #[test]
    fn test_move_101_right() {
        let input = "R101";
        let puzzle = parse_puzzle(input).unwrap();
        let (finish, pass) = solve_puzzle(&puzzle);
        assert_eq!(finish, 0);
        assert_eq!(pass, 1);
    }

    #[test]
    fn test_real_input() {
        let input = include_str!("../../inputs/day-1");
        let puzzle = parse_puzzle(input).unwrap();
        let (times_finish_at_zero, times_pass_zero) = solve_puzzle(&puzzle);
        assert_eq!(times_finish_at_zero, 1145);
        assert_eq!(times_pass_zero, 6561);
    }
}
