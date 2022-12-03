use std::collections::HashSet;

use crate::{helpers::input::ProblemInput, problem::Problem};

use anyhow::Result;

pub struct Day3 {}

impl Default for Day3 {
    fn default() -> Self {
        Self {}
    }
}

impl Problem<usize, usize> for Day3 {
    fn part1(&self, input: &ProblemInput) -> Result<usize> {
        let lines = input.parse_lines(|x| Ok(x.chars().collect::<Vec<char>>()))?;
        let mut priority = 0;

        for mut line in lines {
            let second: HashSet<char> = line.split_off(line.len() / 2).into_iter().collect();
            let first: HashSet<char> = line.into_iter().collect();

            for x in first.intersection(&second) {
                let val = *x as usize;

                if val >= 97 {
                    priority += val - 96;
                } else {
                    priority += (val - 64) + 26;
                }
            }
        }

        Ok(priority)
    }

    fn part2(&self, input: &ProblemInput) -> Result<usize> {
        let lines = input.parse_lines(|x| Ok(x.chars().collect::<HashSet<char>>()))?;
        let mut priority = 0;

        let mut i = 0;

        while i < lines.len() {
            let a = find_badge(&lines[i], &lines[i + 1], &lines[i + 2]) as usize;
            let b = find_badge(&lines[i + 3], &lines[i + 4], &lines[i + 5]) as usize;

            if a >= 97 {
                priority += a - 96;
            } else {
                priority += (a - 64) + 26;
            }

            if b >= 97 {
                priority += b - 96;
            } else {
                priority += (b - 64) + 26;
            }

            i = i + 6;
        }

        Ok(priority)
    }
}

fn find_badge(one: &HashSet<char>, two: &HashSet<char>, three: &HashSet<char>) -> char {
    let intersection: HashSet<char> = one.intersection(two).map(|x| *x).collect();

    for x in intersection.intersection(three) {
        return *x;
    }

    panic!("At the disco")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> ProblemInput {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        crate::helpers::input::ProblemInput::from_sample(input)
    }

    #[test]
    fn sample1() {
        assert_eq!(157, Day3::default().part1(&sample()).unwrap())
    }

    #[test]
    fn sample2() {
        assert_eq!(70, Day3::default().part2(&sample()).unwrap())
    }
}
