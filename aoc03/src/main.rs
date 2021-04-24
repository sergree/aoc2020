use anyhow::Result;
use std::io::{self, Read, Write};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    process(input, &[Slope::new(3, 1)])
}

fn part2(input: &str) -> Result<()> {
    process(
        input,
        &[
            Slope::new(1, 1),
            Slope::new(3, 1),
            Slope::new(5, 1),
            Slope::new(7, 1),
            Slope::new(1, 2),
        ],
    )
}

struct Slope {
    right: usize,
    down: usize,
}

impl Slope {
    fn new(right: usize, down: usize) -> Self {
        Slope { right, down }
    }
}

fn process(input: &str, slopes: &[Slope]) -> Result<()> {
    let result = slopes
        .iter()
        .map(|slope| {
            input
                .lines()
                .enumerate()
                .filter(|&(row_num, _)| row_num % slope.down == 0)
                .map(|(row_num, line)| {
                    let line = line.as_bytes();
                    let col_pos = row_num * slope.right / slope.down % line.len();
                    // We may use the index here because we used the modulus above
                    line[col_pos]
                })
                .filter(|&b| b == b'#')
                .count()
        })
        .product::<usize>();

    writeln!(io::stdout(), "{}", result)?;
    Ok(())
}
