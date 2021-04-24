//! This solution definitely requires some refactoring :C

#[macro_use]
extern crate lazy_static;

use anyhow::{bail, Result};
use regex::{Captures, Regex};
use std::io::{self, Error, ErrorKind, Read, Write};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

lazy_static! {
    static ref RE_PAR: Regex = Regex::new(r"\([0-9*\+ ]+\)").unwrap();
    static ref RE_ADD: Regex = Regex::new(r"\d+ \+ \d+").unwrap();
    static ref RE_PAR_REM: Regex = Regex::new(r"\(\d+\)").unwrap();
}

fn part1(input: &str) -> Result<()> {
    process(input, eval_process_part1)
}

fn part2(input: &str) -> Result<()> {
    process(input, eval_process_part2)
}

fn process(input: &str, eval_fn: impl Fn(&str) -> Result<u64>) -> Result<()> {
    let result = input.lines().flat_map(eval_fn).sum::<u64>();

    writeln!(io::stdout(), "{}", result)?;
    Ok(())
}

fn eval_process_part1(line: &str) -> Result<u64> {
    let mut line = line.to_string();
    while has_parentheses(line.as_str()) {
        line = eval_inside_parentheses(line);
    }
    eval_default(&line, false)
}

fn eval_process_part2(line: &str) -> Result<u64> {
    let mut line = line.to_string();
    while has_parentheses(line.as_str()) {
        while has_additions(line.as_str()) {
            line = eval_additions(line);
            while has_unnecessary_parentheses(line.as_str()) {
                line = remove_unnecessary_parentheses(line);
            }
        }
        line = eval_inside_parentheses(line);
    }
    while has_additions(line.as_str()) {
        line = eval_additions(line);
    }
    eval_default(&line, false)
}

fn has_parentheses(line: &str) -> bool {
    RE_PAR.is_match(line)
}

fn eval_inside_parentheses(line: String) -> String {
    // Todo: find a way to properly fail .replace_all() and then replace .unwrap_or_default()
    RE_PAR
        .replace_all(line.as_str(), |c: &Captures| {
            eval_default(&c[0], true).unwrap_or_default().to_string()
        })
        .to_string()
}

fn has_additions(line: &str) -> bool {
    RE_ADD.is_match(line)
}

fn eval_additions(line: String) -> String {
    // Todo: find a way to properly fail .replace_all() and then replace .unwrap_or_default()
    RE_ADD
        .replace_all(line.as_str(), |c: &Captures| {
            eval_one_addition(&c[0]).unwrap_or_default().to_string()
        })
        .to_string()
}

fn has_unnecessary_parentheses(line: &str) -> bool {
    RE_PAR_REM.is_match(line)
}

fn remove_unnecessary_parentheses(line: String) -> String {
    RE_PAR_REM
        .replace_all(line.as_str(), |c: &Captures| {
            c[0].trim_matches(|c| c == '(' || c == ')').to_string()
        })
        .to_string()
}

#[derive(Debug, Clone, PartialEq)]
enum Operation {
    Add,
    Mul,
}

fn eval_default(expr: &str, trim: bool) -> Result<u64> {
    if !expr.is_ascii() {
        bail!("Invalid input")
    }

    let expr = if trim { &expr[1..expr.len() - 1] } else { expr };

    let result: Result<_> =
        expr.split_ascii_whitespace()
            .enumerate()
            .try_fold((0_u64, None), |mut acc, n| {
                match n.0 & 1 == 0 {
                    // Number
                    true => {
                        let n = n.1.parse()?;
                        match acc.1 {
                            Some(op) => match op {
                                Operation::Add => {
                                    acc = (acc.0 + n, None);
                                }
                                Operation::Mul => {
                                    acc = (acc.0 * n, None);
                                }
                            },
                            None => {
                                acc = (n, None);
                            }
                        }
                    }
                    // Operation
                    false => {
                        let op = match n.1 {
                            "+" => Operation::Add,
                            "*" => Operation::Mul,
                            _ => bail!("Invalid input"),
                        };
                        acc = (acc.0, Some(op));
                    }
                }
                Ok(acc)
            });

    Ok(result?.0)
}

fn eval_one_addition(expr: &str) -> Result<u64> {
    let mut expr_iter = expr.split_ascii_whitespace();
    let first = expr_iter
        .next()
        .ok_or_else(|| Error::from(ErrorKind::InvalidData))?;
    expr_iter
        .next()
        .ok_or_else(|| Error::from(ErrorKind::InvalidData))?;
    let second = expr_iter
        .next()
        .ok_or_else(|| Error::from(ErrorKind::InvalidData))?;
    Ok(first.parse::<u64>()? + second.parse::<u64>()?)
}
