use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::cmp::Ordering;
use std::io::{self, Read, Write};

const PREAMBLE: usize = 25;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let part1_result = part1(&input)?;
    part2(&input, part1_result)?;
    Ok(())
}

fn part1(input: &str) -> Result<u64> {
    let numbers = get_data(input)?;

    for (idx, &number) in numbers.iter().enumerate() {
        if idx < PREAMBLE {
            continue;
        }

        // We may use the index because we checked the length
        if numbers[(idx - PREAMBLE)..idx]
            .iter()
            .cloned()
            .combinations(2)
            .map(|v| v.iter().sum::<u64>())
            .all(|x| x != number)
        {
            writeln!(io::stdout(), "{}", number)?;
            return Ok(number);
        }
    }

    Err(anyhow!("Invalid input"))
}

fn part2(input: &str, invalid_number: u64) -> Result<()> {
    let numbers = get_data(input)?;

    'outer: for (idx, number) in numbers.iter().enumerate() {
        // I should have used slicing here to reduce allocs, but I decided to go the easy way
        let mut current_numbers = vec![*number];

        for next_number in numbers.iter().skip(idx + 1) {
            current_numbers.push(*next_number);

            let sum = current_numbers.iter().cloned().sum::<u64>();

            match sum.cmp(&invalid_number) {
                Ordering::Equal => {
                    // We may .unwrap() here, because current_numbers is DEFINITELY not empty
                    let min = *current_numbers.iter().min().unwrap();
                    let max = *current_numbers.iter().max().unwrap();
                    writeln!(io::stdout(), "{}", min + max)?;
                    break 'outer;
                }
                Ordering::Greater => {
                    break;
                }
                _ => {}
            }
        }
    }

    Ok(())
}

fn get_data(input: &str) -> Result<Vec<u64>> {
    let data = input.lines().map(|l| l.parse()).collect::<Result<_, _>>()?;
    Ok(data)
}
