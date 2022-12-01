use crate::{problem::Problem, helpers::input::ProblemInput};

use anyhow::Result;

pub struct Day1 { }

impl Default for Day1 {
    fn default() -> Self {
        Self {  }
    }
}

impl Problem<usize, usize> for Day1 {
    fn part1(&self, input: &ProblemInput) -> Result<usize> {
        let energy = input.parse_sections(|x| x.parse::<usize>().map_err(|e| e.into()))?;

        Ok(energy.iter().map(|v| v.iter().sum()).max().unwrap())
    }

    fn part2(&self, input: &ProblemInput) -> Result<usize> {
        let energy = input.parse_sections(|x| x.parse::<usize>().map_err(|e| e.into()))?;

        let mut cals = energy.iter().map(|v| v.iter().sum()).collect::<Vec<usize>>();
        cals.sort();
        let sum = cals.pop().unwrap() + cals.pop().unwrap() + cals.pop().unwrap();
        Ok(sum)
    }
}