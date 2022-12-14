use std::{ops::Add, str::FromStr};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Ord, PartialOrd)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s
            .split(',')
            .map(|p| p.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        Ok(Point {
            x: values[0],
            y: values[1],
        })
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Slope {
    pub dx: i64,
    pub dy: i64,
}

pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Add<Slope> for Point {
    fn add(self, other: Slope) -> Self {
        Self {
            x: (self.x + other.dx),
            y: (self.y + other.dy)
        }
    }
    type Output = Point;
}

impl Add<&Slope> for Point {
    fn add(self, other: &Slope) -> Self {
        Self {
            x: (self.x + other.dx),
            y: (self.y + other.dy)
        }
    }
    type Output = Point;
}

impl From<Slope> for f64 {
    fn from(slope: Slope) -> Self {
        (slope.dy as f64) / (slope.dx as f64)
    }
}

impl Point {
    pub fn slope_between(&self, other: &Point) -> Slope {
        Slope {
            dy: -1 * (self.y - other.y),
            dx: -1 * (self.x - other.x),
        }
    }

    pub fn manhattan_distance(&self, other: &Point) -> i64 {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as i64     
    }
}

impl Slope {
    fn reduce(self) -> Slope {
        let gcd = num::integer::gcd(self.dx, self.dy);

        return Slope {
            dx: (self.dx / gcd),
            dy: (self.dy / gcd),
        };
    }
}

pub struct LinePointIterator {
    ret: Point,
    end: Point,
    slope: Slope,
    trip: bool,
}

impl IntoIterator for Line {
    type Item = Point;
    type IntoIter = LinePointIterator;

    fn into_iter(self) -> Self::IntoIter {
        if self.start == self.end {
            return LinePointIterator {
                ret: self.start,
                end: self.end,
                slope: Slope { dx: 1, dy: 1 },
                trip: false,
            };
        }

        LinePointIterator {
            ret: self.start,
            end: self.end,
            slope: self.start.slope_between(&self.end).reduce(),
            trip: false,
        }
    }
}

impl Iterator for LinePointIterator {
    type Item = Point;
    fn next(&mut self) -> Option<Point> {
        if !self.trip {
            let old = self.ret;

            if old.x == self.end.x && old.y == self.end.y {
                self.trip = true;
            } else {
                let next_point = old + self.slope;
                self.ret = next_point;
            }

            return Some(old);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slope4x4() {
        let start = Point { x: 1, y: 1 };
        let end = Point { x: 5, y: 5 };
        let slope = start.slope_between(&end);

        assert_eq!(Slope { dx: 4, dy: 4 }, slope)
    }

    #[test]
    fn slope_reduce4x4() {
        let start = Point { x: 1, y: 1 };
        let end = Point { x: 5, y: 5 };
        let slope = start.slope_between(&end).reduce();

        assert_eq!(Slope { dx: 1, dy: 1 }, slope)
    }

    #[test]
    fn slope_iter_positives() {
        let start = Point { x: 1, y: 1 };
        let end = Point { x: 5, y: 5 };

        let steps: Vec<Point> = Line { start, end }.into_iter().collect();

        let expected = vec![
            Point { x: 1, y: 1 },
            Point { x: 2, y: 2 },
            Point { x: 3, y: 3 },
            Point { x: 4, y: 4 },
            Point { x: 5, y: 5 },
        ];

        assert_eq!(expected, steps);
    }

    #[test]
    fn slope_iter_negative() {
        let start = Point { x: 5, y: 5 };
        let end = Point { x: 1, y: 1 };

        let steps: Vec<Point> = Line { start, end }.into_iter().collect();

        let expected = vec![
            Point { x: 5, y: 5 },
            Point { x: 4, y: 4 },
            Point { x: 3, y: 3 },
            Point { x: 2, y: 2 },
            Point { x: 1, y: 1 },
        ];

        assert_eq!(expected, steps);
    }

    #[test]
    fn slope_iter_vertical() {
        let start = Point { x: 5, y: 1 };
        let end = Point { x: 5, y: 5 };

        let steps: Vec<Point> = Line { start, end }.into_iter().collect();

        let expected = vec![
            Point { x: 5, y: 1 },
            Point { x: 5, y: 2 },
            Point { x: 5, y: 3 },
            Point { x: 5, y: 4 },
            Point { x: 5, y: 5 },
        ];

        assert_eq!(expected, steps);
    }

    #[test]
    fn slope_single_point_none() {
        let start = Point { x: 5, y: 5 };
        let end = Point { x: 5, y: 5 };

        let steps: Vec<Point> = Line { start, end }.into_iter().collect();

        let expected = vec![Point { x: 5, y: 5 }];

        assert_eq!(expected, steps);
    }
}
