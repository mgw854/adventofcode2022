use crate::{helpers::input::ProblemInput, problem::Problem};

use anyhow::Result;

pub struct Day0 {}

impl Default for Day0 {
    fn default() -> Self {
        Self {}
    }
}

impl Problem<u8, u8> for Day0 {
    fn part1(&self, input: &ProblemInput) -> Result<u8> {
        Ok(1)
    }

    fn part2(&self, input: &ProblemInput) -> Result<u8> {
        Ok(2)
    }
}
