use anyhow::Result;
use itertools::Itertools;
use std::io::{self, Read, Write};

const YEAR: u32 = 2020;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    process(input, 2)
}

fn part2(input: &str) -> Result<()> {
    process(input, 3)
}

fn process(input: &str, entry_count: usize) -> Result<()> {
    let mut numbers = input
        .lines()
        .map(|l| l.parse::<u32>())
        .collect::<std::result::Result<Vec<_>, _>>()?;

    // Significantly speeds up the search
    numbers.sort_unstable();

    for combination in numbers.into_iter().combinations(entry_count) {
        if combination.iter().sum::<u32>() == YEAR {
            writeln!(io::stdout(), "{}", combination.into_iter().product::<u32>())?;
            break;
        }
    }

    Ok(())
}
