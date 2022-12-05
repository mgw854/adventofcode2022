use std::{collections::{VecDeque, HashMap}, ops::Index};

use crate::{helpers::input::ProblemInput, problem::Problem};

use anyhow::Result;
use regex::Regex;

pub struct Day5 {}

impl Default for Day5 {
    fn default() -> Self {
        Self {}
    }
}

impl Problem<String, String> for Day5 {
    fn part1(&self, input: &ProblemInput) -> Result<String> {
        let (stack, instructions) = input.value().split_at(input.value().find("\n\n").unwrap());

        let mut stacks = parse_stacks(stack);
        let steps = parse_instructions(instructions);

        for instruction in steps {
            let mut intermediate = VecDeque::new();
            
            let source = stacks.get_mut(&instruction.from).unwrap();

             for _ in 0..instruction.count {
                intermediate.push_front(source.pop_back().unwrap());
            }

            let destination = stacks.get_mut(&instruction.to).unwrap();

            for _ in 0..instruction.count {
                destination.push_back(intermediate.pop_back().unwrap());
            }
        }

        let mut r = String::new();

        for i in 1..=stacks.len() {
            r.push(*stacks.get(&i).unwrap().back().unwrap());
        }
            
        Ok(r)
    }

    fn part2(&self, input: &ProblemInput) -> Result<String> {
        let (stack, instructions) = input.value().split_at(input.value().find("\n\n").unwrap());

        let mut stacks = parse_stacks(stack);
        let steps = parse_instructions(instructions);

        for instruction in steps {
            let mut intermediate = VecDeque::new();
            
            let source = stacks.get_mut(&instruction.from).unwrap();

             for _ in 0..instruction.count {
                intermediate.push_front(source.pop_back().unwrap());
            }

            let destination = stacks.get_mut(&instruction.to).unwrap();

            for _ in 0..instruction.count {
                destination.push_back(intermediate.pop_front().unwrap());
            }
        }

        let mut r = String::new();

        for i in 1..=stacks.len() {
            r.push(*stacks.get(&i).unwrap().back().unwrap());
        }
            
        Ok(r)
    }
}

fn parse_stacks(stack: &str) -> HashMap<usize, VecDeque<char>> {
    let mut stacks : HashMap<usize, VecDeque<char>> = HashMap::new();

    for line in stack.lines().filter(|x| x.contains('[')) {
        for (i,c) in line.char_indices().filter(|(_, x)| *x as u8 >= 65 && *x as u8 <= 90) {
            if let Some(boxstack) = stacks.get_mut(&((i+3)/4)) {
                boxstack.push_front(c);
            } else {
                let mut newboxstack = VecDeque::new();
                newboxstack.push_front(c);
                stacks.insert((i+3)/4, newboxstack);
            }
        }
    }

    stacks
}

fn parse_instructions(instructions: &str) -> Vec<MoveInstructions> {
    let mut output = Vec::new();

    let re = Regex::new(r"(?m)move (?P<count>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();

    for capture in re.captures_iter(&instructions) {
        let count = usize::from_str_radix(&capture["count"], 10).unwrap();
        let from = usize::from_str_radix(&capture["from"], 10).unwrap();
        let to = usize::from_str_radix(&capture["to"], 10).unwrap();

        output.push(MoveInstructions { count, from, to })        
    }

    output
}

struct MoveInstructions {
    count: usize,
    from: usize,
    to: usize
}


#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> ProblemInput {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        crate::helpers::input::ProblemInput::from_sample(input)
    }

    #[test]
    fn sample1() {
        assert_eq!("CMZ", Day5::default().part1(&sample()).unwrap())
    }

    
    #[test]
    fn sample2() {
        assert_eq!("MCD", Day5::default().part2(&sample()).unwrap())
    }
}