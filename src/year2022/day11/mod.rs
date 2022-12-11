use std::collections::HashMap;

use crate::{helpers::input::ProblemInput, problem::Problem};

use anyhow::{anyhow, Result};

pub struct Day11 {}

impl Default for Day11 {
    fn default() -> Self {
        Self {}
    }
}

impl Problem<usize, usize> for Day11 {
    fn part1(&self, input: &ProblemInput) -> Result<usize> {
        let monkeys = vec![
            Monkey { items: vec![59,65,86,56,74,57,56], operation: |x| x*17, test: |x| if x % 3 == 0 { 3 } else { 6 }, inspections: 0 },
            Monkey { items: vec![63, 83, 50, 63, 56], operation: |x| x+2, test: |x| if x % 13 == 0 { 3 } else { 0 }, inspections: 0 },
            Monkey { items: vec![93, 79, 74, 55], operation: |x| x+1, test: |x| if x % 2 == 0 { 0 } else { 1 }, inspections: 0 },
            Monkey { items: vec![86, 61, 67, 88, 94, 69, 56, 91], operation: |x| x+7, test: |x| if x % 11 == 0 { 6 } else { 7 }, inspections: 0 },
            Monkey { items: vec![76, 50, 51], operation: |x| x*x, test: |x| if x % 19 == 0 { 2 } else { 5 }, inspections: 0 },
            Monkey { items: vec![ 77, 76], operation: |x| x+8, test: |x| if x % 17 == 0 { 2 } else { 1 }, inspections: 0 },
            Monkey { items: vec![74], operation: |x| x*2, test: |x| if x % 5 == 0 { 4 } else { 7 }, inspections: 0 },
            Monkey { items: vec![86, 85, 52, 86, 91, 95], operation: |x| x+6, test: |x| if x % 7 == 0 { 4 } else { 5 }, inspections: 0 },
            ];
        let answer = monkey_turns(monkeys);
        Ok(answer)
    }

    fn part2(&self, input: &ProblemInput) -> Result<usize> {
        let monkeys = vec![
            Monkey { items: vec![59,65,86,56,74,57,56], operation: |x| x*17, test: |x| if x % 3 == 0 { 3 } else { 6 }, inspections: 0 },
            Monkey { items: vec![63, 83, 50, 63, 56], operation: |x| x+2, test: |x| if x % 13 == 0 { 3 } else { 0 }, inspections: 0 },
            Monkey { items: vec![93, 79, 74, 55], operation: |x| x+1, test: |x| if x % 2 == 0 { 0 } else { 1 }, inspections: 0 },
            Monkey { items: vec![86, 61, 67, 88, 94, 69, 56, 91], operation: |x| x+7, test: |x| if x % 11 == 0 { 6 } else { 7 }, inspections: 0 },
            Monkey { items: vec![76, 50, 51], operation: |x| x*x, test: |x| if x % 19 == 0 { 2 } else { 5 }, inspections: 0 },
            Monkey { items: vec![ 77, 76], operation: |x| x+8, test: |x| if x % 17 == 0 { 2 } else { 1 }, inspections: 0 },
            Monkey { items: vec![74], operation: |x| x*2, test: |x| if x % 5 == 0 { 4 } else { 7 }, inspections: 0 },
            Monkey { items: vec![86, 85, 52, 86, 91, 95], operation: |x| x+6, test: |x| if x % 7 == 0 { 4 } else { 5 }, inspections: 0 },
            ];
        let answer = monkey_turns2(monkeys, 3*13*2*11*19*17*5*7);
        Ok(answer)
    }
}

struct Monkey {
    items: Vec<usize>,
    operation: fn(usize) -> usize,
    test: fn(usize) -> usize,
    inspections: usize
}

impl Monkey {
    fn take_turn(&mut self) -> HashMap<usize, Vec<usize>> {
        let mut allocations = HashMap::new();

        for item in self.items.iter() {
            let worry = (self.operation)(*item) / 3;
            allocations.entry((self.test)(worry)).or_insert(Vec::new()).push(worry);
            self.inspections += 1;
        }

        self.items.clear();

        allocations
    }

    fn take_anxious_turn(&mut self, divisor: usize) -> HashMap<usize, Vec<usize>> {
        let mut allocations = HashMap::new();

        for item in self.items.iter() {
            let worry = (self.operation)(*item) % divisor;
            allocations.entry((self.test)(worry)).or_insert(Vec::new()).push(worry);
            self.inspections += 1;
        }

        self.items.clear();

        allocations
    }
}

fn monkey_turns(mut monkeys: Vec<Monkey>) -> usize {
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            let thrown = monkey.take_turn();

            for entry in thrown {
                for item in entry.1 {
                    monkeys[entry.0].items.push(item);
                }
            }
        }
    }

    let mut business = monkeys.iter().map(|m| m.inspections).collect::<Vec<usize>>();
    business.sort_by(|a, b| b.cmp(a));
    business[0] * business[1]
}



fn monkey_turns2(mut monkeys: Vec<Monkey>, divisor: usize) -> usize {
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            let thrown = monkey.take_anxious_turn(divisor);

            for entry in thrown {
                for item in entry.1 {
                    monkeys[entry.0].items.push(item);
                }
            }
        }
    }

    let mut business = monkeys.iter().map(|m| m.inspections).collect::<Vec<usize>>();
    business.sort_by(|a, b| b.cmp(a));
    business[0] * business[1]
}


#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> ProblemInput {
        let input = "";

        crate::helpers::input::ProblemInput::from_sample(input)
    }

    #[test]
       fn monkey_turns_part1() {
        let monkeys = vec![
            Monkey { items: vec![79,98], operation: |x| x*19, test: |x| if x % 23 == 0 { 2 } else { 3 }, inspections: 0 },
            Monkey { items: vec![54,65,75,74], operation: |x| x+6, test: |x| if x % 19 == 0 { 2 } else { 0 }, inspections: 0 },
            Monkey { items: vec![79,60,97], operation: |x| x*x, test: |x| if x % 13 == 0 { 1 } else { 3 }, inspections: 0 },
            Monkey { items: vec![74], operation: |x| x+3, test: |x| if x % 17 == 0 { 0 } else { 1 }, inspections: 0 },
        ];
        let answer = monkey_turns(monkeys);
        assert_eq!(10605, answer);
    }

    #[test]
       fn monkey_turns_part2() {
        let monkeys = vec![
            Monkey { items: vec![79,98], operation: |x| x*19, test: |x| if x % 23 == 0 { 2 } else { 3 }, inspections: 0 },
            Monkey { items: vec![54,65,75,74], operation: |x| x+6, test: |x| if x % 19 == 0 { 2 } else { 0 }, inspections: 0 },
            Monkey { items: vec![79,60,97], operation: |x| x*x, test: |x| if x % 13 == 0 { 1 } else { 3 }, inspections: 0 },
            Monkey { items: vec![74], operation: |x| x+3, test: |x| if x % 17 == 0 { 0 } else { 1 }, inspections: 0 },
        ];
        let answer = monkey_turns2(monkeys, 23*19*13*17);
        assert_eq!(2713310158, answer);
    }
}