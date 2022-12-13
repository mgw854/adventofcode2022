mod helpers;
mod problem;
mod year2022;

use crate::year2022::day13;
use anyhow::Result;
use helpers::input::ProblemInput;

use crate::problem::Problem;

fn main() -> Result<()> {
    let input = ProblemInput::load(2022, 13)?;
    let problem = day13::Day13::default();

    println!("{}", "The answer to part 1 is:");
    println!("{}", problem.part1(&input)?);

    println!("{}", "The answer to part 2 is:");
    println!("{}", problem.part2(&input)?);

    Ok(())
}
