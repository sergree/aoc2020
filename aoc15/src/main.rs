use anyhow::Result;
use std::collections::{hash_map::Entry, HashMap, VecDeque};
use std::io::{self, Read, Write};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    process(input, 2020)
}

fn part2(input: &str) -> Result<()> {
    process(input, 30000000)
}

fn process(input: &str, max_n: usize) -> Result<()> {
    let mut number_iter = input.split(',').map(|s| s.trim_end().parse::<u64>());

    let mut history: HashMap<_, VecDeque<_>> = HashMap::new();
    let mut previous_number = 0;
    let mut current_number = 0;

    for i in 1..=max_n {
        // To speed up the execution, we need to remove the match on each iteration
        // But for now I'm fine with that
        current_number = match number_iter.next() {
            Some(n) => n?,
            None => match history.entry(previous_number) {
                Entry::Occupied(pvd) => {
                    let pvd = pvd.get();
                    match pvd.len() {
                        1 => 0,
                        // We may call .unwrap() here, because we checked the length above
                        2 => (*pvd.get(0).unwrap() - *pvd.get(1).unwrap()) as u64,
                        // This arm is really unreachable because of lines 50 and 51
                        _ => unreachable!(),
                    }
                }
                Entry::Vacant(_) => 0,
            },
        };

        let cvd = history.entry(current_number).or_insert_with(VecDeque::new);
        cvd.push_front(i);
        cvd.truncate(2);
        previous_number = current_number;
    }

    writeln!(io::stdout(), "{}", current_number)?;
    Ok(())
}
