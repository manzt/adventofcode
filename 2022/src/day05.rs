use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::{num::ParseIntError, str::FromStr};

fn parse_stacks(s: &str) -> Vec<Vec<char>> {
    let mut lines: Vec<_> = s.lines().collect();
    lines
        .pop()
        .unwrap()
        .chars()
        .enumerate()
        .filter_map(|(i, c)| match c {
            ' ' => None,
            _ => Some(
                lines
                    .iter()
                    .rev()
                    .filter_map(|line| match line.chars().nth(i) {
                        Some(' ') | None => None,
                        x => x,
                    })
                    .collect::<Vec<_>>(),
            ),
        })
        .collect()
}

#[derive(Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl FromStr for Move {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
        }
        let cap = RE.captures(s).unwrap();
        Ok(Move {
            count: cap[1].parse()?,
            from: cap[2].parse()?,
            to: cap[3].parse()?,
        })
    }
}

type Stack = Vec<char>;

fn parse_input(input: String) -> (Vec<Stack>, Vec<Move>) {
    let (header, rest) = input.split_once("\n\n").unwrap();
    let moves = rest
        .lines()
        .map(Move::from_str)
        .map(Result::unwrap)
        .collect();
    (parse_stacks(header), moves)
}

pub fn part1(input: String) -> Result<()> {
    let (mut stacks, moves) = parse_input(input);
    for m in moves {
        for _ in 0..m.count {
            if let Some(c) = stacks[m.from - 1].pop() {
                stacks[m.to - 1].push(c);
            }
        }
    }
    let results: Option<String> = stacks.iter().map(|s| s.last()).collect();
    println!("day05 part1: {}", results.unwrap());
    Ok(())
}

pub fn part2(input: String) -> Result<()> {
    let (mut stacks, moves) = parse_input(input);
    for m in moves {
        let start = stacks[m.from - 1].len() - m.count;
        let mut crates: Vec<_> = stacks[m.from - 1].drain(start..).collect();
        stacks[m.to - 1].append(&mut crates);
    }
    let results: Option<String> = stacks.iter().map(|s| s.last()).collect();
    println!("day05 part2: {}", results.unwrap());
    Ok(())
}
