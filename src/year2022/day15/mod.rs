use std::collections::HashSet;

use crate::{helpers::input::ProblemInput, problem::Problem};
use crate::helpers::negativegraphplot::Point;
use anyhow::{anyhow, Result};
use regex::Regex;

pub struct Day15 {}

impl Default for Day15 {
    fn default() -> Self {
        Self {}
    }
}

impl Problem<usize, usize> for Day15 {
    fn part1(&self, input: &ProblemInput) -> Result<usize> {
        let pairs = parse_pairs(input.value());

        let mut row2000000 = HashSet::new();

        for p in pairs.iter() {
            let manhattan_distance = p.manhattan_distance();
            for point in  get_manhattan_points(&p.sensor, manhattan_distance).into_iter().filter(|x| x.y == 2000000) {
                row2000000.insert(point);
            }
        }

        for p in pairs {
            row2000000.remove(&p.beacon);
        }

        Ok(row2000000.len())
    }

    fn part2(&self, input: &ProblemInput) -> Result<usize> {
        let max = 4000000;
        let pairs = parse_pairs(input.value());
        
        let distress = find_distress_beacon(&pairs, max);
        let tuning_frequency = (distress.x as usize) * 4000000 + (distress.y as usize);

        // 1888744000000 is too low
        Ok(tuning_frequency)
    }
}

fn find_distress_beacon(pairs: &Vec<SensorBeaconPair>, max: i64) -> Point {
    for y in 0..=max {
        let mut points = HashSet::new();

        for x in 0..=max {
            points.insert(Point { x, y });
        }

        for p in pairs.iter() {
            let manhattan_distance = p.manhattan_distance();
            for point in  get_manhattan_points2(&p.sensor, manhattan_distance, y).into_iter() {
                points.remove(&point);
            }
        }        

        if points.len() > 0 {
            return points.into_iter().next().unwrap();
        }
    }


    panic!();
}

fn parse_pairs(instructions: &str) -> Vec<SensorBeaconPair> {
    let mut output = Vec::new();

    let re = Regex::new(r"(?m)Sensor at x=(?P<sx>-?\d+), y=(?P<sy>-?\d+): closest beacon is at x=(?P<bx>-?\d+), y=(?P<by>-?\d+)").unwrap();

    for capture in re.captures_iter(&instructions) {
        let sensorx = i64::from_str_radix(&capture["sx"], 10).unwrap();
        let sensory = i64::from_str_radix(&capture["sy"], 10).unwrap();
        let beaconx = i64::from_str_radix(&capture["bx"], 10).unwrap();
        let beacony = i64::from_str_radix(&capture["by"], 10).unwrap();

        output.push(SensorBeaconPair { sensor: Point { x: sensorx, y: sensory }, beacon: Point { x: beaconx, y: beacony } });
    }

    output
}

struct SensorBeaconPair {
    sensor: Point,
    beacon: Point
}

impl SensorBeaconPair {
    fn manhattan_distance(&self) -> i64 {
        (self.sensor.x.abs_diff(self.beacon.x) + self.sensor.y.abs_diff(self.beacon.y)) as i64
    }
}

fn get_manhattan_points(point: &Point, manhattan_distance: i64) -> Vec<Point> {
    let mut points = Vec::new();

    let minx = point.x - manhattan_distance;
    let miny = point.y - manhattan_distance;
    let maxx = point.x + manhattan_distance;
    let maxy = point.y + manhattan_distance;

    if miny <= 2000000 && 2000000 <= maxy {
        let y = 2000000;
        for x in minx..=maxx {
            //for y in miny..=maxy {
                let proposed = Point { x, y };
    
                if proposed.manhattan_distance(&point) <= manhattan_distance {
                    points.push(proposed);
                }
            //}
        }
    }

    points
}


fn get_manhattan_points2(point: &Point, manhattan_distance: i64, y: i64) -> Vec<Point> {
    let mut points = Vec::new();

    let minx = (point.x - manhattan_distance).clamp(0, 4000000);
    let miny = (point.y - manhattan_distance).clamp(0, 4000000);
    let maxx = (point.x + manhattan_distance).clamp(0, 4000000);
    let maxy = (point.y + manhattan_distance).clamp(0, 4000000);

    if miny <= y && y <= maxy {
        for x in minx..=maxx {
            let proposed = Point { x, y };

            if proposed.manhattan_distance(&point) <= manhattan_distance {
                points.push(proposed);
            }
    }
}

    points
}


#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> ProblemInput {
        let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

        crate::helpers::input::ProblemInput::from_sample(input)
    }

    #[test]
    fn sample1() {
        assert_eq!(26, Day15::default().part1(&sample()).unwrap())
    }

    
    #[tecast]
    fn sample2() {
        assert_eq!(56000011, Day15::default().part2(&sample()).unwrap())
    }
}