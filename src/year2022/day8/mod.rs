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

        dbg!(&trees);
        
        Ok(trees.iter().filter(|t| t.1.visible).count())
    }

    fn part2(&self, input: &ProblemInput) -> Result<usize> {
        Ok(0)
    }
}

fn is_visible_any(current_height: u8, point: Point, trees: &HashMap<Point, Interest>, maxx: usize, maxy: usize) -> bool {
    if point == (Point { x: 3, y: 3 }) {
        dbg!(point);
    }

  is_visible(current_height, trees, Point { x: point.x - 1, y: point.y }, Point { x: 0, y: point.y }) ||
  is_visible(current_height, trees, Point { x: point.x + 1, y: point.y }, Point { x: maxx, y: point.y }) ||
  is_visible(current_height, trees, Point { x: point.x, y: point.y - 1 }, Point { x: point.x, y: 0 }) ||
  is_visible(current_height, trees, Point { x: point.x, y: point.y + 1 }, Point { x: point.x, y: maxy })
}

fn is_visible(current_height: u8, trees: &HashMap<Point, Interest>, start: Point, end: Point) -> bool {
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

#[derive(Debug)]
pub struct Interest {
    height: u8,
    visible: bool,
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
        assert_eq!(0, Day8::default().part2(&sample()).unwrap())
    }
}
