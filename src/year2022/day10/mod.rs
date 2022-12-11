use std::str::FromStr;

use crate::{helpers::input::ProblemInput, problem::Problem};

use anyhow::{anyhow, Result};

pub struct Day10 {}

impl Default for Day10 {
    fn default() -> Self {
        Self {}
    }
}

impl Problem<i64, String> for Day10 {
    fn part1(&self, input: &ProblemInput) -> Result<i64> {
        let mut instructions = input
            .parse_lines(|l| l.parse::<OpCode>().map_err(|_| anyhow!("Bad format")))?
            .into_iter();

        let mut sum = 0;
        let mut x = 1;
        let mut cycle = 0;

        while let Some(instruction) = instructions.next() {
            cycle = cycle + 1;
            match instruction {
                OpCode::AddX(v) => {
                    if care_cycle(cycle) {
                        sum += cycle * x;
                    }

                    cycle = cycle + 1;
                }
                OpCode::Noop => {}
            };

            if care_cycle(cycle) {
                sum += cycle * x;
            };

            match instruction {
                OpCode::AddX(v) => {
                    x += v;
                }
                OpCode::Noop => {}
            };
        }

        return Ok(sum);
    }

    fn part2(&self, input: &ProblemInput) -> Result<String> {
        let mut instructions = input
            .parse_lines(|l| l.parse::<OpCode>().map_err(|_| anyhow!("Bad format")))?
            .into_iter();

        let mut x: i64 = 1;
        let mut cycle: i64 = 0;
        let mut pos : i64 = -1;

        let mut crt = String::new();

        while let Some(instruction) = instructions.next() {
            cycle = cycle + 1;
            pos = (pos + 1) % 40;

            if pos.abs_diff(x) <= 1 {
                dbg!(cycle);
                dbg!(x);
                crt.push('#');
            } else {
                dbg!(cycle);
                dbg!(x);
               crt.push('.');
            }

            if cycle % 40 == 0 {
                crt.push('\n');
            }

            match instruction {
                OpCode::AddX(_) => {
                    cycle = cycle + 1;
                    pos = (pos + 1) % 40;

                    if pos.abs_diff(x) <= 1 {
                        dbg!(cycle);
                        dbg!(x);
                        crt.push('#');
                    } else {
                        dbg!(cycle);
                        dbg!(x);
                             crt.push('.');
                    }
        
                    if cycle % 40 == 0 {
                        crt.push('\n');
                    }        
                }
                OpCode::Noop => {}
            };

            match instruction {
                OpCode::AddX(v) => {
                    x += v;
                }
                OpCode::Noop => {}
            };
        }

        return Ok(crt);
    }
}

fn care_cycle(cycle: i64) -> bool {
    cycle == 20 || cycle == 60 || cycle == 100 || cycle == 140 || cycle == 180 || cycle == 220
}

pub enum OpCode {
    AddX(i64),
    Noop,
}

impl FromStr for OpCode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let args = s.split(' ').collect::<Vec<&str>>();

        let answer = match args[0] {
            "addx" => OpCode::AddX(args[1].parse().unwrap()),
            "noop" => OpCode::Noop,
            _ => {
                return Err(());
            }
        };

        Ok(answer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> ProblemInput {
        let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

        crate::helpers::input::ProblemInput::from_sample(input)
    }

    #[test]
    fn sample1() {
        assert_eq!(13140, Day10::default().part1(&sample()).unwrap())
    }

    #[test]
    fn sample2() {
        let output = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";
        assert_eq!(output, Day10::default().part2(&sample()).unwrap())
    }
}
