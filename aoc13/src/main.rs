use anyhow::Result;
use std::io::{self, Error, ErrorKind, Read, Write};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut lines = input.lines();
    let earliest_timestamp = lines
        .next()
        .ok_or_else(|| Error::from(ErrorKind::InvalidData))?
        .parse::<u32>()?;
    let (min_id, min_timestamp) = lines
        .next()
        .ok_or_else(|| Error::from(ErrorKind::InvalidData))?
        .split(',')
        .flat_map(|s| s.parse::<u32>())
        .map(|id| {
            let mut timestamp = id;
            while timestamp < earliest_timestamp {
                timestamp += id;
            }
            (id, timestamp)
        })
        .min_by_key(|x| x.1)
        .ok_or_else(|| Error::from(ErrorKind::InvalidData))?;

    writeln!(
        io::stdout(),
        "{}",
        (min_timestamp - earliest_timestamp) * min_id
    )?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let bus_ids: Vec<(i64, i64)> = input
        .lines()
        .nth(1)
        .ok_or_else(|| Error::from(ErrorKind::InvalidData))?
        .split(',')
        .enumerate()
        .flat_map(|(ix, s)| s.parse::<i64>().map(|n| (n - ix as i64, n)))
        .collect::<Vec<_>>();
    let bus_id_product = bus_ids.iter().map(|n| n.1).product::<i64>();

    // https://en.wikipedia.org/wiki/Chinese_remainder_theorem
    // The math is grabbed from https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
    let crt = bus_ids
        .iter()
        .map(|&(residue, modulus)| {
            let p = bus_id_product / modulus;
            residue * ((egcd(p, modulus).1 % modulus + modulus) % modulus) * p
        })
        .sum::<i64>()
        % bus_id_product;

    writeln!(io::stdout(), "{}", crt)?;
    Ok(())
}

// https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm
// https://en.wikipedia.org/wiki/B%C3%A9zout%27s_identity
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    match b {
        0 => (a.abs(), a.signum(), 0),
        _ => {
            let (d, coef_b, coef_a) = egcd(b, a % b);
            (d, coef_a, coef_b - coef_a * (a / b))
        }
    }
}
