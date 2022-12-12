use std::collections::HashMap;

use crate::{
    helpers::{graphplot::Point, input::ProblemInput, topography::HeightMap},
    problem::Problem,
};

use anyhow::{anyhow, Result};

pub struct Day12 {}

impl Default for Day12 {
    fn default() -> Self {
        Self {}
    }
}

impl Problem<usize, usize> for Day12 {
    fn part1(&self, input: &ProblemInput) -> Result<usize> {
        let start = input.value().replace('\n', "").find('S').unwrap();
        let end = input.value().replace('\n', "").find('E').unwrap();
        let eol = input.value().find('\n').unwrap();

        let start = Point {
            x: start % eol,
            y: start / eol,
        };
        let end = Point {
            x: end % eol,
            y: end / eol,
        };

        let map = HeightMap::parse(input.value(), parse_char);
        Ok(map.find_path(find_path_rule, start, end).unwrap())
    }

    fn part2(&self, input: &ProblemInput) -> Result<usize> {
        let start = input.value().replace('\n', "").find('S').unwrap();
        let end = input.value().replace('\n', "").find('E').unwrap();
        let eol = input.value().find('\n').unwrap();

        let end = Point {
            x: end % eol,
            y: end / eol,
        };

        let map = HeightMap::parse(input.value(), parse_char);

        let mut options = Vec::new();

        for (i, _) in input
            .value()
            .replace('\n', "")
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == 'a' || *c == 'S')
        {
            let start = Point {
                x: i % eol,
                y: i / eol,
            };

            if let Some(found) = map.find_path(find_path_rule, start, end) {
                options.push(found);
            }
        }

        Ok(*options.iter().min().unwrap())
    }
}

fn find_path_rule(point: &Point, map: &HashMap<Point, u8>) -> Vec<Point> {
    let height_limit = *map.get(point).unwrap() + 1;

    let potential_neighbors = if point.x == 0 && point.y == 0 {
        vec![
            Point {
                x: point.x + 1,
                y: point.y,
            },
            Point {
                x: point.x,
                y: point.y + 1,
            },
        ]
    } else if point.x == 0 {
        vec![
            Point {
                x: point.x,
                y: point.y - 1,
            },
            Point {
                x: point.x + 1,
                y: point.y,
            },
            Point {
                x: point.x,
                y: point.y + 1,
            },
        ]
    } else if point.y == 0 {
        vec![
            Point {
                x: point.x - 1,
                y: point.y,
            },
            Point {
                x: point.x + 1,
                y: point.y,
            },
            Point {
                x: point.x,
                y: point.y + 1,
            },
        ]
    } else {
        vec![
            Point {
                x: point.x,
                y: point.y - 1,
            },
            Point {
                x: point.x - 1,
                y: point.y,
            },
            Point {
                x: point.x + 1,
                y: point.y,
            },
            Point {
                x: point.x,
                y: point.y + 1,
            },
        ]
    };

    potential_neighbors
        .into_iter()
        .filter(|x| map.get(x).map(|s| *s <= height_limit).unwrap_or(false))
        .collect()
}

fn parse_char(c: char) -> u8 {
    if c == 'S' {
        0
    } else if c == 'E' {
        25
    } else {
        (c as u8) - 97
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> ProblemInput {
        let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

        crate::helpers::input::ProblemInput::from_sample(input)
    }

    #[test]
    fn sample1() {
        assert_eq!(31, Day12::default().part1(&sample()).unwrap())
    }

    #[test]
    fn sample2() {
        assert_eq!(29, Day12::default().part2(&sample()).unwrap())
    }
}
