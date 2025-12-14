use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub fn read_input(day: &str) -> Result<String> {
    let path_str = format!("inputs/{}", day);
    let path = Path::new(&path_str);
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read file {}", path.display()))?;
    Ok(content)
}

pub struct Solution {
    pub task_1: usize,
    pub task_2: usize,
}
