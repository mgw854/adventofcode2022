use crate::helpers::input::ProblemInput;

use anyhow::Result;

pub trait Problem<T1, T2> {
    fn part1(&self, input: &ProblemInput) -> Result<T1>;
    fn part2(&self, input: &ProblemInput) -> Result<T2>;
}