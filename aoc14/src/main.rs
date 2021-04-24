use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;
use std::io::{self, Error, ErrorKind, Read, Write};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    process(input, false)
}

fn part2(input: &str) -> Result<()> {
    process(input, true)
}

enum VariableMask {
    Zero(u64),
    Floating(Vec<u64>),
}

fn process(input: &str, floating: bool) -> Result<()> {
    let mut current_one_mask = 0;
    let mut current_variable_mask = match floating {
        false => VariableMask::Zero(0),
        true => VariableMask::Floating(vec![]),
    };

    let mut memory = HashMap::new();

    for line in input.lines() {
        match line.starts_with("mask") {
            true => {
                let mask = line
                    .get(7..)
                    .ok_or_else(|| Error::from(ErrorKind::InvalidData))?;
                current_one_mask = u64::from_str_radix(&mask.replace("X", "0"), 2)?;
                match floating {
                    false => {
                        current_variable_mask =
                            VariableMask::Zero(u64::from_str_radix(&mask.replace("X", "1"), 2)?)
                    }
                    true => {
                        let current_floating_mask = mask.replace("1", "0").replace("X", "1");
                        // Ideally, this should be solved using tricky bitwise logic, but here it's just string manipulation
                        current_variable_mask = VariableMask::Floating(
                            (0..current_floating_mask.len())
                                .map(|i| b'0'..=current_floating_mask.as_bytes()[i])
                                .multi_cartesian_product()
                                .flat_map(String::from_utf8)
                                .flat_map(|v| u64::from_str_radix(&v, 2))
                                .collect::<Vec<_>>(),
                        );
                    }
                }
            }
            false => {
                let mut number_iter = line
                    .split(|c: char| !c.is_ascii_digit())
                    .filter(|s| !s.is_empty())
                    .map(|s| s.parse::<u64>());
                let key = number_iter
                    .next()
                    .ok_or_else(|| Error::from(ErrorKind::InvalidData))??;
                let value = number_iter
                    .next()
                    .ok_or_else(|| Error::from(ErrorKind::InvalidData))??;

                match current_variable_mask {
                    VariableMask::Zero(current_zero_mask) => {
                        *memory.entry(key).or_insert(0) =
                            value & current_zero_mask | current_one_mask;
                    }
                    VariableMask::Floating(ref current_floating_masks) => {
                        for floating_mask in current_floating_masks.iter() {
                            *memory
                                .entry((key | current_one_mask) ^ floating_mask)
                                .or_insert(0) = value;
                        }
                    }
                }
            }
        }
    }

    writeln!(io::stdout(), "{}", memory.values().sum::<u64>())?;
    Ok(())
}
