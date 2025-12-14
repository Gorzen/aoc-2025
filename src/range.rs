use std::{fmt::Display, ops::RangeInclusive};

use anyhow::{Context, Result, anyhow};

/// Inclusive range from start to end
#[derive(Debug, PartialEq)]
pub struct Range {
    start: usize,
    end: usize,
}

impl Range {
    pub fn range(&self) -> RangeInclusive<usize> {
        self.start..=self.end
    }

    /// Returns the number of elements in range
    pub fn count(&self) -> usize {
        self.end - self.start + 1
    }

    pub fn parse(str: &str) -> Result<Self> {
        let x: Vec<&str> = str.split('-').collect();
        if x.len() != 2 {
            return Err(anyhow!(
                "Invalid range format, expected x-y, found '{}'",
                str
            ));
        }
        let start: usize = x[0]
            .parse()
            .with_context(|| format!("Failed to parse range's start: '{}'", x[0]))?;
        let end: usize = x[1]
            .parse()
            .with_context(|| format!("Failed to parse range's end: '{}'", x[1]))?;

        if end < start {
            return Err(anyhow!(
                "Cannot create inclusive range where end < start: '{}'",
                str
            ));
        }

        Ok(Range { start, end })
    }

    pub fn is_included(&self, n: usize) -> bool {
        self.start <= n && n <= self.end
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.start, self.end)
    }
}

pub fn merge_ranges(mut ranges: Vec<Range>) -> Vec<Range> {
    // Soring is needed to simplify the logic of merging. We know the range we see has a start that is bigger or equal to the last seen.
    // Thanks to that, we only need to compare to the last range, not to all seen ranges. A new range cannot create an overlap between 2 previously seen ranges.
    ranges.sort_by(|a, b| a.start.cmp(&b.start));

    let mut merged_ranges: Vec<Range> = Vec::new();

    for range in ranges {
        match merged_ranges.last_mut() {
            Some(last_range) => {
                if range.start <= last_range.end {
                    // Overlap, merge
                    last_range.end = last_range.end.max(range.end);
                } else {
                    merged_ranges.push(range);
                }
            }
            None => merged_ranges.push(range),
        }
    }

    merged_ranges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty_range() {
        let input = "";
        let range = Range::parse(input);
        assert!(range.is_err());
        assert!(
            range
                .unwrap_err()
                .to_string()
                .contains("Invalid range format, expected x-y, found ''")
        );
    }

    #[test]
    fn test_parse_invalid_range() {
        let input = "2-1";
        let range = Range::parse(input);
        assert!(range.is_err());
        assert!(
            range
                .unwrap_err()
                .to_string()
                .contains("Cannot create inclusive range where end < start: '2-1'")
        );
    }

    #[test]
    fn test_parse_valid_singleton_range() {
        let input = "1-1";
        let range = Range::parse(input);
        assert!(range.is_ok());
        assert_eq!(range.unwrap(), Range { start: 1, end: 1 });
    }

    #[test]
    fn test_parse_valid_range() {
        let input = "0-10";
        let range = Range::parse(input);
        assert!(range.is_ok());
        assert_eq!(range.unwrap(), Range { start: 0, end: 10 });
    }

    #[test]
    fn test_merge_ranges() {
        fn new_range(start: usize, end: usize) -> Range {
            Range { start, end }
        }

        let ranges = vec![
            new_range(3, 5),
            new_range(10, 14),
            new_range(16, 20),
            new_range(12, 18),
        ];
        let merged_ranged = merge_ranges(ranges);
        assert_eq!(merged_ranged, vec![new_range(3, 5), new_range(10, 20)])
    }
}
