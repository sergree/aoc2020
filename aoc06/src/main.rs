use anyhow::Result;
use std::collections::HashSet;
use std::io::{self, Read, Write};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let unique_chars: Vec<HashSet<_>> = get_data(input)
        .map(|g| g.lines().flat_map(|s| s.chars()).collect())
        .collect();

    print_len_sum(unique_chars)
}

fn part2(input: &str) -> Result<()> {
    let groups: Vec<Vec<HashSet<_>>> = get_data(input)
        .map(|g| g.lines().map(|l| l.chars().collect()).collect())
        .collect();

    let group_intersections: Vec<HashSet<_>> = groups
        .into_iter()
        .map(|vhs| {
            let mut vhs_iter = vhs.into_iter();
            let first = vhs_iter.next().unwrap_or_default();
            vhs_iter.fold(first, |acc, x| acc.intersection(&x).cloned().collect())
        })
        .collect();

    print_len_sum(group_intersections)
}

fn get_data(input: &str) -> impl Iterator<Item = &str> {
    input.split("\r\n\r\n")
}

fn print_len_sum<T>(vhs: Vec<HashSet<T>>) -> Result<()> {
    writeln!(
        io::stdout(),
        "{}",
        vhs.into_iter().map(|hs| hs.len()).sum::<usize>()
    )?;
    Ok(())
}
