use crate::common::Solution;
use anyhow::{Result, anyhow};

#[derive(Debug)]
pub struct Puzzle {
    problems: Vec<Problem>,
}

/// Operators (addition and multiplication) are commutative, ignore the right to left columns and problems for task 2.
/// If we still want to do it, only need to reverse the problems and their columns for task 2.
#[derive(Debug)]
struct Problem {
    rows: Vec<usize>,
    columns: Vec<usize>,
    operator: Operator,
}

impl Problem {
    fn new(columns_numbers: &[Vec<Number>], operator: Operator) -> Result<Problem> {
        let columns: Vec<usize> = Self::vec_numbers_to_usize(columns_numbers)?;
        let rows: Vec<usize> = Self::vec_numbers_to_usize(&transpose(columns_numbers)?)?;
        Ok(Problem {
            rows,
            columns,
            operator,
        })
    }

    fn vec_numbers_to_usize(numbers: &[Vec<Number>]) -> Result<Vec<usize>> {
        numbers
            .iter()
            .map(|row| Self::numbers_to_usize(row))
            .collect()
    }

    fn numbers_to_usize(numbers: &[Number]) -> Result<usize> {
        if numbers.iter().all(|n| n == &Number::Empty) {
            return Err(anyhow!(
                "All numbers are Empty - resulting integer is undefined."
            ));
        }

        let mut number: usize = 0;

        for n in numbers {
            if let Number::Digit(d) = n {
                number *= 10;
                number += *d as usize;
            }
        }

        Ok(number)
    }
}

#[derive(Clone, PartialEq)]
enum Number {
    Empty,
    Digit(u32),
}

impl TryFrom<char> for Number {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self> {
        value
            .to_digit(10)
            .map(Number::Digit)
            .or({
                if value == ' ' {
                    Some(Number::Empty)
                } else {
                    None
                }
            })
            .ok_or(anyhow!(
                "Invalid char (= '{}') for number. Expected digit or whitespace",
                value
            ))
    }
}

#[derive(Clone, Debug)]
enum Operator {
    Add,
    Multiply,
}

impl TryFrom<char> for Operator {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self> {
        if value == '+' {
            Ok(Operator::Add)
        } else if value == '*' {
            Ok(Operator::Multiply)
        } else {
            Err(anyhow!(
                "Invalid operator character '{}'. Expected '+' or '*'.",
                value
            ))
        }
    }
}

pub fn parse_puzzle(input: &str) -> Result<Puzzle> {
    // Read the whole input into memory to read column by column
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    if lines.is_empty() {
        return Err(anyhow!("Input is empty"));
    }

    let num_lines = lines.len();

    let line_length = lines[0].len();

    if lines.iter().any(|line| line.len() != line_length) {
        return Err(anyhow!("Not all lines have the same length"));
    }

    let mut problems: Vec<Problem> = Vec::new();

    let mut curr_operator: Option<Operator> = None;
    let mut curr_columns: Vec<Vec<Number>> = Vec::new();

    for column_index in 0..line_length {
        let column: Vec<char> = lines.iter().map(|line| line[column_index]).collect();

        let numbers: Vec<Number> = column[0..num_lines - 1]
            .iter()
            .map(|c| (*c).try_into())
            .collect::<Result<Vec<Number>>>()?;

        let operator_char = column[num_lines - 1];
        let operator: Option<Operator> = if operator_char == ' ' {
            None
        } else {
            Some(operator_char.try_into()?)
        };

        if let Some(operator) = operator {
            // New operator, we are reading a new problem

            // If we have a current problem defined, push it
            if let Some(curr_operator) = curr_operator
                && !curr_columns.is_empty()
            {
                let problem = Problem::new(&curr_columns, curr_operator)?;
                problems.push(problem);
            }

            // Update current operator and clear stored numbers
            curr_operator = Some(operator);
            curr_columns.clear();
        }

        // If we have a non-empty column, store it
        if numbers.iter().any(|n| n != &Number::Empty) {
            curr_columns.push(numbers);
        }
    }

    // Push last problem
    if let Some(curr_operator) = curr_operator
        && !curr_columns.is_empty()
    {
        let problem = Problem::new(&curr_columns, curr_operator)?;
        problems.push(problem);
    }

    Ok(Puzzle { problems })
}

pub fn solve_puzzle(puzzle: Puzzle) -> Solution {
    let mut task_1 = 0;
    let mut task_2 = 0;

    for problem in &puzzle.problems {
        match problem.operator {
            Operator::Add => {
                task_1 += problem.rows.iter().sum::<usize>();
                task_2 += problem.columns.iter().sum::<usize>();
            }
            Operator::Multiply => {
                task_1 += problem.rows.iter().product::<usize>();
                task_2 += problem.columns.iter().product::<usize>();
            }
        };
    }

    Solution { task_1, task_2 }
}

/// Transpose matrix NxM to matrix MxN
fn transpose<T: Clone>(matrix: &[Vec<T>]) -> Result<Vec<Vec<T>>> {
    if matrix.is_empty() {
        return Err(anyhow!("Empty transpose"));
    }

    let n = matrix.len();
    let m = matrix[0].len();

    if matrix.iter().any(|row| row.len() != m) {
        return Err(anyhow!(
            "Matrix can't be transposed as not all rows have same length"
        ));
    }

    let mut res: Vec<Vec<T>> = Vec::with_capacity(m);

    for i in 0..m {
        res.push(Vec::with_capacity(n));
        for v in matrix.iter() {
            res[i].push(v[i].clone());
        }
    }

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_puzzle() {
        let input = include_str!("../inputs/examples/day_6");
        let puzzle = parse_puzzle(input).unwrap();
        dbg!(&puzzle);
        let solution = solve_puzzle(puzzle);
        assert_eq!(solution.task_1, 4277556);
        assert_eq!(solution.task_2, 3263827);
    }

    #[test]
    fn test_real_input() {
        let input = include_str!("../inputs/day_6");
        let puzzle = parse_puzzle(input).unwrap();
        let solution = solve_puzzle(puzzle);
        assert_eq!(solution.task_1, 4693419406682);
        assert_eq!(solution.task_2, 9029931401920);
    }
}
