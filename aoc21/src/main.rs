use anyhow::Result;
use std::collections::{HashMap, HashSet};
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
    let (foods, detected) = get_data(input)?;

    writeln!(
        io::stdout(),
        "{}",
        foods
            .iter()
            .map(|f| f
                .ingredients
                .iter()
                .filter(|&i| !detected.contains_key(i))
                .count())
            .sum::<usize>()
    )?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let (_, detected) = get_data(input)?;
    let mut detected = detected.into_iter().collect::<Vec<_>>();
    detected.sort_unstable_by_key(|e| e.1.to_string());

    writeln!(
        io::stdout(),
        "{}",
        detected
            .into_iter()
            .map(|e| e.0)
            .collect::<Vec<_>>()
            .join(",")
    )?;
    Ok(())
}

fn get_data(input: &str) -> Result<(Vec<Food>, HashMap<String, String>)> {
    let foods: Vec<_> = input.lines().map(Food::from_str).collect::<Result<_>>()?;

    let allergens: HashSet<_> = foods
        .iter()
        .flat_map(|v| v.allergens.iter().map(|s| s.as_str()))
        .collect();

    let mut detected = HashMap::new();

    while detected.len() != allergens.len() {
        for &allergen in allergens.iter() {
            if detected.contains_key(allergen) {
                continue;
            }

            let intersect = foods
                .iter()
                .filter(|f| f.allergens.contains(allergen))
                .map(|f| {
                    let mut current_ingredients = f.ingredients.clone();

                    for ingredient in detected.keys() {
                        current_ingredients.remove(ingredient);
                    }

                    current_ingredients
                })
                .reduce(|acc, f| acc.intersection(&f).cloned().collect());

            if let Some(intersect) = intersect {
                match intersect.len() {
                    len if len == 1 => {
                        detected.insert(
                            // We may call .unwrap() here because we checked the length above
                            intersect.iter().cloned().next().unwrap(),
                            allergen.to_string(),
                        );
                    }
                    _ => {}
                }
            }
        }
    }

    Ok((foods, detected))
}

#[derive(Debug)]
struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

impl FromStr for Food {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut value_iter = value.split(" (contains ");
        let ingredients_str = value_iter
            .next()
            .ok_or_else(|| Error::from(ErrorKind::InvalidData))?;
        let allergens_str = value_iter
            .next()
            .ok_or_else(|| Error::from(ErrorKind::InvalidData))?;

        Ok(Food {
            ingredients: ingredients_str
                .split_ascii_whitespace()
                .map(|s| s.to_string())
                .collect(),
            allergens: allergens_str
                .split(", ")
                .map(|s| s.trim_end_matches(')').to_string())
                .collect(),
        })
    }
}
