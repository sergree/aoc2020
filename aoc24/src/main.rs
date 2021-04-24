use anyhow::{bail, Result};
use std::collections::HashSet;
use std::io::{self, Error, ErrorKind, Read, Write};
use std::ops::Add;
use std::str::FromStr;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    process(input, 0)
}

fn part2(input: &str) -> Result<()> {
    process(input, 100)
}

fn process(input: &str, days: usize) -> Result<()> {
    let mut blacks = get_blacks(input)?;
    for _ in 0..days {
        blacks = day(blacks);
    }
    writeln!(io::stdout(), "{}", blacks.len())?;
    Ok(())
}

fn get_blacks(input: &str) -> Result<HashSet<AxialCoordinate>> {
    let tiles = get_tiles(input)?;

    let mut blacks: HashSet<AxialCoordinate> = HashSet::new();

    for tile in tiles {
        let axial = tile.axial_coordinate();

        match blacks.contains(&axial) {
            true => blacks.remove(&axial),
            false => blacks.insert(axial),
        };
    }

    Ok(blacks)
}

fn get_tiles(input: &str) -> Result<Vec<Tile>> {
    let tiles = input
        .lines()
        .map(|l| Tile::from_str(l))
        .collect::<Result<_, _>>()?;
    Ok(tiles)
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct AxialCoordinate(i32, i32);

impl Add for AxialCoordinate {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        AxialCoordinate(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Debug)]
struct Tile {
    directions: Vec<Direction>,
}

impl Tile {
    // There are many hexagonal coordinate systems
    // But i like `axial`
    // https://www.redblobgames.com/grids/hexagons/
    fn axial_coordinate(&self) -> AxialCoordinate {
        self.directions
            .iter()
            .fold(AxialCoordinate(0, 0), |acc, cur| cur.axial() + acc)
    }
}

impl FromStr for Tile {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut chars = value.chars();
        let mut directions = vec![];

        while let Some(first) = chars.next() {
            let direction = match first {
                's' | 'n' => {
                    let second = chars
                        .next()
                        .ok_or_else(|| Error::from(ErrorKind::InvalidData))?;
                    match (first, second) {
                        ('n', 'e') => Direction::NorthEast,
                        ('n', 'w') => Direction::NorthWest,
                        ('s', 'e') => Direction::SouthEast,
                        ('s', 'w') => Direction::SouthWest,
                        _ => bail!("Invalid input"),
                    }
                }
                'e' => Direction::East,
                'w' => Direction::West,
                _ => bail!("Invalid input"),
            };
            directions.push(direction);
        }

        Ok(Tile { directions })
    }
}

#[derive(Debug)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

impl Direction {
    fn axial(&self) -> AxialCoordinate {
        match self {
            Direction::East => AxialCoordinate(1, 0),
            Direction::SouthEast => AxialCoordinate(0, 1),
            Direction::SouthWest => AxialCoordinate(-1, 1),
            Direction::West => AxialCoordinate(-1, 0),
            Direction::NorthWest => AxialCoordinate(0, -1),
            Direction::NorthEast => AxialCoordinate(1, -1),
        }
    }
}

fn day(old_blacks: HashSet<AxialCoordinate>) -> HashSet<AxialCoordinate> {
    let mut new_blacks: HashSet<AxialCoordinate> = HashSet::new();

    let processed: HashSet<AxialCoordinate> = HashSet::new();
    for &current_coord in old_blacks.iter() {
        let checklist = [
            current_coord,
            current_coord + Direction::NorthEast.axial(),
            current_coord + Direction::NorthWest.axial(),
            current_coord + Direction::SouthEast.axial(),
            current_coord + Direction::SouthWest.axial(),
            current_coord + Direction::West.axial(),
            current_coord + Direction::East.axial(),
        ];

        for &coord_to_check in checklist.iter() {
            if processed.contains(&coord_to_check) {
                continue;
            }

            let is_black = old_blacks.contains(&coord_to_check);
            let adjacent = [
                coord_to_check + Direction::NorthEast.axial(),
                coord_to_check + Direction::NorthWest.axial(),
                coord_to_check + Direction::SouthEast.axial(),
                coord_to_check + Direction::SouthWest.axial(),
                coord_to_check + Direction::West.axial(),
                coord_to_check + Direction::East.axial(),
            ];
            let adjacent_count = adjacent.iter().filter(|&a| old_blacks.contains(a)).count();

            match is_black {
                true => {
                    if (1..=2).contains(&adjacent_count) {
                        new_blacks.insert(coord_to_check);
                    }
                }
                false => {
                    if adjacent_count == 2 {
                        new_blacks.insert(coord_to_check);
                    }
                }
            }
        }
    }

    new_blacks
}
