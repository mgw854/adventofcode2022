use std::collections::HashMap;

use super::graphplot::Point;

use pathfinding::prelude::bfs;

pub struct HeightMap {
    width: usize,
    height: usize,
    points: HashMap<Point, u8>,
}

impl HeightMap {
    pub fn parse(input: &str, parsing_function: fn(char) -> u8) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut points: HashMap<Point, u8> = HashMap::new();

        for (y, line) in input.lines().enumerate() {
            height = y;
            width = line.trim().len();

            for (x, c) in line.char_indices() {
                points.insert(Point { x, y }, parsing_function(c));
            }
        }

        HeightMap {
            width,
            height,
            points,
        }
    }

    pub fn find_path(
        &self,
        rules: fn(&Point, &HashMap<Point, u8>) -> Vec<Point>,
        start: Point,
        end: Point,
    ) -> Option<usize> {
        let result = bfs(&start, |p| rules(p, &self.points), |p| *p == end);
        result.map(|v| v.len() - 1)
    }
}
