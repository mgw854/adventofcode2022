use crate::{helpers::input::ProblemInput, problem::Problem};

use anyhow::Result;
use itertools::Itertools;

pub struct Day6 {}

impl Default for Day6 {
    fn default() -> Self {
        Self {}
    }
}

impl Problem<usize, usize> for Day6 {
    fn part1(&self, input: &ProblemInput) -> Result<usize> {
        for (i, w) in input.get_chars().windows(4).enumerate() {
            if w.iter().unique().count() == 4 {
                return Ok(i+4);
            }
        }

        Ok(0)
    }

    fn part2(&self, input: &ProblemInput) -> Result<usize> {
        for (i, w) in input.get_chars().windows(14).enumerate() {
            if w.iter().unique().count() == 14 {
                return Ok(i+14);
            }
        }

        Ok(0)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> ProblemInput {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

        crate::helpers::input::ProblemInput::from_sample(input)
    }

    #[test]
    fn sample1() {
        assert_eq!(7, Day6::default().part1(&sample()).unwrap())
    }

    
    #[test]
    fn sample2() {
        assert_eq!(19, Day6::default().part2(&sample()).unwrap())
    }
}