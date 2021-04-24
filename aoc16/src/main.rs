use anyhow::{bail, Result};
use std::collections::{HashMap, HashSet};
use std::io::{self, Error, ErrorKind, Read, Write};
use std::ops::RangeInclusive;
use std::str::FromStr;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let (named_ranges, _, nearby_tickets) = get_data(input)?;

    writeln!(
        io::stdout(),
        "{}",
        nearby_tickets
            .into_iter()
            .flatten()
            .filter(|v| {
                !named_ranges
                    .iter()
                    .any(|nr| nr.ranges[0].contains(v) || nr.ranges[1].contains(v))
            })
            .sum::<u32>()
    )?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let (named_ranges, my_ticket, nearby_tickets) = get_data(input)?;

    let good_tickets = nearby_tickets
        .into_iter()
        .filter(|vc| {
            vc.iter().all(|v| {
                named_ranges
                    .iter()
                    .any(|nr| nr.ranges[0].contains(v) || nr.ranges[1].contains(v))
            })
        })
        .chain(std::iter::once(my_ticket.clone()))
        .collect::<Vec<_>>();

    let mut possible_transcripts: HashMap<usize, HashSet<String>> = HashMap::new();

    if !good_tickets.iter().all(|gt| gt.len() == 20) {
        bail!("Invalid input");
    }

    for i in 0_usize..20_usize {
        for range in named_ranges.iter() {
            let found = good_tickets
                .iter()
                // We have checked the lengths of the vectors, so we may use the index
                .all(|t| range.ranges[0].contains(&t[i]) || range.ranges[1].contains(&t[i]));

            if found {
                let v = possible_transcripts.entry(i).or_insert_with(HashSet::new);
                v.insert(range.name.clone());
            }
        }
    }

    let mut possible_transcripts = possible_transcripts
        .into_iter()
        .map(|(k, v)| (k, v))
        .collect::<Vec<_>>();

    possible_transcripts.sort_unstable_by_key(|x| x.1.len());

    let mut transcript: HashMap<String, usize> = HashMap::new();
    let mut found: HashSet<String> = HashSet::new();

    for (ix, current_hs) in possible_transcripts.into_iter() {
        let diff = current_hs.difference(&found).cloned().collect::<Vec<_>>();

        if diff.len() != 1 {
            bail!("Can't find transcript");
        }

        let diff = diff
            .into_iter()
            .next()
            .ok_or_else(|| Error::from(ErrorKind::InvalidData))?;
        transcript.insert(diff.clone(), ix);
        found.insert(diff);
    }

    writeln!(
        io::stdout(),
        "{}",
        transcript
            .into_iter()
            .filter(|(k, _)| k.starts_with("departure"))
            .map(|(_, v)| v)
            .map(|i| my_ticket[i] as u64)
            .product::<u64>()
    )?;
    Ok(())
}

#[derive(Debug)]
struct NamedRange {
    name: String,
    ranges: [RangeInclusive<u32>; 2],
}

impl FromStr for NamedRange {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut splitted = value.split(':');
        let name = splitted
            .next()
            .ok_or_else(|| Error::from(ErrorKind::InvalidData))?
            .to_string();
        let mut ranges_iter = splitted
            .next()
            .ok_or_else(|| Error::from(ErrorKind::InvalidData))?
            .split(|c: char| !c.is_ascii_digit())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse());
        Ok(NamedRange {
            name,
            ranges: [
                RangeInclusive::new(
                    ranges_iter
                        .next()
                        .ok_or_else(|| Error::from(ErrorKind::InvalidData))??,
                    ranges_iter
                        .next()
                        .ok_or_else(|| Error::from(ErrorKind::InvalidData))??,
                ),
                RangeInclusive::new(
                    ranges_iter
                        .next()
                        .ok_or_else(|| Error::from(ErrorKind::InvalidData))??,
                    ranges_iter
                        .next()
                        .ok_or_else(|| Error::from(ErrorKind::InvalidData))??,
                ),
            ],
        })
    }
}

type Data = Result<(Vec<NamedRange>, Vec<u32>, Vec<Vec<u32>>)>;

fn get_data(input: &str) -> Data {
    let mut input_iter = input.split("\r\n\r\n");

    let named_ranges = input_iter
        .next()
        .ok_or_else(|| Error::from(ErrorKind::InvalidData))?
        .lines()
        .map(|s| NamedRange::from_str(s))
        .collect::<Result<Vec<_>>>()?;

    let parse_ticket = |s: &str| {
        s.split(',')
            .map(|t| t.parse())
            .collect::<Result<Vec<_>, _>>()
    };

    let my_ticket = get_tickets(&mut input_iter)
        .map(parse_ticket)
        .next()
        .ok_or_else(|| Error::from(ErrorKind::InvalidData))??;

    let nearby_tickets = get_tickets(&mut input_iter)
        .map(parse_ticket)
        .collect::<Result<_, _>>()?;

    Ok((named_ranges, my_ticket, nearby_tickets))
}

fn get_tickets<'a>(
    input_iter: &mut impl Iterator<Item = &'a str>,
) -> impl Iterator<Item = &'a str> {
    // We just substitute an empty str, if the input is bad
    input_iter.next().unwrap_or_default().lines().skip(1)
}
