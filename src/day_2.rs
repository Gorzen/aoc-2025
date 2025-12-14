use anyhow::Result;

// TODO: I wonder if I should make all days pass with empty inputs...

use crate::{
    common::Solution,
    range::{Range, merge_ranges},
};

#[derive(Debug)]
pub struct Puzzle {
    ranges: Vec<Range>,
}

pub fn parse_puzzle(input: &str) -> Result<Puzzle> {
    let ranges = input
        .split(',')
        .map(Range::parse)
        .collect::<Result<Vec<Range>>>()?;

    Ok(Puzzle { ranges })
}

pub fn solve_puzzle(puzzle: Puzzle) -> Solution {
    // Allocate memory once
    // Capacity 20 is enough for any u64 (max ~1.8e19)
    let mut buffer: Vec<u8> = Vec::with_capacity(20);

    let mut invalid_sum_1 = 0;
    let mut invalid_sum_2 = 0;

    let merged_ranges = merge_ranges(puzzle.ranges);

    for range in &merged_ranges {
        for id in range.range() {
            get_digits_into(id, &mut buffer);
            if !is_id_valid_1(&buffer) {
                invalid_sum_1 += id;
            }
            if !is_id_valid_2(&buffer) {
                invalid_sum_2 += id;
            }
        }
    }

    Solution {
        task_1: invalid_sum_1,
        task_2: invalid_sum_2,
    }
}

/// Get digits of a number as a vector
fn get_digits_into(mut id: usize, buffer: &mut Vec<u8>) {
    // Clear content, but keep allocated memory
    buffer.clear();

    if id == 0 {
        buffer.push(0);
    }

    while id > 0 {
        buffer.push((id % 10) as u8);
        id /= 10;
    }

    // Reverse the digits so they are in order
    // This is faster than always pre-pending because inserting at 0 has complexity O(n) because it moves elements
    buffer.reverse();
}

/// Implementation for Part one
fn is_id_valid_1(digits: &[u8]) -> bool {
    if !digits.len().is_multiple_of(2) {
        return true;
    }

    let first_half = &digits[..digits.len() / 2];
    let second_half = &digits[digits.len() / 2..];

    first_half != second_half
}

/// Implementation for Part two
fn is_id_valid_2(digits: &[u8]) -> bool {
    let n = digits.len();

    for split in 1..=(n / 2) {
        if !n.is_multiple_of(split) {
            // Do not bother checking - it will not split in chunks of equal sizes -> early exit
            continue;
        }

        let mut chunks = digits.chunks(split);

        let num_chunks = chunks.len();

        let first_chunk = chunks.next().unwrap();

        // Check if the rest of the chunks are equal to the first
        if num_chunks > 1 && chunks.all(|chunk| chunk == first_chunk) {
            // Match found -> invalid ID
            return false;
        }
    }

    // Did not find a matching split -> valid ID
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_puzzle() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let puzzle = parse_puzzle(input).unwrap();
        let solution = solve_puzzle(puzzle);
        assert_eq!(solution.task_1, 1227775554);
        assert_eq!(solution.task_2, 4174379265);
    }

    #[test]
    fn test_is_id_valid_2() {
        assert_eq!(is_id_valid_2(&[1, 1]), false);
        assert_eq!(is_id_valid_2(&[1, 1, 2]), true);
        assert_eq!(is_id_valid_2(&[1, 2, 1, 2]), false);
        assert_eq!(is_id_valid_2(&[1, 2, 1, 2, 3]), true);
        assert_eq!(is_id_valid_2(&[1, 2, 3, 1, 2, 3]), false);
        assert_eq!(is_id_valid_2(&[1, 2, 3, 4, 1, 2, 3, 4]), false);
        assert_eq!(is_id_valid_2(&[1, 2, 3, 1, 2, 3, 1, 2, 3]), false);
        assert_eq!(is_id_valid_2(&[1, 2, 1, 2, 1, 2, 1, 2, 1, 2]), false);
        assert_eq!(is_id_valid_2(&[1, 1, 1, 1, 1, 1, 1]), false);
    }

    #[test]
    fn test_real_input() {
        let input = include_str!("../inputs/day_2");
        let puzzle = parse_puzzle(input).unwrap();
        let solution = solve_puzzle(puzzle);
        assert_eq!(solution.task_1, 26255179562);
        assert_eq!(solution.task_2, 31680313976);
    }
}
