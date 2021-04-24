//! ndarray solution
//! because I don't like Vec<Vec<_>> stuff :3

use anyhow::{bail, Result};
use ndarray::{concatenate, s, Array2, ArrayView1, ArrayView2, Axis};
use std::collections::{BTreeMap, VecDeque};
use std::convert::TryFrom;
use std::io::{self, Error, ErrorKind, Read, Write};
use std::ops::RangeInclusive;
use std::str::FromStr;

// We had to get these magic numbers manually by analyzing the input data :O
const TILE_SIZE: usize = 10;
// We know that we should have 144 input tiles, and we have a square map
const WORLD_SIZE: usize = 12;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let world = fill_world(input)?;
    writeln!(
        io::stdout(),
        "{}",
        world
            .corner_id_product()
            .ok_or_else(|| Error::from(ErrorKind::InvalidData))?
    )?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let world = fill_world(input)?;
    let mut gworld = GluedWorld::try_from(world)?;

    // |                  # |
    // |#    ##    ##    ###|
    // | #  #  #  #  #  #   |
    // 3x20
    let monster_str = "                  # \n#    ##    ##    ###\n #  #  #  #  #  #   ";
    let monster = monster_str.lines().enumerate().fold(
        Array2::<u8>::default((3, 20)),
        |mut acc, (row_n, line)| {
            line.chars().enumerate().for_each(|(col_n, ch)| {
                if ch == '#' {
                    acc[[row_n, col_n]] = 1
                }
            });
            acc
        },
    );

    gworld.remove_pattern(monster.view())?;
    writeln!(io::stdout(), "{}", gworld.habitat())?;
    Ok(())
}

fn fill_world(input: &str) -> Result<World> {
    let mut tiles: VecDeque<Tile> = input
        .split("\r\n\r\n")
        .filter(|s| !s.is_empty())
        .map(Tile::from_str)
        .collect::<Result<_>>()?;

    if tiles.len() < WORLD_SIZE * WORLD_SIZE {
        bail!("Invalid input");
    }

    // Just place first_tile in the middle of the World
    // We may use .unwrap() here because we checked the length
    let mut world = World::new(tiles.pop_front().unwrap());

    loop {
        if tiles.is_empty() {
            break;
        }

        // We may use .unwrap() here because we checked the length
        let current_tile = tiles.pop_front().unwrap();

        if let Err(tile) = world.try_place(current_tile) {
            tiles.push_back(tile);
        }

        // This code will loop forever if we have enough tiles, but they don't fit together :C
    }

    Ok(world)
}

struct Transformer;

impl Transformer {
    // Counterclockwise
    // https://github.com/rust-ndarray/ndarray/issues/866
    fn rotate(array: &mut Array2<u8>) {
        array.swap_axes(0, 1);
        array.invert_axis(Axis(0));
    }

    fn flip(array: &mut Array2<u8>) {
        // The workaround - because .reversed_axis() can't mutate inplace
        let mut temp = Array2::<u8>::default((0, 0));
        std::mem::swap(&mut temp, array);
        temp = temp.reversed_axes();
        std::mem::swap(&mut temp, array);
    }
}

#[derive(Debug)]
struct Tile {
    id: u64,
    array: Array2<u8>,
    side: usize,
}

impl Tile {
    fn upper(&self) -> ArrayView1<u8> {
        self.array.row(0)
    }

    fn lower(&self) -> ArrayView1<u8> {
        self.array.row(self.side - 1)
    }

    fn left(&self) -> ArrayView1<u8> {
        self.array.column(0)
    }

    fn right(&self) -> ArrayView1<u8> {
        self.array.column(self.side - 1)
    }

    fn rotate(&mut self) {
        Transformer::rotate(&mut self.array);
    }

    fn flip(&mut self) {
        Transformer::flip(&mut self.array);
    }

    fn inner(&self) -> ArrayView2<u8> {
        self.array.slice(s![1..(self.side - 1), 1..(self.side - 1)])
    }
}

impl FromStr for Tile {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut value_lines = value.lines();
        let id = value_lines
            .next()
            .ok_or_else(|| Error::from(ErrorKind::InvalidData))?
            .trim_start_matches("Tile ")
            .trim_end_matches(':')
            .parse()?;

        let side = TILE_SIZE;
        let mut array = Array2::<u8>::default((side, side));

        for (row_n, row) in value_lines.enumerate() {
            for (col_n, char) in row.chars().enumerate() {
                if char == '#' {
                    array[[row_n, col_n]] = 1;
                }
                if col_n >= TILE_SIZE {
                    bail!("Invalid input")
                }
            }
            if row_n >= TILE_SIZE {
                bail!("Invalid input")
            }
        }

        Ok(Tile { id, array, side })
    }
}

#[derive(Debug)]
struct World {
    map: BTreeMap<(i32, i32), Tile>,
    limit_x: Option<RangeInclusive<i32>>,
    limit_y: Option<RangeInclusive<i32>>,
}

enum PlaceDirection {
    Left,
    Right,
    Top,
    Bottom,
}

impl PlaceDirection {
    fn coord(&self, other: &(i32, i32)) -> (i32, i32) {
        match self {
            PlaceDirection::Left => (other.0 - 1, other.1),
            PlaceDirection::Right => (other.0 + 1, other.1),
            PlaceDirection::Top => (other.0, other.1 + 1),
            PlaceDirection::Bottom => (other.0, other.1 - 1),
        }
    }

    fn suit(&self, old_tile: &Tile, new_tile: &Tile) -> bool {
        match self {
            PlaceDirection::Left => old_tile.left() == new_tile.right(),
            PlaceDirection::Right => old_tile.right() == new_tile.left(),
            PlaceDirection::Top => old_tile.upper() == new_tile.lower(),
            PlaceDirection::Bottom => old_tile.lower() == new_tile.upper(),
        }
    }
}

static PLACE_DIRECTIONS: &[PlaceDirection] = &[
    PlaceDirection::Left,
    PlaceDirection::Right,
    PlaceDirection::Top,
    PlaceDirection::Bottom,
];

enum PlaceAction {
    Rotate,
    Flip,
}

static PLACE_ACTIONS: &[PlaceAction] = &[
    PlaceAction::Rotate,
    PlaceAction::Rotate,
    PlaceAction::Rotate,
    PlaceAction::Flip,
    PlaceAction::Rotate,
    PlaceAction::Rotate,
    PlaceAction::Rotate,
];

impl World {
    fn new(first_tile: Tile) -> Self {
        let mut map = BTreeMap::new();
        map.insert((0, 0), first_tile);

        World {
            map,
            limit_x: None,
            limit_y: None,
        }
    }

    fn try_place(&mut self, mut new_tile: Tile) -> ::std::result::Result<(), Tile> {
        let mut new_coord_confirmed = None;
        let mut place_action_iter = PLACE_ACTIONS.iter();

        'outer: loop {
            // For each tile in map
            for (old_coord, old_tile) in self.map.iter() {
                // Try to place in the:
                // Left | Right | Up | Down
                for direction in PLACE_DIRECTIONS {
                    let new_coord = direction.coord(old_coord);

                    if !self.within_limits(&new_coord) {
                        continue;
                    }

                    if self.map.contains_key(&new_coord) {
                        continue;
                    }

                    if direction.suit(old_tile, &new_tile) {
                        new_coord_confirmed = Some(new_coord);
                        break 'outer;
                    }
                }
            }
            // If not - choose 1 in order
            // Rotate, Rotate, Rotate, Flip, Rotate, Rotate, Rotate
            // If not - Err
            match place_action_iter.next() {
                Some(action) => match action {
                    PlaceAction::Rotate => new_tile.rotate(),
                    PlaceAction::Flip => new_tile.flip(),
                },
                None => break 'outer,
            }
        }

        match new_coord_confirmed {
            None => Err(new_tile),
            Some(ncc) => {
                self.map.insert(ncc, new_tile);
                self.update_limits();
                Ok(())
            }
        }
    }

    fn within_limits(&self, new_coord: &(i32, i32)) -> bool {
        if let Some(lim_x) = &self.limit_x {
            if !lim_x.contains(&new_coord.0) {
                return false;
            }
        }
        if let Some(lim_y) = &self.limit_y {
            if !lim_y.contains(&new_coord.1) {
                return false;
            }
        }
        true
    }

    fn update_limits(&mut self) {
        if self.limit_x.is_none() {
            // We may use .unwrap() here because we have at least one tile in the world
            let min_x = self.map.keys().map(|x| x.0).min().unwrap();
            let max_x = self.map.keys().map(|x| x.0).max().unwrap();

            if max_x - min_x >= WORLD_SIZE as i32 - 1 {
                self.limit_x = Some(RangeInclusive::new(min_x, max_x));
            }
        }
        if self.limit_y.is_none() {
            // We may use .unwrap() here because we have at least one tile in the world
            let min_y = self.map.keys().map(|x| x.1).min().unwrap();
            let max_y = self.map.keys().map(|x| x.1).max().unwrap();

            if max_y - min_y >= WORLD_SIZE as i32 - 1 {
                self.limit_y = Some(RangeInclusive::new(min_y, max_y));
            }
        }
    }

    fn limits_as_tuple(&self) -> Option<(i32, i32, i32, i32)> {
        match (&self.limit_x, &self.limit_y) {
            (Some(ref limit_x), Some(ref limit_y)) => {
                let min_x = *limit_x.start();
                let max_x = *limit_x.end();
                let min_y = *limit_y.start();
                let max_y = *limit_y.end();
                Some((min_x, max_x, min_y, max_y))
            }
            _ => None,
        }
    }

    fn corner_id_product(&self) -> Option<u64> {
        let (min_x, max_x, min_y, max_y) = self.limits_as_tuple()?;

        let coords = [
            (min_x, min_y),
            (min_x, max_y),
            (max_x, min_y),
            (max_x, max_y),
        ];

        // We may use .unwrap() here because we DEFINITELY have elements with these coords
        Some(coords.iter().map(|c| self.map.get(c).unwrap().id).product())
    }
}

struct GluedWorld {
    array: Array2<u8>,
}

impl TryFrom<World> for GluedWorld {
    type Error = anyhow::Error;

    fn try_from(world: World) -> Result<Self, Self::Error> {
        let (min_x, max_x, min_y, max_y) = world
            .limits_as_tuple()
            .ok_or_else(|| Error::from(ErrorKind::InvalidData))?;

        // Let's make sure that the world size is correct and that all the tiles are filled
        if world.map.len() != WORLD_SIZE * WORLD_SIZE
            || max_x - min_x + 1 != WORLD_SIZE as i32
            || max_y - min_y + 1 != WORLD_SIZE as i32
        {
            bail!("Invalid input");
        }

        // Then we may use .unwrap() here safely
        let array_lines = (min_y..=max_y)
            .rev()
            .map(|y| {
                let array_line_views = (min_x..=max_x)
                    .map(|x| world.map.get(&(x, y)).unwrap().inner())
                    .collect::<Vec<_>>();
                concatenate(Axis(1), &array_line_views).unwrap()
            })
            .collect::<Vec<_>>();

        let array_view_lines = array_lines.iter().map(|al| al.view()).collect::<Vec<_>>();

        Ok(GluedWorld {
            array: concatenate(Axis(0), &array_view_lines).unwrap(),
        })
    }
}

impl GluedWorld {
    fn habitat(&self) -> usize {
        self.array.iter().filter(|&&v| v == 1).count()
    }

    fn remove_pattern(&mut self, pattern: ArrayView2<u8>) -> Result<()> {
        let mut changed = false;
        let mut place_action_iter = PLACE_ACTIONS.iter().chain(PLACE_ACTIONS.iter());

        loop {
            let pattern_len_x = pattern.shape()[0];
            let pattern_len_y = pattern.shape()[1];

            // I would like to use .windows() here, but they are not mutable
            // https://github.com/rust-ndarray/ndarray/pull/826

            let self_len_x = self.array.shape()[0];
            let self_len_y = self.array.shape()[1];

            for x in 0..=(self_len_x - pattern_len_x) {
                for y in 0..=(self_len_y - pattern_len_y) {
                    let bounds = s![x..(x + pattern_len_x), y..(y + pattern_len_y)];

                    let slice_cloned = self.array.slice(&bounds).to_owned() | pattern;
                    let mut slice_mut = self.array.slice_mut(&bounds);

                    if slice_cloned != slice_mut {
                        continue;
                    }

                    slice_mut -= &pattern;
                    changed = true;
                }
            }

            if changed {
                break;
            }

            match place_action_iter.next() {
                Some(action) => match action {
                    PlaceAction::Rotate => Transformer::rotate(&mut self.array),
                    PlaceAction::Flip => Transformer::flip(&mut self.array),
                },
                // Bad case: the pattern is not detected on the map completely
                None => bail!("Invalid input"),
            }
        }
        Ok(())
    }
}
