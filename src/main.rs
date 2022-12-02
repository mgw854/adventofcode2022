mod year2015;
mod helpers;
mod problem;

use anyhow::Result;
use helpers::input::ProblemInput;
use crate::year2015::day1;

use crate::problem::Problem;

fn main() -> Result<()> {
    let input = ProblemInput::load(1)?;
    let problem = day1::Day1::default();

    println!("{}", "The answer to part 1 is:");
    println!("{}", problem.part1(&input)?);

    println!("{}", "The answer to part 2 is:");
    println!("{}", problem.part2(&input)?);

    Ok(())
}
