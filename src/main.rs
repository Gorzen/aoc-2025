use aoc_2025::common;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!(
            "Expected two arguments - the program and the day to run (day_1, day_2, ...). Got {} arguments.",
            args.len()
        );
        return;
    }

    let day = args[1].as_str();

    match day {
        "day_1" => run_day!(day_1),
        "day_2" => run_day!(day_2),
        "day_3" => run_day!(day_3),
        "day_4" => run_day!(day_4),
        other => {
            eprintln!("Unexpected argument {}. Expected day_1, day_2, ...", other);
        }
    }
}

#[macro_export]
macro_rules! run_day {
    ($day:ident) => {{
        let input = match common::read_input(stringify!($day)) {
            Ok(input) => input,
            Err(e) => {
                eprintln!("Error reading puzzle for {}: {}", stringify!($day), e);
                return;
            }
        };

        // Consume input
        let puzzle = match aoc_2025::$day::parse_puzzle(&input) {
            Ok(puzzle) => puzzle,
            Err(e) => {
                eprintln!("Error parsing puzzle for {}: {}", stringify!($day), e);
                return;
            }
        };

        // Consume puzzle and pass ownership to solver
        let solution = aoc_2025::$day::solve_puzzle(puzzle);

        println!(
            "Solution:\n- Task 1: {}\n- Task 2: {}",
            solution.task_1, solution.task_2,
        );
    }};
}
