use anyhow::Result;
use std::io::{self, Error, ErrorKind, Read, Write};

const DIVISOR: u64 = 20201227;
const SUBJECT_NUMBER: u64 = 7;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut input_iter = input.lines().map(|l| l.parse());

    let card_pub_key = input_iter
        .next()
        .ok_or_else(|| Error::from(ErrorKind::InvalidData))??;
    let door_pub_key = input_iter
        .next()
        .ok_or_else(|| Error::from(ErrorKind::InvalidData))??;

    let mut value = 1;
    let mut loop_size = 0;
    while value != card_pub_key {
        value = value * SUBJECT_NUMBER % DIVISOR;
        loop_size += 1;
    }

    writeln!(
        io::stdout(),
        "{}",
        mod_pow(door_pub_key, loop_size, DIVISOR)
    )?;
    Ok(())
}

/// https://en.wikipedia.org/wiki/Modular_exponentiation
fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp >>= 1;
        base = base * base % modulus
    }
    result
}
