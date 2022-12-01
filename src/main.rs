mod day0;
mod helpers;
mod problem;

use anyhow::Result;
use helpers::input::get_input;

use crate::problem::Problem;

fn main() -> Result<()> {
    let input = get_input(0)?;
    let problem = day0::Day0::default();

    println!("{}", "The answer to part 1 is:");
    println!("{}", problem.part1(&input)?);

    println!("{}", "The answer to part 2 is:");
    println!("{}", problem.part2(&input)?);

    Ok(())
}
