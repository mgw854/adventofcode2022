use std::{ops::RangeInclusive, str::FromStr};

use crate::{helpers::input::ProblemInput, problem::Problem};

use anyhow::{anyhow, Result};
use num::iter::Range;

pub struct Day4 {}

impl Default for Day4 {
    fn default() -> Self {
        Self {}
    }
}

impl Problem<usize, usize> for Day4 {
    fn part1(&self, input: &ProblemInput) -> Result<usize> {
        let pairs = input.parse_lines(|x| x.parse::<AssignmentPair>().map_err(|e| anyhow!("Missing attribute: {}", "")))?;

        let count = pairs.iter().filter(|p| p.is_one_strict_subset()).count();

        Ok(count)
    }

    fn part2(&self, input: &ProblemInput) -> Result<usize> {
        let pairs = input.parse_lines(|x| x.parse::<AssignmentPair>().map_err(|e| anyhow!("Missing attribute: {}", "")))?;

        let count = pairs.iter().filter(|p| p.is_overlap()).count();

        Ok(count)
    }
}

pub struct AssignmentPair {
    pub firstElf: RangeInclusive<usize>,
    pub secondElf: RangeInclusive<usize>
}

impl AssignmentPair {
    fn is_one_strict_subset(&self) -> bool {
        (self.firstElf.start() <= self.secondElf.start() && self.firstElf.end() >= self.secondElf.end()) ||
        (self.secondElf.start() <= self.firstElf.start() && self.secondElf.end() >= self.firstElf.end())
    }
    
    fn is_overlap(&self) -> bool {
        (self.firstElf.start() <= self.secondElf.start() && self.firstElf.end() >= self.secondElf.start()) ||
        (self.secondElf.start() <= self.firstElf.start() && self.secondElf.end() >= self.firstElf.start())
    }
}

impl FromStr for AssignmentPair {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let matches: Vec<Vec<usize>> = s.split(',').map(|m| m.split('-').map(|r| r.parse::<usize>().unwrap()).collect::<Vec<usize>>()).collect();

        Ok(AssignmentPair {
            firstElf: RangeInclusive::new(matches[0][0], matches[0][1]),
            secondElf: RangeInclusive::new(matches[1][0], matches[1][1])
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> ProblemInput {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        crate::helpers::input::ProblemInput::from_sample(input)
    }

    #[test]
    fn sample1() {
        assert_eq!(2, Day4::default().part1(&sample()).unwrap())
    }

    
    #[test]
    fn sample2() {
        assert_eq!(4, Day4::default().part2(&sample()).unwrap())
    }
}