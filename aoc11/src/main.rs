use anyhow::{anyhow, Result};
use std::convert::TryFrom;
use std::io::{self, Error, ErrorKind, Read, Write};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    process(input, false)
}

fn part2(input: &str) -> Result<()> {
    process(input, true)
}

fn process(input: &str, part2: bool) -> Result<()> {
    let mut waiting_area = WaitingArea::new(input)?;

    loop {
        waiting_area.round(part2);

        if !waiting_area.changed() {
            writeln!(io::stdout(), "{}", waiting_area.occupied_count())?;
            break;
        }
    }

    Ok(())
}

#[derive(Debug, PartialEq, Clone)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}

impl TryFrom<char> for Seat {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Seat::Floor),
            'L' => Ok(Seat::Empty),
            '#' => Ok(Seat::Occupied),
            _ => Err(anyhow!("Invalid input")),
        }
    }
}

const OFFSETS: &[(i32, i32)] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Debug)]
struct WaitingArea {
    seats: Vec<Vec<Seat>>,
    height: usize,
    width: usize,
    seats_previous: Vec<Vec<Seat>>,
}

// We may safely use indexes everywhere here, because we check the lengths of the vectors in the constructor
impl WaitingArea {
    fn new(input: &str) -> Result<Self> {
        let seats = input
            .lines()
            .map(|l| {
                l.chars().flat_map(Seat::try_from).fold(vec![], |mut a, s| {
                    a.push(s);
                    a
                })
            })
            .collect::<Vec<_>>();
        let height = seats.len();
        let width = seats
            .get(0)
            .ok_or_else(|| Error::from(ErrorKind::InvalidData))?
            .len();
        // To work safely with indexes, we need to check that the number of characters in each line is equal to each other
        match seats.iter().all(|r| r.len() == width) {
            true => Ok(WaitingArea {
                seats,
                height,
                width,
                seats_previous: vec![],
            }),
            false => Err(anyhow!("Invalid input")),
        }
    }

    fn round(&mut self, part2: bool) {
        self.seats_previous = self.seats.clone();

        for (row_ix, row) in self.seats_previous.iter().enumerate() {
            for (col_ix, seat) in row.iter().enumerate() {
                match seat {
                    Seat::Empty => {
                        let occupied = match part2 {
                            false => self.adjacent_occupied_count(row_ix, col_ix),
                            true => self.remote_occupied_count(row_ix, col_ix),
                        };
                        if occupied == 0 {
                            self.seats[row_ix][col_ix] = Seat::Occupied;
                        }
                    }
                    Seat::Occupied => {
                        let occupied = match part2 {
                            false => self.adjacent_occupied_count(row_ix, col_ix),
                            true => self.remote_occupied_count(row_ix, col_ix),
                        };
                        let occupied_max = if !part2 { 4 } else { 5 };
                        if occupied >= occupied_max {
                            self.seats[row_ix][col_ix] = Seat::Empty;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn adjacent_occupied_count(&self, y: usize, x: usize) -> usize {
        OFFSETS
            .iter()
            .map(|(yo, xo)| (yo + y as i32, xo + x as i32))
            .filter(|&(y, x)| y >= 0 && y < self.height as i32 && x >= 0 && x < self.width as i32)
            .filter(|&(y, x)| self.seats_previous[y as usize][x as usize] == Seat::Occupied)
            .count()
    }

    fn remote_occupied_count(&self, y: usize, x: usize) -> usize {
        let mut count = 0;

        for offset in OFFSETS.iter() {
            let (mut yo, mut xo) = *offset;

            loop {
                let y = y as i32 + yo;
                let x = x as i32 + xo;

                if y >= 0 && y < self.height as i32 && x >= 0 && x < self.width as i32 {
                    match self.seats_previous[y as usize][x as usize] {
                        Seat::Occupied => {
                            count += 1;
                            break;
                        }
                        Seat::Empty => {
                            break;
                        }
                        _ => {}
                    }
                } else {
                    break;
                }

                yo += offset.0;
                xo += offset.1;
            }
        }

        count
    }

    fn changed(&self) -> bool {
        self.seats != self.seats_previous
    }

    fn occupied_count(&self) -> usize {
        self.seats
            .iter()
            .flatten()
            .filter(|&s| s == &Seat::Occupied)
            .count()
    }
}
