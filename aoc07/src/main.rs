use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::io::{self, Error, ErrorKind, Read, Write};
use std::str::FromStr;

const MY_BAG: &str = "shiny gold";

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let bags = get_data(input)?;

    let mut should_contain = HashSet::new();
    should_contain.insert(MY_BAG.to_string());

    loop {
        let mut inserted = false;

        for bag in bags.iter() {
            let mut need_to_insert = false;

            for can_contain in bag.contains.iter() {
                if !should_contain.contains(&bag.name) && should_contain.contains(can_contain) {
                    need_to_insert = true;
                    break;
                }
            }

            if need_to_insert {
                should_contain.insert(bag.name.clone());
                inserted = true;
            }
        }

        if !inserted {
            break;
        }
    }

    writeln!(io::stdout(), "{}", should_contain.len() - 1)?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let bags: HashMap<_, _> = get_data(input)?
        .into_iter()
        .map(|b| (b.name, b.contains))
        .collect();

    writeln!(io::stdout(), "{}", bag_count(&bags, MY_BAG)?)?;
    Ok(())
}

fn bag_count(hm: &HashMap<String, Vec<String>>, name: &str) -> Result<usize> {
    let contains = hm
        .get(name)
        .ok_or_else(|| Error::from(ErrorKind::InvalidData))?;

    let mut count = 0;

    for name in contains {
        count += 1;
        count += bag_count(hm, name)?;
    }

    Ok(count)
}

// We should rather use &str here to reduce allocs, but I decided to go the easy way
#[derive(Debug)]
struct Bag {
    name: String,
    contains: Vec<String>,
}

impl FromStr for Bag {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut line_iter = line.split(" bags contain ");
        let name = line_iter
            .next()
            .ok_or_else(|| Error::from(ErrorKind::InvalidData))?
            .to_string();
        let rest = line_iter
            .next()
            .ok_or_else(|| Error::from(ErrorKind::InvalidData))?;

        let contains = match rest {
            "no other bags." => vec![],
            _ => {
                let mut names = vec![];

                for content in rest.split(", ") {
                    let mut content_iter = content.split_ascii_whitespace();
                    let count = content_iter
                        .next()
                        .ok_or_else(|| Error::from(ErrorKind::InvalidData))?
                        .parse()?;
                    let name_part_1 = content_iter
                        .next()
                        .ok_or_else(|| Error::from(ErrorKind::InvalidData))?;
                    let name_part_2 = content_iter
                        .next()
                        .ok_or_else(|| Error::from(ErrorKind::InvalidData))?;

                    for _ in 0..count {
                        names.push(format!("{} {}", name_part_1, name_part_2))
                    }
                }

                names
            }
        };

        Ok(Bag { name, contains })
    }
}

fn get_data(input: &str) -> Result<Vec<Bag>> {
    input
        .lines()
        .map(|l| Bag::from_str(l))
        .collect::<Result<Vec<_>>>()
}
