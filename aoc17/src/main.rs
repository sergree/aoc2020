//! Generic over dimension solution
//! Requires stable Rust 1.51+

use anyhow::{bail, Result};
use itertools::Itertools;
use std::collections::HashSet;
use std::io::{self, Read, Write};
use std::str::FromStr;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut world = World::<3>::from_str(input)?;
    world.cycles(6);
    writeln!(io::stdout(), "{}", world.active_count())?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut world = World::<4>::from_str(input)?;
    world.cycles(6);
    writeln!(io::stdout(), "{}", world.active_count())?;
    Ok(())
}

struct World<const DIM: usize> {
    cubes: HashSet<Cube<DIM>>,
    offsets: Vec<Vec<i32>>,
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Cube<const DIM: usize> {
    coords: [i32; DIM],
}

impl<const DIM: usize> Cube<DIM> {
    fn new(coords: [i32; DIM]) -> Self {
        Cube { coords }
    }
}

impl<const DIM: usize> FromStr for World<DIM> {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if DIM < 2 {
            bail!("There must be at least 2 dimensions");
        }

        let mut cubes = HashSet::new();

        for (y, row) in value.lines().enumerate() {
            for (x, char) in row.chars().enumerate() {
                if char == '#' {
                    let mut values = [0; DIM];
                    // We may use the index because we checked the length above
                    values[0] = x as i32;
                    values[1] = y as i32;
                    cubes.insert(Cube::new(values));
                }
            }
        }

        let offsets = (0..DIM)
            .map(|_| -1..=1)
            .multi_cartesian_product()
            .filter(|v| !v.iter().all(|n| *n == 0))
            .collect();

        Ok(World { cubes, offsets })
    }
}

impl<const DIM: usize> World<DIM> {
    fn cycle(&mut self) {
        let mut new_cubes = HashSet::new();

        let mut lower_bounds = [0; DIM];
        let mut upper_bounds = [0; DIM];

        if self.cubes.is_empty() {
            return;
        }

        (0..DIM).for_each(|i| {
            // We may call .unwrap() here because we checked the length above
            lower_bounds[i] = self.cubes.iter().map(|c| c.coords[i]).min().unwrap() - 1;
            upper_bounds[i] = self.cubes.iter().map(|c| c.coords[i]).max().unwrap() + 1;
        });

        (0..DIM)
            .map(|i| lower_bounds[i]..=upper_bounds[i])
            .multi_cartesian_product()
            .for_each(|values_vec| {
                let mut values = [0; DIM];
                (0..DIM).for_each(|j| values[j] = values_vec[j]);

                let cube = Cube::new(values);

                let active = self.cubes.contains(&cube);
                let active_neighbors = self.active_neighbors(&cube);

                match (active, active_neighbors) {
                    (true, 2..=3) | (false, 3) => {
                        new_cubes.insert(cube);
                    }
                    _ => {}
                }
            });

        self.cubes = new_cubes;
    }

    fn cycles(&mut self, count: usize) {
        (0..count).for_each(|_| self.cycle());
    }

    fn active_neighbors(&self, cube: &Cube<DIM>) -> usize {
        self.offsets
            .iter()
            .filter(|o| {
                let mut values = [0; DIM];
                (0..DIM).for_each(|i| values[i] = cube.coords[i] + o[i]);
                self.cubes.contains(&Cube::new(values))
            })
            .count()
    }

    fn active_count(&self) -> usize {
        self.cubes.iter().count()
    }
}
