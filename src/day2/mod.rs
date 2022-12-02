use std::str::FromStr;

use crate::{helpers::input::ProblemInput, problem::Problem};

use anyhow::{anyhow, Result};

pub struct Day2 {}

impl Default for Day2 {
    fn default() -> Self {
        Self {}
    }
}

impl Problem<usize, usize> for Day2 {
    fn part1(&self, input: &ProblemInput) -> Result<usize> {
        let rounds = input.parse_lines(|x| {
            x.parse::<Round>()
                .map_err(|e| anyhow!("Missing attribute: {}", ""))
        })?;
        Ok(rounds.iter().map(|r| r.score()).sum())
    }

    fn part2(&self, input: &ProblemInput) -> Result<usize> {
        let rounds = input.parse_lines(|x| {
            x.parse::<DeterministicRound>()
                .map_err(|e| anyhow!("Missing attribute: {}", ""))
        })?;
        Ok(rounds.iter().map(|r| r.score()).sum())
    }
}

pub enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<char> for RPS {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(RPS::Rock),
            'B' => Ok(RPS::Paper),
            'C' => Ok(RPS::Scissors),
            'X' => Ok(RPS::Rock),
            'Y' => Ok(RPS::Paper),
            'Z' => Ok(RPS::Scissors),
            _ => Err(()),
        }
    }
}

pub struct Round {
    pub opponent: RPS,
    pub you: RPS,
}

pub struct DeterministicRound {
    pub opponent: RPS,
    pub you: Outcome,
}

pub enum Outcome {
    Win,
    Lose,
    Tie,
}

impl TryFrom<char> for Outcome {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Outcome::Lose),
            'Y' => Ok(Outcome::Tie),
            'Z' => Ok(Outcome::Win),
            _ => Err(()),
        }
    }
}

impl Round {
    pub fn score(&self) -> usize {
        let shape_score = match self.you {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        };

        let outcome_score = match self.outcome() {
            Outcome::Lose => 0,
            Outcome::Tie => 3,
            Outcome::Win => 6,
        };

        shape_score + outcome_score
    }

    fn outcome(&self) -> Outcome {
        match self.you {
            RPS::Rock => match self.opponent {
                RPS::Paper => Outcome::Lose,
                RPS::Scissors => Outcome::Win,
                _ => Outcome::Tie,
            },
            RPS::Paper => match self.opponent {
                RPS::Scissors => Outcome::Lose,
                RPS::Rock => Outcome::Win,
                _ => Outcome::Tie,
            },
            RPS::Scissors => match self.opponent {
                RPS::Rock => Outcome::Lose,
                RPS::Paper => Outcome::Win,
                _ => Outcome::Tie,
            },
        }
    }
}

impl FromStr for Round {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let matches: Vec<&str> = s.split(' ').collect();

        Ok(Round {
            opponent: matches[0].chars().next().unwrap().try_into()?,
            you: matches[1].chars().next().unwrap().try_into()?,
        })
    }
}

impl FromStr for DeterministicRound {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let matches: Vec<&str> = s.split(' ').collect();

        Ok(DeterministicRound {
            opponent: matches[0].chars().next().unwrap().try_into()?,
            you: matches[1].chars().next().unwrap().try_into()?,
        })
    }
}

impl DeterministicRound {
    pub fn score(&self) -> usize {
        let shape_score = match self.outcome_shape() {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        };

        let outcome_score = match self.you {
            Outcome::Lose => 0,
            Outcome::Tie => 3,
            Outcome::Win => 6,
        };

        shape_score + outcome_score
    }

    fn outcome_shape(&self) -> RPS {
        match self.you {
            Outcome::Lose => match self.opponent {
                RPS::Rock => RPS::Scissors,
                RPS::Paper => RPS::Rock,
                RPS::Scissors => RPS::Paper,
            },
            Outcome::Tie => match self.opponent {
                RPS::Rock => RPS::Rock,
                RPS::Paper => RPS::Paper,
                RPS::Scissors => RPS::Scissors,
            },
            Outcome::Win => match self.opponent {
                RPS::Rock => RPS::Paper,
                RPS::Paper => RPS::Scissors,
                RPS::Scissors => RPS::Rock,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> ProblemInput {
        let input = "A Y
B X
C Z";

        crate::helpers::input::ProblemInput::from_sample(input)
    }

    #[test]
    fn sample1() {
        assert_eq!(15, Day2::default().part1(&sample()).unwrap())
    }

    #[test]
    fn sample2() {
        assert_eq!(12, Day2::default().part2(&sample()).unwrap())
    }
}
