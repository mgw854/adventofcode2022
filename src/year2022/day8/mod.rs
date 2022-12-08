use crate::helpers::graphplot::Line;
use crate::helpers::graphplot::Point;
use crate::{helpers::input::ProblemInput, problem::Problem};
use std::collections::HashMap;

use anyhow::Result;

pub struct Day8 {}

impl Default for Day8 {
    fn default() -> Self {
        Self {}
    }
}

impl Problem<usize, usize> for Day8 {
    fn part1(&self, input: &ProblemInput) -> Result<usize> {
        let raw = input
            .value()
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| (c.to_digit(10).unwrap() + 1) as u8)
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<Vec<u8>>>();

        let mut trees = HashMap::<Point, Interest>::new();

        let mut maxy = 0;
        let mut maxx = 0;

        for (y, line) in raw.iter().enumerate() {
            for (x, t) in line.iter().enumerate() {
                trees.insert(
                    Point { x, y },
                    Interest {
                        height: *t,
                        visible: false,
                        scenic_score: 0,
                    },
                );
                maxx = x;
            }

            maxy = y;
        }

        for y in 0..=maxy {
            for x in 0..=maxx {
                if y == 0 || x == 0 || y == maxy || x == maxx {
                    trees.entry(Point { x, y }).and_modify(|tree| {
                        tree.visible = true;
                    });
                    continue;
                }

                let current_height = trees.get(&Point { x, y }).unwrap().height;

                if is_visible_any(current_height, Point { x, y }, &trees, maxx, maxy) {
                    trees.entry(Point { x, y }).and_modify(|tree| {
                        tree.visible = true;
                    });
                    continue;
                }
            }
        }

        Ok(trees.iter().filter(|t| t.1.visible).count())
    }

    fn part2(&self, input: &ProblemInput) -> Result<usize> {
        let raw = input
            .value()
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| (c.to_digit(10).unwrap() + 1) as u8)
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<Vec<u8>>>();

        let mut trees = HashMap::<Point, Interest>::new();

        let mut maxy = 0;
        let mut maxx = 0;

        for (y, line) in raw.iter().enumerate() {
            for (x, t) in line.iter().enumerate() {
                trees.insert(
                    Point { x, y },
                    Interest {
                        height: *t,
                        visible: false,
                        scenic_score: 0,
                    },
                );
                maxx = x;
            }

            maxy = y;
        }

        for y in 0..=maxy {
            for x in 0..=maxx {
                if y == 0 || x == 0 || y == maxy || x == maxx {
                    trees.entry(Point { x, y }).and_modify(|tree| {
                        tree.scenic_score = 0;
                    });
                    continue;
                }

                let current_height = trees.get(&Point { x, y }).unwrap().height;

                let score = get_scores(current_height, Point { x, y }, &trees, maxx, maxy);

                trees.entry(Point { x, y }).and_modify(|tree| {
                    tree.scenic_score = score;
                });
            }
        }

        Ok(trees.iter().map(|t| t.1.scenic_score).max().unwrap())
    }
}

fn is_visible_any(
    current_height: u8,
    point: Point,
    trees: &HashMap<Point, Interest>,
    maxx: usize,
    maxy: usize,
) -> bool {
    is_visible(
        current_height,
        trees,
        Point {
            x: point.x - 1,
            y: point.y,
        },
        Point { x: 0, y: point.y },
    ) || is_visible(
        current_height,
        trees,
        Point {
            x: point.x + 1,
            y: point.y,
        },
        Point {
            x: maxx,
            y: point.y,
        },
    ) || is_visible(
        current_height,
        trees,
        Point {
            x: point.x,
            y: point.y - 1,
        },
        Point { x: point.x, y: 0 },
    ) || is_visible(
        current_height,
        trees,
        Point {
            x: point.x,
            y: point.y + 1,
        },
        Point {
            x: point.x,
            y: maxy,
        },
    )
}

fn is_visible(
    current_height: u8,
    trees: &HashMap<Point, Interest>,
    start: Point,
    end: Point,
) -> bool {
    let mut visible = true;
    let mut found = false;

    let line = Line { start, end };

    for p in line {
        found = true;
        let next_height = trees.get(&p).unwrap().height;

        if next_height >= current_height {
            visible = false;
            break;
        }
    }

    found && visible
}

fn get_scores(
    current_height: u8,
    point: Point,
    trees: &HashMap<Point, Interest>,
    maxx: usize,
    maxy: usize,
) -> usize {
    get_score(
        current_height,
        trees,
        Point {
            x: point.x - 1,
            y: point.y,
        },
        Point { x: 0, y: point.y },
    ) * get_score(
        current_height,
        trees,
        Point {
            x: point.x + 1,
            y: point.y,
        },
        Point {
            x: maxx,
            y: point.y,
        },
    ) * get_score(
        current_height,
        trees,
        Point {
            x: point.x,
            y: point.y - 1,
        },
        Point { x: point.x, y: 0 },
    ) * get_score(
        current_height,
        trees,
        Point {
            x: point.x,
            y: point.y + 1,
        },
        Point {
            x: point.x,
            y: maxy,
        },
    )
}

fn get_score(
    current_height: u8,
    trees: &HashMap<Point, Interest>,
    start: Point,
    end: Point,
) -> usize {
    let mut score = 0;

    let line = Line { start, end };

    for p in line {
        score += 1;
        let next_height = trees.get(&p).unwrap().height;

        if next_height >= current_height {
            break;
        }
    }

    score
}

#[derive(Debug)]
pub struct Interest {
    height: u8,
    visible: bool,
    scenic_score: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> ProblemInput {
        let input = "30373
25512
65332
33549
35390";

        crate::helpers::input::ProblemInput::from_sample(input)
    }

    #[test]
    fn sample1() {
        assert_eq!(21, Day8::default().part1(&sample()).unwrap())
    }

    #[test]
    fn sample2() {
        assert_eq!(8, Day8::default().part2(&sample()).unwrap())
    }
}
