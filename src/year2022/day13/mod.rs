use std::{ops::Index, str::FromStr};

use crate::{helpers::input::ProblemInput, problem::Problem};

use anyhow::{anyhow, Result};
use chumsky::prelude::*;

pub struct Day13 {}

impl Default for Day13 {
    fn default() -> Self {
        Self {}
    }
}

impl Problem<usize, usize> for Day13 {
    fn part1(&self, input: &ProblemInput) -> Result<usize> {
        let inputs = input
            .parse_sections(|s| parser().parse(s).map_err(|e| anyhow!("Parse failed")))
            .unwrap();

        let mut correct_order = 0;

        for (i, pair) in inputs.iter().enumerate() {
            if pair[0] < pair[1] {
                correct_order += i + 1;
            }
        }

        Ok(correct_order)
    }

    fn part2(&self, input: &ProblemInput) -> Result<usize> {
        let mut binding = input
            .parse_sections(|s| parser().parse(s).map_err(|e| anyhow!("Parse failed")))
            .unwrap();

        binding.push(vec![
            parser().parse("[[2]]").unwrap(),
            parser().parse("[[6]]").unwrap(),
        ]);

        let mut inputs = binding.into_iter().flatten().collect::<Vec<Packet>>();
        inputs.sort();

        let div1 = inputs
            .iter()
            .position(|l| *l == parser().parse("[[2]]").unwrap())
            .unwrap()
            + 1;
        let div2 = inputs
            .iter()
            .position(|l| *l == parser().parse("[[6]]").unwrap())
            .unwrap()
            + 1;

        Ok(div1 * div2)
    }
}

#[derive(Debug)]
enum Packet {
    Integer(u32),
    List(Vec<Packet>),
}

impl Eq for Packet {}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(match self {
            Self::Integer(a) => match other {
                Self::Integer(b) => a.partial_cmp(b).unwrap(),
                Self::List(b) => Self::List(vec![Packet::Integer(*a)])
                    .partial_cmp(other)
                    .unwrap(),
            },
            Self::List(a) => match other {
                Self::Integer(b) => self
                    .partial_cmp(&Self::List(vec![Packet::Integer(*b)]))
                    .unwrap(),
                Self::List(b) => {
                    let mut i = 0;

                    while i < a.len() && i < b.len() {
                        match a[i].partial_cmp(&b[i]) {
                            Some(std::cmp::Ordering::Less) => {
                                return Some(std::cmp::Ordering::Less)
                            }
                            Some(std::cmp::Ordering::Greater) => {
                                return Some(std::cmp::Ordering::Greater)
                            }
                            _ => {}
                        };

                        i = i + 1;
                    }

                    a.len().cmp(&b.len())
                }
            },
        })
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Integer(l0), Self::Integer(r0)) => l0 == r0,
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            _ => false,
        }
    }
}

fn parser() -> impl Parser<char, Packet, Error = Simple<char>> {
    let expr = recursive(|expr| {
        let int = text::int(10).map(|s: String| Packet::Integer(s.parse().unwrap()));
        let array = expr
            .clone()
            .chain(just(',').ignore_then(expr.clone()).repeated())
            .or_not()
            .flatten()
            .delimited_by(just('['), just(']'))
            .map(|s| Packet::List(s))
            .labelled("list");

        let atom = array.or(int);

        atom
    });

    expr.then_ignore(end())
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::*;

    fn sample() -> ProblemInput {
        let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

        crate::helpers::input::ProblemInput::from_sample(input)
    }

    #[test]
    fn sample1() {
        assert_eq!(13, Day13::default().part1(&sample()).unwrap())
    }

    #[test]
    fn sample2() {
        assert_eq!(140, Day13::default().part2(&sample()).unwrap())
    }

    #[test]
    fn pair1() {
        let s1: Packet = parser().parse("[1,1,3,1,1]").unwrap();
        let s2: Packet = parser().parse("[1,1,5,1,1]").unwrap();

        assert_eq!(Ordering::Less, s1.partial_cmp(&s2).unwrap());
    }

    #[test]
    fn pair2() {
        let s1: Packet = parser().parse("[[1],[2,3,4]]").unwrap();
        let s2: Packet = parser().parse("[[1],4]").unwrap();

        assert_eq!(Ordering::Less, s1.partial_cmp(&s2).unwrap());
    }

    #[test]
    fn pair3() {
        let s1: Packet = parser().parse("[9]").unwrap();
        let s2: Packet = parser().parse("[[8,7,6]]").unwrap();

        assert_ne!(Ordering::Less, s1.partial_cmp(&s2).unwrap());
    }

    #[test]
    fn pair4() {
        let s1: Packet = parser().parse("[[4,4],4,4]").unwrap();
        let s2: Packet = parser().parse("[[4,4],4,4,4]").unwrap();

        assert_eq!(Ordering::Less, s1.partial_cmp(&s2).unwrap());
    }

    #[test]
    fn pair5() {
        let s1: Packet = parser().parse("[7,7,7,7]").unwrap();
        let s2: Packet = parser().parse("[7,7,7]").unwrap();

        assert_ne!(Ordering::Less, s1.partial_cmp(&s2).unwrap());
    }

    #[test]
    fn pair6() {
        let s1: Packet = parser().parse("[]").unwrap();
        let s2: Packet = parser().parse("[3]").unwrap();

        assert_eq!(Ordering::Less, s1.partial_cmp(&s2).unwrap());
    }

    #[test]
    fn pair7() {
        let s1: Packet = parser().parse("[[[]]]").unwrap();
        let s2: Packet = parser().parse("[[]]").unwrap();

        assert_ne!(Ordering::Less, s1.partial_cmp(&s2).unwrap());
    }

    #[test]
    fn pair8() {
        let s1: Packet = parser().parse("[1,[2,[3,[4,[5,6,7]]]],8,9]").unwrap();
        let s2: Packet = parser().parse("[1,[2,[3,[4,[5,6,0]]]],8,9]").unwrap();

        assert_ne!(Ordering::Less, s1.partial_cmp(&s2).unwrap());
    }
}
