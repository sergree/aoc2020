use anyhow::{anyhow, Result};
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
    let cups = game(get_data(input)?, 100);

    writeln!(
        io::stdout(),
        "{}",
        cups.into_iter()
            .skip(1)
            // We may call .unwrap() here because we have verified that the input data is numbers from 1 to 9
            // We could have handled it in some clever way, but I don't see the point
            .map(|d| std::char::from_digit(d as u32, 10).unwrap())
            .collect::<String>()
    )?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let cups = game(
        get_data(input)?.into_iter().chain(10..=1_000_000).collect(),
        10_000_000,
    );

    let mut cups_iter = cups.into_iter().skip(1);
    writeln!(
        io::stdout(),
        "{}",
        cups_iter
            .next()
            .ok_or_else(|| Error::from(ErrorKind::InvalidData))?
            * cups_iter
                .next()
                .ok_or_else(|| Error::from(ErrorKind::InvalidData))?
    )?;
    Ok(())
}

// We may safely use indexes and .unwrap()s in this function because we have checked the input data in get_data()
fn game(mut cups: Vec<usize>, moves: usize) -> Vec<usize> {
    // We use some hand-made vec-based linked list
    // Where each value points to the next value
    let mut cups_linked_list: Vec<usize> = vec![0; cups.len() + 1];

    let mut previous = *cups.iter().last().unwrap();

    for &cup in cups.iter() {
        cups_linked_list[previous] = cup;
        previous = cup;
    }

    let mut current = *cups.get(0).unwrap();
    let max = *cups.iter().max().unwrap();

    for _ in 0..moves {
        let mut pick_up = [0_usize; 3];
        pick_up[0] = cups_linked_list[current];
        pick_up[1] = cups_linked_list[pick_up[0]];
        pick_up[2] = cups_linked_list[pick_up[1]];

        // Close the loop
        cups_linked_list[current] = cups_linked_list[pick_up[2]];

        let mut destination = current - 1;
        loop {
            if destination == 0 {
                destination = max;
            }

            if !pick_up.contains(&destination) {
                break;
            }

            destination -= 1;
        }

        // Reinsert
        cups_linked_list[pick_up[2]] = cups_linked_list[destination];
        cups_linked_list[destination] = pick_up[0];

        // Move current
        current = cups_linked_list[current];
    }

    // Rebuild back
    // 1 should be the first element
    let mut current = 1;
    for cup in &mut cups {
        *cup = current;
        current = cups_linked_list[current];
    }

    cups
}

fn get_data(input: &str) -> Result<Vec<usize>> {
    let data: Vec<_> = input
        .lines()
        .next()
        .ok_or_else(|| Error::from(ErrorKind::InvalidData))?
        .chars()
        .map(|c| {
            c.to_digit(10)
                .map(|d| d as usize)
                .ok_or_else(|| Error::from(ErrorKind::InvalidData))
        })
        .collect::<Result<_, _>>()?;

    // We need to check that the data is numbers from 1 to 9
    let data_hs: HashSet<_> = data.iter().copied().collect();
    let correct_data_hs: HashSet<_> = (1_usize..=9).collect();

    match data_hs == correct_data_hs {
        true => Ok(data),
        false => Err(anyhow!("Invalid input")),
    }
}
