use anyhow::{anyhow, Result};
use std::collections::HashSet;
use std::io::{self, Error, ErrorKind, Read, Write};
use std::str::FromStr;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let operations = get_data(input)?;

    let mut visited = HashSet::new();

    let mut accumulator: i32 = 0;
    let mut cursor: i32 = 0;

    loop {
        if visited.contains(&cursor) {
            break;
        }
        visited.insert(cursor);

        match operations
            .get(cursor as usize)
            .ok_or_else(|| Error::from(ErrorKind::InvalidData))?
        {
            Operation::Acc(n) => {
                accumulator += n;
                cursor += 1;
            }
            Operation::Jmp(n) => {
                cursor += *n;
            }
            Operation::Nop(_) => {
                cursor += 1;
            }
        }
    }

    writeln!(io::stdout(), "{}", accumulator)?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let operations = get_data(input)?;

    let mut changed = None;
    let mut change_tries = HashSet::new();

    let mut visited = HashSet::new();

    let mut accumulator: i32 = 0;
    let mut cursor: i32 = 0;

    loop {
        if visited.contains(&cursor) {
            visited.clear();
            accumulator = 0;
            cursor = 0;
            changed = None;
            continue;
        }
        visited.insert(cursor);

        match operations
            .get(cursor as usize)
            .ok_or_else(|| Error::from(ErrorKind::InvalidData))?
        {
            Operation::Acc(n) => {
                accumulator += n;
                cursor += 1;
            }
            Operation::Jmp(n) => {
                if changed.is_none() && !change_tries.contains(&(cursor as usize)) {
                    changed = Some(cursor as usize);
                    change_tries.insert(cursor as usize);
                    cursor += 1;
                } else {
                    cursor += *n;
                }
            }
            Operation::Nop(n) => {
                if changed.is_none() && !change_tries.contains(&(cursor as usize)) {
                    changed = Some(cursor as usize);
                    change_tries.insert(cursor as usize);
                    cursor += *n;
                } else {
                    cursor += 1;
                }
            }
        }

        if cursor as usize == operations.len() {
            break;
        }
    }

    writeln!(io::stdout(), "{}", accumulator)?;
    Ok(())
}

enum Operation {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl FromStr for Operation {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut value_iter = value.split_ascii_whitespace();
        let instruction = value_iter
            .next()
            .ok_or_else(|| Error::from(ErrorKind::InvalidData))?;
        let argument = value_iter
            .next()
            .ok_or_else(|| Error::from(ErrorKind::InvalidData))?
            .trim_start_matches('+')
            .parse()?;
        match instruction {
            "acc" => Ok(Operation::Acc(argument)),
            "jmp" => Ok(Operation::Jmp(argument)),
            "nop" => Ok(Operation::Nop(argument)),
            _ => Err(anyhow!("Invalid input")),
        }
    }
}

fn get_data(input: &str) -> Result<Vec<Operation>> {
    input.lines().map(|l| Operation::from_str(l)).collect()
}
