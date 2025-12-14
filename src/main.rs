use anyhow::{Result, anyhow};
use aoc_2025::common;
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(anyhow!(
            "Expected two arguments - the program and the day to run (day_1, ..., day_12). Got {} arguments.",
            args.len()
        ));
    }

    let day = args[1].as_str();

    match day {
        "day_1" => run_day!(day_1),
        "day_2" => run_day!(day_2),
        "day_3" => run_day!(day_3),
        "day_4" => run_day!(day_4),
        "day_5" => run_day!(day_5),
        other => Err(anyhow!(
            "Unexpected argument {}. Expected day_1, ..., day_12",
            other
        )),
    }
}

#[macro_export]
macro_rules! run_day {
    ($day:ident) => {{
        let input = match common::read_input(stringify!($day)) {
            Ok(input) => input,
            Err(e) => {
                return Err(anyhow!(
                    "Error reading puzzle for {}: {}",
                    stringify!($day),
                    e
                ));
            }
        };

        // Consume input
        let puzzle = match aoc_2025::$day::parse_puzzle(&input) {
            Ok(puzzle) => puzzle,
            Err(e) => {
                return Err(anyhow!(
                    "Error parsing puzzle for {}: {}",
                    stringify!($day),
                    e
                ));
            }
        };

        // Consume puzzle and pass ownership to solver
        let solution = aoc_2025::$day::solve_puzzle(puzzle);

        println!(
            "Solution:\n- Task 1: {}\n- Task 2: {}",
            solution.task_1, solution.task_2,
        );

        Ok(())
    }};
}
