use std::{str::FromStr, collections::HashSet};

use crate::{helpers::{input::ProblemInput, graphplot::Point}, problem::Problem};

use anyhow::{anyhow, Result};

pub struct Day9 {}

impl Default for Day9 {
    fn default() -> Self {
        Self {}
    }
}

impl Problem<usize, usize> for Day9 {
    fn part1(&self, input: &ProblemInput) -> Result<usize> {
        let instructions = input.parse_lines(|l| l.parse::<MoveInstruction>().map_err(|_| anyhow!("Bad format")))?;
        
        let mut visited = HashSet::new();

        let mut head = Point { x: 1000, y: 1000 };
        let mut tail = head;
        visited.insert(tail);

        for instruction in instructions {
            for i in 0..instruction.times {
                head = match instruction.direction {
                    Direction::Up => Point { x: head.x, y: head.y - 1 },
                    Direction::Down => Point { x: head.x, y: head.y + 1 },
                    Direction::Left => Point { x: head.x - 1, y: head.y },
                    Direction::Right => Point { x: head.x + 1, y: head.y },
                };
                
                tail = follow(head, tail);

                visited.insert(tail);
            }
        }

        Ok(visited.len())
    }

    fn part2(&self, input: &ProblemInput) -> Result<usize> {
        let instructions = input.parse_lines(|l| l.parse::<MoveInstruction>().map_err(|_| anyhow!("Bad format")))?;
        
        let mut visited = HashSet::new();

        let mut head = Point { x: 1000, y: 1000 };
        let mut k1 = head;
        let mut k2 = head;
        let mut k3 = head;
        let mut k4 = head;
        let mut k5 = head;
        let mut k6 = head;
        let mut k7 = head;
        let mut k8 = head;
        let mut k9 = head;
        visited.insert(k9);

        for instruction in instructions {
            for i in 0..instruction.times {
                head = match instruction.direction {
                    Direction::Up => Point { x: head.x, y: head.y - 1 },
                    Direction::Down => Point { x: head.x, y: head.y + 1 },
                    Direction::Left => Point { x: head.x - 1, y: head.y },
                    Direction::Right => Point { x: head.x + 1, y: head.y },
                };
                
                k1 = follow(head, k1);
                k2 = follow(k1, k2);
                k3 = follow(k2, k3);
                k4 = follow(k3, k4);
                k5 = follow(k4, k5);
                k6 = follow(k5, k6);
                k7 = follow(k6, k7);
                k8 = follow(k7, k8);
                k9 = follow(k8, k9);

                visited.insert(k9);
            }
        }

        Ok(visited.len())
    }
}

fn follow(head: Point, tail: Point) -> Point {
    if head.x.abs_diff(tail.x) > 1 || head.y.abs_diff(tail.y) > 1 {
        if head.x != tail.x && head.y != tail.y {
            // Move the tail diagonally
            if head.x > tail.x && head.y > tail.y {
                Point { x: tail.x + 1, y: tail.y + 1 }
            } else if head.x > tail.x && head.y < tail.y {
                Point { x: tail.x + 1, y: tail.y - 1 }
            } else if head.x < tail.x && head.y > tail.y {
                Point { x: tail.x - 1, y: tail.y + 1 }
            } else if head.x < tail.x && head.y < tail.y {
                Point { x: tail.x - 1, y: tail.y - 1 }
            } else {
                tail
            }
        } else if head.x == tail.x {
            // move toward head on y
            if head.y > tail.y {
                Point { x: tail.x, y: tail.y + 1 }
            } else {
                Point { x: tail.x, y: tail.y - 1 }
            }
        } else if head.y == tail.y {
            // Move toward head on x
            if head.x > tail.x {
                Point { x: tail.x + 1, y: tail.y }
            } else {
                Point { x: tail.x - 1, y: tail.y }
            }
        } else {
            tail
        }
    } else {
        tail
    }
}

pub struct MoveInstruction {
    pub direction: Direction,
    pub times: usize
}

impl FromStr for MoveInstruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect::<Vec<&str>>();

        Ok(MoveInstruction { direction: parts[0].parse()?, times: parts[1].parse().unwrap() })
    }
}

pub enum Direction {
    Left,
    Right,
    Up,
    Down
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            _ => Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> ProblemInput {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

        crate::helpers::input::ProblemInput::from_sample(input)
    }

    #[test]
    fn sample1() {
        assert_eq!(13, Day9::default().part1(&sample()).unwrap())
    }

    
    #[test]
    fn sample2() {
        let input = "R 5
        U 8
        L 8
        D 3
        R 17
        D 10
        L 25
        U 20";
        
                
        assert_eq!(36, Day9::default().part2(&crate::helpers::input::ProblemInput::from_sample(input)).unwrap())
    }
}