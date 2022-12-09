use std::collections::HashMap;

use crate::{helpers::input::ProblemInput, problem::Problem};

use anyhow::{anyhow, Result};

pub struct Day7 {}

impl Default for Day7 {
    fn default() -> Self {
        Self {}
    }
}

impl Problem<usize, usize> for Day7 {
    fn part1(&self, input: &ProblemInput) -> Result<usize> {
        let mut wires = HashMap::new();

        for instruction in input.parse_lines(op_parser) {
            wires
        }

        Ok(0)
    }

    fn part2(&self, input: &ProblemInput) -> Result<usize> {
        todo!()
    }
}

fn op_parser(input: &str) -> Result<Op> {
    let components : Vec<&str> = input.split(' ').collect();
    
    if components[0] == "NOT" {
        Ok(Op::Not(components[1], components[2]))
    } else {
        match components[1] {
            "->" => Ok(Op::Set(components[2], components[0].parse().unwrap())),
            "AND" => Ok(Op::And(components[0], components[2], components[4])),
            "OR" => Ok(Op::Or(components[0], components[2], components[4])),
            "LSHIFT" => Ok(Op::Lshift(components[0], components[2].parse().unwrap(), components[4])),
            "RSHIFT" =>  Ok(Op::Rshift(components[0], components[2].parse().unwrap(), components[4])),
            _ => Err(anyhow!("Bad Op"))
        }
    }
}

pub enum Op<'a> {
    Set(&'a str, u16),
    And(&'a str, &'a str, &'a str),
    Or(&'a str, &'a str, &'a str),
    Lshift(&'a str, u8, &'a str),
    Rshift(&'a str, u8, &'a str),
    Not(&'a str, &'a str)
}


#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> ProblemInput {
        let input = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";

        crate::helpers::input::ProblemInput::from_sample(input)
    }

    #[test]
    fn sample1() {
        assert_eq!(0, Day7::default().part1(&sample()).unwrap())
    }

    
    #[test]
    fn sample2() {
        assert_eq!(0, Day7::default().part2(&sample()).unwrap())
    }
}