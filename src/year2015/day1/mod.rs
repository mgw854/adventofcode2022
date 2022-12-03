use crate::{helpers::input::ProblemInput, problem::Problem};

use anyhow::{anyhow, Result};

pub struct Day1 {}

impl Default for Day1 {
    fn default() -> Self {
        Self {}
    }
}

impl Problem<usize, usize> for Day1 {
    fn part1(&self, input: &ProblemInput) -> Result<usize> {
        let chars = input.get_chars();
        let floors_up = chars.iter().filter(|&x| x == &'(').count();
        let floors_down = chars.iter().filter(|&x| x == &')').count();
        Ok(floors_up - floors_down)
    }

    fn part2(&self, input: &ProblemInput) -> Result<usize> {
        let chars = input.get_chars();

        let mut pos = 0;

        for (i, c) in chars.iter().enumerate() {
            pos = pos
                + match c {
                    '(' => 1,
                    ')' => -1,
                    _ => 0,
                };

            if (pos == -1) {
                return Ok(i + 1);
            }
        }

        Err(anyhow!("You shouldn't be able to get here"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> ProblemInput {
        let input = "))(((((";

        crate::helpers::input::ProblemInput::from_sample(input)
    }

    #[test]
    fn sample1() {
        assert_eq!(3, Day1::default().part1(&sample()).unwrap())
    }

    #[test]
    fn sample2() {
        let input = crate::helpers::input::ProblemInput::from_sample("()())");
        assert_eq!(5, Day1::default().part2(&input).unwrap())
    }
}
