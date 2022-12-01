use crate::{helpers::input::ProblemInput, problem::Problem};

use anyhow::Result;

pub struct Day2 {}

impl Default for Day2 {
    fn default() -> Self {
        Self {}
    }
}

impl Problem<usize, usize> for Day2 {
    fn part1(&self, input: &ProblemInput) -> Result<usize> {
        todo!()
    }

    fn part2(&self, input: &ProblemInput) -> Result<usize> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> ProblemInput {
        let input = "";

        crate::helpers::input::ProblemInput::from_sample(input)
    }

    #[test]
    fn sample1() {
        assert_eq!(0, Day2::default().part1(&sample()).unwrap())
    }

    #[test]
    fn sample2() {
        assert_eq!(0, Day2::default().part2(&sample()).unwrap())
    }
}
