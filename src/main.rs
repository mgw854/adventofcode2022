mod helpers;
mod problem;
mod year2015;

use crate::year2015::day1;
use anyhow::Result;
use helpers::input::ProblemInput;

use crate::problem::Problem;

fn main() -> Result<()> {
    let input = ProblemInput::load(2015, 1)?;
    let problem = day1::Day1::default();

    println!("{}", "The answer to part 1 is:");
    println!("{}", problem.part1(&input)?);

    println!("{}", "The answer to part 2 is:");
    println!("{}", problem.part2(&input)?);

    Ok(())
}
