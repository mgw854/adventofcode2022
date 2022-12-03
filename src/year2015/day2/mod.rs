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
        let lines = input
            .parse_lines(|x| Ok(x.split('x').map(|s| s.to_owned()).collect::<Vec<String>>()))?;

        let mut sqft = 0;

        for line in lines {
            let l: usize = line[0].parse()?;
            let w: usize = line[1].parse()?;
            let h: usize = line[2].parse()?;

            let sides = vec![l * w, w * h, l * h];

            sqft += 2 * l * w + 2 * w * h + 2 * h * l + sides.iter().min().unwrap();
        }

        Ok(sqft)
    }

    fn part2(&self, input: &ProblemInput) -> Result<usize> {
        let lines = input
            .parse_lines(|x| Ok(x.split('x').map(|s| s.to_owned()).collect::<Vec<String>>()))?;

        let mut ft = 0;

        for line in lines {
            let l: usize = line[0].parse()?;
            let w: usize = line[1].parse()?;
            let h: usize = line[2].parse()?;

            let sides = vec![2 * l + 2 * w, 2 * w + 2 * h, 2 * l + 2 * h];

            ft += l * w * h + sides.iter().min().unwrap();
        }

        Ok(ft)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> ProblemInput {
        let input = "2x3x4
1x1x10";

        crate::helpers::input::ProblemInput::from_sample(input)
    }

    #[test]
    fn sample1() {
        assert_eq!(101, Day2::default().part1(&sample()).unwrap())
    }

    #[test]
    fn sample2() {
        assert_eq!(48, Day2::default().part2(&sample()).unwrap())
    }
}
