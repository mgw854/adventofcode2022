mod helpers;
mod problem;
mod year2022;

use crate::year2022::day6;
use anyhow::Result;
use helpers::input::ProblemInput;

use crate::problem::Problem;

fn main() -> Result<()> {
    let input = ProblemInput::load(2022, 6)?;
    let problem = day6::Day6::default();

    println!("{}", "The answer to part 1 is:");
    println!("{}", problem.part1(&input)?);

    println!("{}", "The answer to part 2 is:");
    println!("{}", problem.part2(&input)?);

    Ok(())
}
