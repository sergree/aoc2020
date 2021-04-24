use anyhow::{anyhow, Result};
use std::cmp::Ordering;
use std::collections::{HashSet, VecDeque};
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
    process(input, false)
}

fn part2(input: &str) -> Result<()> {
    process(input, true)
}

fn process(input: &str, recursive: bool) -> Result<()> {
    let (first_player, second_player) = get_player_data(input)?;

    let (winner, _) = combat(recursive, first_player, second_player);

    writeln!(io::stdout(), "{}", winner.score())?;
    Ok(())
}

fn get_player_data(input: &str) -> Result<(Deck, Deck)> {
    let mut player_iter = input.split("\r\n\r\n");

    let first_player = Deck::from_str(
        player_iter
            .next()
            .ok_or_else(|| Error::from(ErrorKind::InvalidData))?,
    )?;
    let second_player = Deck::from_str(
        player_iter
            .next()
            .ok_or_else(|| Error::from(ErrorKind::InvalidData))?,
    )?;

    // We need to check that all card values are unique
    let mut card_values = first_player
        .deck
        .iter()
        .chain(second_player.deck.iter())
        .copied()
        .collect::<Vec<_>>();
    let all_card_count = card_values.len();
    card_values.sort_unstable();
    card_values.dedup();
    let unique_card_count = card_values.len();

    match all_card_count == unique_card_count {
        true => Ok((first_player, second_player)),
        false => Err(anyhow!("Invalid input")),
    }
}

enum Winner {
    First,
    Second,
}

fn combat(recursive: bool, mut first_player: Deck, mut second_player: Deck) -> (Deck, Winner) {
    let mut history: HashSet<(Vec<u32>, Vec<u32>)> = HashSet::new();

    loop {
        let first_player_vec = first_player.to_vec();
        let second_player_vec = second_player.to_vec();
        let player_vecs = (first_player_vec, second_player_vec);

        if history.contains(&(player_vecs)) {
            return (first_player, Winner::First);
        }

        history.insert(player_vecs);

        let (winner, first, second) = match (first_player.top(), second_player.top()) {
            (Some(first), Some(second))
                if recursive
                    && first_player.len() >= first as usize
                    && second_player.len() >= second as usize =>
            {
                let (_, who_won) = combat(
                    true,
                    first_player.clone_take(first as usize),
                    second_player.clone_take(second as usize),
                );
                (who_won, first, second)
            }
            (Some(first), Some(second)) => match first.cmp(&second) {
                Ordering::Less => (Winner::Second, first, second),
                // We checked that the value of all the cards is unique, so this code is definitely unreachable
                Ordering::Equal => unreachable!(),
                Ordering::Greater => (Winner::First, first, second),
            },
            // This part of the code is definitely unreachable, because we break before it
            // (line 123 in the previous iteration)
            _ => unreachable!(),
        };

        match winner {
            Winner::First => {
                first_player.move_bottom(first);
                first_player.move_bottom(second);
            }
            Winner::Second => {
                second_player.move_bottom(second);
                second_player.move_bottom(first);
            }
        }

        if first_player.len() == 0 || second_player.len() == 0 {
            break;
        }
    }

    match first_player.len() > 0 {
        true => (first_player, Winner::First),
        false => (second_player, Winner::Second),
    }
}

#[derive(Debug)]
struct Deck {
    deck: VecDeque<u32>,
}

impl FromStr for Deck {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let deck = value
            .lines()
            .skip(1)
            .map(|l| l.parse())
            .collect::<Result<_, _>>()?;
        Ok(Deck { deck })
    }
}

impl Deck {
    fn top(&mut self) -> Option<u32> {
        self.deck.pop_front()
    }

    fn move_bottom(&mut self, card: u32) {
        self.deck.push_back(card)
    }

    fn len(&self) -> usize {
        self.deck.len()
    }

    fn score(&self) -> u32 {
        self.deck
            .iter()
            .rev()
            .enumerate()
            .map(|(ix, card)| card * (ix as u32 + 1))
            .sum()
    }

    fn to_vec(&self) -> Vec<u32> {
        self.deck.iter().cloned().collect()
    }

    fn clone_take(&self, count: usize) -> Deck {
        Deck {
            deck: self.deck.iter().cloned().take(count).collect(),
        }
    }
}
