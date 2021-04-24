use anyhow::Result;
use std::collections::HashSet;
use std::io::{self, Error, ErrorKind, Read, Write};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    writeln!(
        io::stdout(),
        "{}",
        get_data(input)
            .max()
            .ok_or_else(|| Error::from(ErrorKind::InvalidData))?
    )?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let all_passes: HashSet<_> = (0..(2_u32.pow(10))).collect();
    let nearby_passes: HashSet<_> = get_data(input).collect();
    let nearby_passes_min = nearby_passes
        .iter()
        .min()
        .ok_or_else(|| Error::from(ErrorKind::InvalidData))?;
    let nearby_passes_max = nearby_passes
        .iter()
        .max()
        .ok_or_else(|| Error::from(ErrorKind::InvalidData))?;
    let my_seat = all_passes
        .difference(&nearby_passes)
        .find(|&x| x > nearby_passes_min && x < nearby_passes_max)
        .ok_or_else(|| Error::from(ErrorKind::InvalidData))?;

    writeln!(io::stdout(), "{}", my_seat)?;
    Ok(())
}

fn get_data(input: &str) -> impl Iterator<Item = u32> + '_ {
    input.lines().flat_map(|l| get_id(l))
}

fn get_id(line: &str) -> Result<u32> {
    let row = &line
        .get(0..7)
        .ok_or_else(|| Error::from(ErrorKind::InvalidData))?
        .replace("F", "0")
        .replace("B", "1");
    let column = &line
        .get(7..10)
        .ok_or_else(|| Error::from(ErrorKind::InvalidData))?
        .replace("L", "0")
        .replace("R", "1");
    Ok(u32::from_str_radix(row, 2)? * 8 + u32::from_str_radix(column, 2)?)
}
