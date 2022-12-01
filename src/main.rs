mod day1;
mod helpers;
mod problem;

use anyhow::Result;
use helpers::input::get_input;

use crate::problem::Problem;

fn main() -> Result<()> {
    let input = get_input(1)?;
    let problem = day1::Day1::default();

    println!("{}", "The answer to part 1 is:");
    println!("{}", problem.part1(&input)?);

    println!("{}", "The answer to part 2 is:");
    println!("{}", problem.part2(&input)?);

    Ok(())
}
