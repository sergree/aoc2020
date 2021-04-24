use anyhow::{anyhow, Result};
use std::convert::TryFrom;
use std::io::{self, Error, ErrorKind, Read, Write};
use std::ops::RangeInclusive;
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

fn process(input: &str, advanced_validation: bool) -> Result<()> {
    let unverified_passports = get_unverified_data(input);

    match advanced_validation {
        false => writeln!(io::stdout(), "{}", unverified_passports.len())?,
        true => writeln!(
            io::stdout(),
            "{}",
            unverified_passports
                .into_iter()
                .flat_map(Passport::try_from)
                .count()
        )?,
    }

    Ok(())
}

fn get_unverified_data(input: &str) -> Vec<PassportUnverified> {
    input
        .split("\r\n\r\n")
        .flat_map(PassportUnverified::try_from)
        .filter(|p| p.has_all_required_fields())
        .collect()
}

#[derive(Default, Debug)]
struct PassportUnverified<'a> {
    birth_year: Option<&'a str>,
    issue_year: Option<&'a str>,
    expiration_year: Option<&'a str>,
    height: Option<&'a str>,
    hair_color: Option<&'a str>,
    eye_color: Option<&'a str>,
    passport_id: Option<&'a str>,
    country_id: Option<&'a str>,
}

// I use TryFrom instead of FromStr because of E0308
impl<'a> TryFrom<&'a str> for PassportUnverified<'a> {
    type Error = anyhow::Error;

    fn try_from(items: &'a str) -> Result<Self, Self::Error> {
        let mut new_passport = PassportUnverified::default();

        for key_value in items.split_ascii_whitespace() {
            let mut item_iter = key_value.split(':');
            let key = item_iter
                .next()
                .ok_or_else(|| Error::from(ErrorKind::InvalidData))?;
            let value = item_iter
                .next()
                .ok_or_else(|| Error::from(ErrorKind::InvalidData))?;

            match key {
                "byr" => new_passport.birth_year = Some(value),
                "iyr" => new_passport.issue_year = Some(value),
                "eyr" => new_passport.expiration_year = Some(value),
                "hgt" => new_passport.height = Some(value),
                "hcl" => new_passport.hair_color = Some(value),
                "ecl" => new_passport.eye_color = Some(value),
                "pid" => new_passport.passport_id = Some(value),
                "cid" => new_passport.country_id = Some(value),
                _ => {}
            }
        }

        Ok(new_passport)
    }
}

impl PassportUnverified<'_> {
    fn has_all_required_fields(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.passport_id.is_some()
    }
}

// I allow this here because we use this type for validation only
#[allow(dead_code)]
struct Passport {
    birth_year: u16,
    issue_year: u16,
    expiration_year: u16,
    height: Height,
    hair_color: Color,
    eye_color: EyeColor,
    passport_id: u32,
    country_id: Option<u32>,
}

enum Height {
    Cm(u16),
    In(u16),
}

impl FromStr for Height {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.len() {
            4 | 5 => {
                // We should add better UTF-8 support in production here
                // But for now we are fine with using the index
                let unit = value
                    .get((value.len() - 2)..value.len())
                    .ok_or_else(|| Error::from(ErrorKind::InvalidData))?;
                let value = value
                    .get(0..(value.len() - 2))
                    .ok_or_else(|| Error::from(ErrorKind::InvalidData))?
                    .parse()?;

                match unit {
                    "cm" if (150..=193).contains(&value) => Ok(Height::Cm(value)),
                    "in" if (59..=79).contains(&value) => Ok(Height::In(value)),
                    _ => Err(anyhow!("Invalid input")),
                }
            }
            _ => Err(anyhow!("Invalid input")),
        }
    }
}

struct Color(u32);

impl FromStr for Color {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match (value.len(), value.starts_with('#')) {
            (7, true) => Ok(Color(u32::from_str_radix(&value[1..7], 16)?)), // We have already checked the length and the first character, so we may use the index here
            _ => Err(anyhow!("Invalid input")),
        }
    }
}

enum EyeColor {
    Amb,
    Blu,
    Brn,
    Gry,
    Grn,
    Hzl,
    Oth,
}

impl FromStr for EyeColor {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "amb" => Ok(EyeColor::Amb),
            "blu" => Ok(EyeColor::Blu),
            "brn" => Ok(EyeColor::Brn),
            "gry" => Ok(EyeColor::Gry),
            "grn" => Ok(EyeColor::Grn),
            "hzl" => Ok(EyeColor::Hzl),
            "oth" => Ok(EyeColor::Oth),
            _ => Err(anyhow!("Invalid input")),
        }
    }
}

impl TryFrom<PassportUnverified<'_>> for Passport {
    type Error = anyhow::Error;

    fn try_from(u_passport: PassportUnverified) -> Result<Self, Self::Error> {
        // Let's double check this thing
        match u_passport.has_all_required_fields() {
            false => Err(anyhow!("Invalid input")),
            true => {
                // We may use .unwrap()'s here, because we checked .has_all_required_fields() above
                let birth_year =
                    Passport::parse_range(u_passport.birth_year.unwrap(), 1920..=2002)?;

                let issue_year =
                    Passport::parse_range(u_passport.issue_year.unwrap(), 2010..=2020)?;

                let expiration_year =
                    Passport::parse_range(u_passport.expiration_year.unwrap(), 2020..=2030)?;

                let height = Height::from_str(u_passport.height.unwrap())?;

                let hair_color = Color::from_str(u_passport.hair_color.unwrap())?;

                let eye_color = EyeColor::from_str(u_passport.eye_color.unwrap())?;

                let passport_id = Passport::parse_fixed_length(u_passport.passport_id.unwrap(), 9)?;

                let country_id = match u_passport.country_id {
                    Some(cid) => Some(cid.parse()?),
                    None => None,
                };

                Ok(Passport {
                    birth_year,
                    issue_year,
                    expiration_year,
                    height,
                    hair_color,
                    eye_color,
                    passport_id,
                    country_id,
                })
            }
        }
    }
}

impl Passport {
    fn parse_range(raw_value: &str, range: RangeInclusive<u16>) -> Result<u16> {
        match raw_value.parse::<u16>()? {
            value if range.contains(&value) => Ok(value),
            _ => Err(anyhow!("Invalid input")),
        }
    }

    fn parse_fixed_length(raw_value: &str, length: usize) -> Result<u32> {
        match raw_value.parse::<u32>()? {
            value if raw_value.len() == length => Ok(value),
            _ => Err(anyhow!("Invalid input")),
        }
    }
}
