use anyhow::Result;
use std::str::FromStr;

enum Code {
    A,
    B,
    C,
}

#[derive(Debug)]
struct ParseCodeError;

impl FromStr for Code {
    type Err = ParseCodeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Code::A),
            "B" | "Y" => Ok(Code::B),
            "C" | "Z" => Ok(Code::C),
            _ => Err(ParseCodeError),
        }
    }
}

struct Round(Code, Code);

#[derive(Debug)]
struct ParseRoundError;

impl FromStr for Round {
    type Err = ParseRoundError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let results = s
            .split_once(' ')
            .map(|(a, b)| (a.parse::<Code>(), b.parse::<Code>()));
        match results {
            Some((Ok(a), Ok(b))) => Ok(Self(a, b)),
            _ => Err(ParseRoundError),
        }
    }
}

#[derive(Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl From<Code> for Move {
    fn from(code: Code) -> Self {
        match code {
            Code::A => Move::Rock,
            Code::B => Move::Paper,
            Code::C => Move::Scissors,
        }
    }
}

impl Move {
    fn defeats(&self, other: &Move) -> bool {
        matches!(
            (&self, &other),
            (Move::Rock, Move::Scissors)
                | (Move::Paper, Move::Rock)
                | (Move::Scissors, Move::Paper)
        )
    }

    fn should_play(&self, outcome: &Outcome) -> Move {
        match outcome {
            Outcome::Win => match self {
                Move::Rock => Move::Paper,
                Move::Paper => Move::Scissors,
                Move::Scissors => Move::Rock,
            },
            Outcome::Lose => match self {
                Move::Rock => Move::Scissors,
                Move::Paper => Move::Rock,
                Move::Scissors => Move::Paper,
            },
            Outcome::Tie => self.clone(),
        }
    }
}

enum Outcome {
    Win,
    Lose,
    Tie,
}

impl From<Code> for Outcome {
    fn from(code: Code) -> Self {
        match code {
            Code::A => Outcome::Lose,
            Code::B => Outcome::Tie,
            Code::C => Outcome::Win,
        }
    }
}

fn score(them: &Move, me: &Move) -> i32 {
    let shape_score = match me {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    };

    let outcome_score = them
        .defeats(me)
        .then_some(0)
        .or_else(|| me.defeats(them).then_some(6))
        .unwrap_or(3);

    shape_score + outcome_score
}

pub fn part1(input: String) -> Result<i32> {
    let total: i32 = input
        .lines()
        .map(|line| {
            let Round(a, b): Round = line.parse().unwrap();
            score(&Move::from(a), &Move::from(b))
        })
        .sum();
    println!("day02 part1: score={}", total);
    Ok(total)
}

pub fn part2(input: String) -> Result<i32> {
    let total: i32 = input
        .lines()
        .map(|line| {
            let Round(a, b): Round = line.parse().unwrap();
            let move_them = Move::from(a);
            score(&move_them, &move_them.should_play(&Outcome::from(b)))
        })
        .sum();
    println!("day02 part2: score={}", total);
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "A Y\nB X\nC Z";
        assert_eq!(part1(input.to_string()).unwrap(), 15);
    }
}
