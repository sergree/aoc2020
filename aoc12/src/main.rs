use anyhow::{anyhow, Result};
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
    process(input, Ship::process_part1)
}

fn part2(input: &str) -> Result<()> {
    process(input, Ship::process_part2)
}

fn process(input: &str, process_fn: impl Fn(&mut Ship, Instruction)) -> Result<()> {
    let mut ship = Ship::new();
    for line in input.lines() {
        process_fn(&mut ship, Instruction::from_str(line)?);
    }
    writeln!(io::stdout(), "{}", ship.manhattan_distance())?;
    Ok(())
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn rotate_left(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    fn rotate_right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::West => Direction::North,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
        }
    }
}

struct Ship {
    direction: Direction,
    x: i32,
    y: i32,
    waypoint_x: i32,
    waypoint_y: i32,
}

impl Ship {
    fn new() -> Self {
        Ship {
            direction: Direction::East,
            x: 0,
            y: 0,
            waypoint_x: 10,
            waypoint_y: 1,
        }
    }

    fn step(&mut self, direction: Direction, value: u32) {
        match direction {
            Direction::North => {
                self.y += value as i32;
            }
            Direction::West => {
                self.x -= value as i32;
            }
            Direction::South => {
                self.y -= value as i32;
            }
            Direction::East => {
                self.x += value as i32;
            }
        }
    }

    fn process_part1(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::North(v) => {
                self.step(Direction::North, v);
            }
            Instruction::South(v) => {
                self.step(Direction::South, v);
            }
            Instruction::East(v) => {
                self.step(Direction::East, v);
            }
            Instruction::West(v) => {
                self.step(Direction::West, v);
            }
            Instruction::Forward(v) => {
                self.step(self.direction, v);
            }
            Instruction::Left(v) => {
                let v = v / 90;
                for _ in 0..v {
                    self.direction = self.direction.rotate_left();
                }
            }
            Instruction::Right(v) => {
                let v = v / 90;
                for _ in 0..v {
                    self.direction = self.direction.rotate_right();
                }
            }
        }
    }

    fn process_part2(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::North(v) => {
                self.waypoint_y += v as i32;
            }
            Instruction::South(v) => {
                self.waypoint_y -= v as i32;
            }
            Instruction::East(v) => {
                self.waypoint_x += v as i32;
            }
            Instruction::West(v) => {
                self.waypoint_x -= v as i32;
            }
            Instruction::Left(v) => {
                let v = v / 90;
                for _ in 0..v {
                    let wp_x = self.waypoint_x;
                    self.waypoint_x = -self.waypoint_y;
                    self.waypoint_y = wp_x;
                }
            }
            Instruction::Right(v) => {
                let v = v / 90;
                for _ in 0..v {
                    let wp_x = self.waypoint_x;
                    self.waypoint_x = self.waypoint_y;
                    self.waypoint_y = -wp_x;
                }
            }
            Instruction::Forward(v) => {
                for _ in 0..v {
                    self.x += self.waypoint_x;
                    self.y += self.waypoint_y;
                }
            }
        }
    }

    fn manhattan_distance(&self) -> u32 {
        self.x.abs() as u32 + self.y.abs() as u32
    }
}

enum Instruction {
    North(u32),
    South(u32),
    East(u32),
    West(u32),
    Left(u32),
    Right(u32),
    Forward(u32),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut value_chars = value.chars();
        let action = value_chars
            .next()
            .ok_or_else(|| Error::from(ErrorKind::InvalidData))?;
        let value = value_chars.as_str().parse()?;
        match action {
            'N' => Ok(Instruction::North(value)),
            'S' => Ok(Instruction::South(value)),
            'E' => Ok(Instruction::East(value)),
            'W' => Ok(Instruction::West(value)),
            'L' => Ok(Instruction::Left(value)),
            'R' => Ok(Instruction::Right(value)),
            'F' => Ok(Instruction::Forward(value)),
            _ => Err(anyhow!("Invalid input")),
        }
    }
}
