use std::collections::HashSet;

use super::graphplot::{Point, Slope};

pub struct Map {
    width: usize,
    height: usize,
    points_of_interest: HashSet<Point>
}

impl Map {
    fn parse(input: String, interest_marker: char) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut points_of_interest: HashSet<Point> = HashSet::new();
        
        

        for (y, line) in input.lines().enumerate() {
            height = y;
            width = line.trim().len();

            for (x, _) in line.char_indices().filter(|(_, ch)| ch == &interest_marker) {
                points_of_interest.insert(Point { x, y });
            }
        }

        Map { width, height, points_of_interest }
    }

    fn waypoint_hits(&self, start: &Point, slope: &Slope) -> usize {
        let mut now = start.clone();
        let mut hits = 0;

        while now.x <= self.width && now.y <= self.height {
            if self.points_of_interest.contains(&now) {
                hits += 1;
            }

            now = now + slope;
        }

        hits
    }
}