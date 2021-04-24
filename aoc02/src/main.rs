#[macro_use]
extern crate lazy_static;

use anyhow::Result;
use regex::Regex;
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
    process(input, is_valid_part1)
}

fn part2(input: &str) -> Result<()> {
    process(input, is_valid_part2)
}

fn process(input: &str, predicate: impl Fn(&Entry) -> Result<bool>) -> Result<()> {
    let entries = get_entries(input)?;
    writeln!(
        io::stdout(),
        "{}",
        entries
            .into_iter()
            // We just skip invalid entries, if any, without panicking
            .filter(|e| predicate(&e).unwrap_or(false))
            .count()
    )?;
    Ok(())
}

// I allow this here because the output type of the two functions must match
#[allow(clippy::unnecessary_wraps)]
fn is_valid_part1(entry: &Entry) -> Result<bool> {
    Ok(::std::ops::RangeInclusive::new(entry.min, entry.max)
        .contains(&entry.password.chars().filter(|&c| c == entry.char).count()))
}

fn is_valid_part2(entry: &Entry) -> Result<bool> {
    let mut password_chars = entry.password.chars();
    let char_1 = password_chars
        .nth(entry.min - 1)
        .ok_or_else(|| Error::from(ErrorKind::InvalidData))?;
    let char_2 = password_chars
        .nth(entry.max - entry.min - 1)
        .ok_or_else(|| Error::from(ErrorKind::InvalidData))?;
    Ok((char_1 == entry.char) ^ (char_2 == entry.char))
}

fn get_entries(input: &str) -> Result<Vec<Entry>> {
    input.lines().map(|l| Entry::from_str(l)).collect()
}

struct Entry {
    min: usize,
    max: usize,
    char: char,
    password: String,
}

impl FromStr for Entry {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            // We may .unwrap() here, because this regex is correct
            static ref RE: Regex = Regex::new(r"(\d{1,2})-(\d{1,2}) (\w): (\w{1,20})").unwrap();
        }

        let caps = RE
            .captures(line)
            .ok_or_else(|| Error::from(ErrorKind::InvalidData))?;

        Ok(Entry {
            min: caps[1].parse::<usize>()?,
            max: caps[2].parse::<usize>()?,
            char: caps[3]
                .chars()
                .next()
                .ok_or_else(|| Error::from(ErrorKind::InvalidData))?,
            password: caps[4].to_string(),
        })
    }
}
