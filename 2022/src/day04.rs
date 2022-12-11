use std::num::ParseIntError;
use std::str::FromStr;

use anyhow::Result;

#[derive(Debug)]
enum ParseError {
    InvalidFormat,
    Parse(ParseIntError),
}

impl From<ParseIntError> for ParseError {
    fn from(err: ParseIntError) -> Self {
        ParseError::Parse(err)
    }
}
struct Section(std::ops::RangeInclusive<usize>);

impl FromStr for Section {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once('-').ok_or(ParseError::InvalidFormat)?;
        Ok(Section(a.parse()?..=b.parse()?))
    }
}

struct Pair(Section, Section);

impl FromStr for Pair {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(',').ok_or(ParseError::InvalidFormat)?;
        Ok(Pair(left.parse()?, right.parse()?))
    }
}

pub fn part1(input: String) -> Result<()> {
    let total: usize = input
        .lines()
        .map(Pair::from_str)
        .map(Result::unwrap)
        .filter(|pair| {
            let Pair(Section(a), Section(b)) = pair;
            let (larger, smaller) = if a.end() - a.start() > b.end() - b.start() {
                (a, b)
            } else {
                (b, a)
            };
            larger.contains(smaller.start()) && larger.contains(smaller.end())
        })
        .count();
    println!("day04 part1: total={}", total);
    Ok(())
}

pub fn part2(input: String) -> Result<()> {
    let total: usize = input
        .lines()
        .map(Pair::from_str)
        .map(Result::unwrap)
        .filter(|pair| {
            let Pair(Section(a), Section(b)) = pair;
            a.start() <= b.end() && b.start() <= a.end()
        })
        .count();
    println!("day04 part2: total={}", total);
    Ok(())
}
