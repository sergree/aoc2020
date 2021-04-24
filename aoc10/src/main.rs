use anyhow::Result;
use itertools::Itertools;
use std::io::{self, Read, Write};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let diffs = get_diffs(input)?;
    writeln!(
        io::stdout(),
        "{}",
        diffs.iter().filter(|&&d| d == 1).count() * diffs.iter().filter(|&&d| d == 3).count()
    )?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let diffs = get_diffs(input)?;
    let vars = diffs
        .iter()
        .dedup_with_count()
        .filter(|(_, &d)| d == 1)
        .map(|(n, _)| match n {
            // It's basically 2 ^ (n - 1), but we have to consider that there are no 4+ jolt adapters
            // There are only 1-3 jolt adapters
            4 => 7, // Because of this, this 8 becomes a 7
            3 => 4,
            2 => 2,
            _ => 1,
        });
    writeln!(io::stdout(), "{}", vars.product::<usize>())?;
    Ok(())
}

fn get_data(input: &str) -> Result<Vec<u32>> {
    let mut numbers: Vec<_> = input.lines().map(|l| l.parse()).collect::<Result<_, _>>()?;

    // Charging outlet
    numbers.push(0);

    numbers.sort_unstable();

    // Device's built-in adapter
    // The last element is the maximum, because the vector is sorted
    // We may call .unwrap() here because there is at least 1 element in the vector
    let max = numbers.last().copied().unwrap();
    numbers.push(max + 3);

    Ok(numbers)
}

fn get_diffs(input: &str) -> Result<Vec<u32>> {
    let data = get_data(input)?;
    Ok(data.windows(2).map(|w| w[1] - w[0]).collect())
}
