use anyhow::Result;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug)]
struct ParseSackError;

#[derive(Debug)]
struct Sack(HashSet<char>, HashSet<char>);

impl FromStr for Sack {
    type Err = ParseSackError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_at(s.len() / 2);
        Ok(Sack(a.chars().collect(), b.chars().collect()))
    }
}

fn priority(c: &char) -> u32 {
    match *c as u32 {
        x if (97..=127).contains(&x) => x - 97 + 1,
        x if (65..=90).contains(&x) => x - 65 + 27,
        _ => panic!(),
    }
}

pub fn part1(input: String) -> Result<()> {
    let total: u32 = input
        .lines()
        .map(Sack::from_str)
        .map(Result::unwrap)
        .map(|sack| {
            let Sack(a, b) = sack;
            match a.intersection(&b).next() {
                Some(c) => priority(c),
                None => 0,
            }
        })
        .sum();
    println!("day03 part1: total={}", total);
    Ok(())
}

pub fn part2(input: String) -> Result<()> {
    let sacks: Vec<HashSet<char>> = input
        .lines()
        .map(Sack::from_str)
        .map(Result::unwrap)
        .map(|sack| {
            let Sack(a, b) = sack;
            a.union(&b).cloned().collect()
        })
        .collect();

    let total: u32 = sacks
        .chunks(3)
        .map(|grp| {
            let shared = grp.iter().skip(1).fold(grp[0].clone(), |acc, sack| {
                acc.intersection(sack).cloned().collect()
            });
            match shared.iter().next() {
                Some(c) => priority(c),
                None => 0,
            }
        })
        .sum();

    println!("day03 part2: total={}", total);
    Ok(())
}
