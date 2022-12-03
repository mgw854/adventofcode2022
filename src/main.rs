mod day3;
mod helpers;
mod problem;

use anyhow::Result;
use helpers::input::ProblemInput;

use crate::problem::Problem;

fn main() -> Result<()> {
    let input = ProblemInput::load(3)?;
    let problem = day3::Day3::default();

    println!("{}", "The answer to part 1 is:");
    println!("{}", problem.part1(&input)?);

    println!("{}", "The answer to part 2 is:");
    println!("{}", problem.part2(&input)?);

    Ok(())
}
