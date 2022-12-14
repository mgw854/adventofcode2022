use std::{collections::HashSet, vec};

use crate::{
    helpers::{graphplot::Line, graphplot::Point, input::ProblemInput},
    problem::Problem,
};

use anyhow::{anyhow, Result};
use itertools::Itertools;

pub struct Day14 {}

impl Default for Day14 {
    fn default() -> Self {
        Self {}
    }
}

impl Problem<usize, usize> for Day14 {
    fn part1(&self, input: &ProblemInput) -> Result<usize> {
        let rocks = input
            .value()
            .lines()
            .map(|l| get_lines(l))
            .flatten()
            .map(|lin| lin.into_iter())
            .flatten()
            .collect::<HashSet<Point>>();
        let mut sand: HashSet<Point> = HashSet::new();

        let mut pos = Point { x: 500, y: 0 };

        while pos.y < 200 {
            // Can drop one?
            let mut attempt = Point {
                x: pos.x,
                y: pos.y + 1,
            };

            if !rocks.contains(&attempt) && !sand.contains(&attempt) {
                pos = attempt;
                continue;
            }

            // Not possible, shift left one
            attempt = Point {
                x: pos.x - 1,
                y: pos.y + 1,
            };

            if !rocks.contains(&attempt) && !sand.contains(&attempt) {
                pos = attempt;
                continue;
            }

            // Not possible, shift right instead
            attempt = Point {
                x: pos.x + 1,
                y: pos.y + 1,
            };

            if !rocks.contains(&attempt) && !sand.contains(&attempt) {
                pos = attempt;
                continue;
            }

            // Not possible, the unit of sand stops here
            sand.insert(pos);
            pos = Point { x: 500, y: 0 };
        }

        Ok(sand.len())
    }

    fn part2(&self, input: &ProblemInput) -> Result<usize> {
        let mut rocks = input
            .value()
            .lines()
            .map(|l| get_lines(l))
            .flatten()
            .map(|lin| lin.into_iter())
            .flatten()
            .collect::<HashSet<Point>>();
        let mut sand: HashSet<Point> = HashSet::new();

        let floor = rocks.iter().map(|f| f.y).max().unwrap() + 2;

        for x in 0..=1000 {
            rocks.insert(Point { x, y: floor });
        }

        let mut pos = Point { x: 500, y: 0 };

        while !sand.contains(&Point { x: 500, y: 0 }) {
            // Can drop one?
            let mut attempt = Point {
                x: pos.x,
                y: pos.y + 1,
            };

            if !rocks.contains(&attempt) && !sand.contains(&attempt) {
                pos = attempt;
                continue;
            }

            // Not possible, shift left one
            attempt = Point {
                x: pos.x - 1,
                y: pos.y + 1,
            };

            if !rocks.contains(&attempt) && !sand.contains(&attempt) {
                pos = attempt;
                continue;
            }

            // Not possible, shift right instead
            attempt = Point {
                x: pos.x + 1,
                y: pos.y + 1,
            };

            if !rocks.contains(&attempt) && !sand.contains(&attempt) {
                pos = attempt;
                continue;
            }

            // Not possible, the unit of sand stops here
            sand.insert(pos);
            pos = Point { x: 500, y: 0 };
        }

        Ok(sand.len())
    }
}

fn get_lines(l: &str) -> Vec<Line> {
    let mut ret = Vec::new();

    for (a, b) in l.split(" -> ").tuple_windows() {
        let a = a.parse::<Point>().unwrap();
        let b = b.parse::<Point>().unwrap();
        ret.push(Line { start: a, end: b });
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> ProblemInput {
        let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

        crate::helpers::input::ProblemInput::from_sample(input)
    }

    #[test]
    fn sample1() {
        assert_eq!(24, Day14::default().part1(&sample()).unwrap())
    }

    #[test]
    fn sample2() {
        assert_eq!(93, Day14::default().part2(&sample()).unwrap())
    }
}
