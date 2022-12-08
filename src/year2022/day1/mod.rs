use crate::{helpers::input::ProblemInput, problem::Problem};

use anyhow::Result;

pub struct Day1 {}

impl Default for Day1 {
    fn default() -> Self {
        Self {}
    }
}

impl Problem<usize, usize> for Day1 {
    fn part1(&self, input: &ProblemInput) -> Result<usize> {
        let energy = input.parse_sections(|x| x.parse::<usize>().map_err(|e| e.into()))?;

        Ok(energy.iter().map(|v| v.iter().sum()).max().unwrap())
    }

    fn part2(&self, input: &ProblemInput) -> Result<usize> {
        let energy = input.parse_sections(|x| x.parse::<usize>().map_err(|e| e.into()))?;

        let mut cals = energy
            .iter()
            .map(|v| v.iter().sum())
            .collect::<Vec<usize>>();
        cals.sort();
        let sum = cals.pop().unwrap() + cals.pop().unwrap() + cals.pop().unwrap();
        Ok(sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> ProblemInput {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        crate::helpers::input::ProblemInput::from_sample(input)
    }

    #[test]
    fn sample1() {
        assert_eq!(24000, Day1::default().part1(&sample()).unwrap())
    }

    #[test]
    fn sample2() {
        assert_eq!(45000, Day1::default().part2(&sample()).unwrap())
    }
}
