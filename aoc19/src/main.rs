//! Inspired by
//! https://dev.to/qviper/advent-of-code-2020-python-solution-day-19-4p9d

use anyhow::{anyhow, bail, Result};
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

fn process(input: &str, part2: bool) -> Result<()> {
    if !input.is_ascii() {
        bail!("Invalid input");
    }

    let mut input_iter = input.split("\r\n\r\n");

    let mut rules = input_iter
        .next()
        .ok_or_else(|| Error::from(ErrorKind::InvalidData))?
        .lines()
        .map(|c| NumberedRule::from_str(c))
        .collect::<Result<Vec<_>>>()?;
    rules.sort_unstable_by_key(|f| f.0);
    let mut rules = rules.into_iter().map(|nr| nr.1).collect::<Vec<_>>();

    if part2 {
        match rules.len() >= 12 {
            true => {
                rules[8] = Rule::Multi(vec![vec![42], vec![42, 8]]);
                rules[11] = Rule::Multi(vec![vec![42, 31], vec![42, 11, 31]])
            }
            false => bail!("Invalid input"),
        }
    }

    let rules = rules;

    // To use the index safely,
    // we need to check that the maximum value in the vector
    // does not exceed the length of the vector
    let max_rule_value = rules
        .iter()
        .map(|r| match r {
            Rule::Multi(v) => v.iter().flatten().copied().max().unwrap_or(0),
            _ => 0,
        })
        .max()
        .ok_or_else(|| Error::from(ErrorKind::InvalidData))?;

    if max_rule_value >= rules.len() {
        bail!("Invalid input");
    }

    let messages = input_iter
        .next()
        .ok_or_else(|| Error::from(ErrorKind::InvalidData))?
        .lines()
        .collect::<Vec<_>>();
    let rule_0 = rules
        .get(0)
        .ok_or_else(|| Error::from(ErrorKind::InvalidData))?;

    match rule_0 {
        Rule::Multi(v) => {
            let stack_0 = cloned_rev(
                v.get(0)
                    .ok_or_else(|| Error::from(ErrorKind::InvalidData))?,
            );
            writeln!(
                io::stdout(),
                "{}",
                messages
                    .iter()
                    .filter(|m| matches(m, &rules, stack_0.clone()))
                    .count()
            )?;
            Ok(())
        }
        _ => Err(anyhow!("Invalid input")),
    }
}

fn cloned_rev(v: &[usize]) -> Vec<StackedValue> {
    v.iter().cloned().rev().map(StackedValue::Usize).collect()
}

fn matches(message: &str, rules: &[Rule], mut stack: Vec<StackedValue>) -> bool {
    if stack.len() > message.len() {
        return false;
    } else if stack.is_empty() || message.is_empty() {
        return stack.is_empty() && message.is_empty();
    }

    // We may call .unwrap() here because we checked the stack length above
    let c = stack.pop().unwrap();

    match c {
        StackedValue::Char(c) => {
            // We may call .unwrap() here because we checked the message length above
            if message.chars().next().unwrap() == c {
                // We can use the index because we have verified that the input data is ASCII
                return matches(&message[1..], rules, stack);
            }
        }
        StackedValue::Usize(n) => {
            // We checked the rule values in line 68 to use the index here
            let values = match &rules[n] {
                Rule::Single(c) => {
                    vec![vec![StackedValue::Char(*c)]]
                }
                Rule::Multi(v) => v.iter().map(|v| cloned_rev(v)).collect(),
            };
            for rule in values {
                let mut ext_stack = stack.clone();
                ext_stack.extend(rule);

                if matches(message, rules, ext_stack) {
                    return true;
                }
            }
        }
    }
    false
}

#[derive(Debug, Clone)]
enum StackedValue {
    Char(char),
    Usize(usize),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Rule {
    Single(char),
    Multi(Vec<Vec<usize>>),
}

#[derive(Debug)]
struct NumberedRule(usize, Rule);

impl FromStr for NumberedRule {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut value_iter = value.split(':');
        let number = value_iter
            .next()
            .ok_or_else(|| Error::from(ErrorKind::InvalidData))?
            .parse()?;
        let rest = value_iter
            .next()
            .ok_or_else(|| Error::from(ErrorKind::InvalidData))?
            .trim_start();
        match rest.chars().next() {
            Some('"') => Ok(NumberedRule(
                number,
                Rule::Single(
                    rest.chars()
                        .nth(1)
                        .ok_or_else(|| Error::from(ErrorKind::InvalidData))?,
                ),
            )),
            _ => {
                let vecs = rest
                    .split(" | ")
                    .map(|s| s.split_ascii_whitespace().map(|n| n.parse()).collect())
                    .collect::<Result<_, _>>()?;
                Ok(NumberedRule(number, Rule::Multi(vecs)))
            }
        }
    }
}
