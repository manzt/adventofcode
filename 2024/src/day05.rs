use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct RuleSet {
    rules: HashMap<u8, HashSet<u8>>,
}

impl RuleSet {
    /// Validates whether the manual is correctly sorted.
    ///
    /// If the manual is already correctly sorted, returns the middle page
    /// in the `Ok` variant.
    ///
    /// If the manual is not correctly sorted, it is sorted internally, and
    /// the middle page of this sorted version is returned in the `Err` variant.
    fn validate(&self, manual: &[u8]) -> Result<u8, u8> {
        let mut sorted = manual.to_vec();
        sorted.sort_by(|a, b| {
            self.rules
                .get(a)
                .and_then(|inner| inner.get(b))
                .map(|_| Ordering::Less)
                .unwrap_or(Ordering::Greater)
        });
        if sorted.iter().zip(manual.iter()).all(|(a, b)| a == b) {
            Ok(manual[manual.len() / 2])
        } else {
            Err(sorted[sorted.len() / 2])
        }
    }
}

impl std::str::FromStr for RuleSet {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rules = HashMap::<u8, HashSet<u8>>::new();
        for line in s.lines() {
            let (a, b) = line
                .split_once('|')
                .ok_or_else(|| anyhow::anyhow!("Invalid input, missing '|'"))?;
            rules.entry(a.parse()?).or_default().insert(b.parse()?);
        }
        Ok(RuleSet { rules })
    }
}

#[derive(Debug)]
pub struct Puzzle {
    pub rules: RuleSet,
    pub manuals: Vec<Vec<u8>>,
}

impl std::str::FromStr for Puzzle {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (rules, manuals) = s
            .split_once("\n\n")
            .ok_or_else(|| anyhow::anyhow!("Invalid input, missing '\\n\\n'"))?;
        Ok(Puzzle {
            rules: rules.parse()?,
            manuals: manuals
                .lines()
                .map(|line| {
                    line.split(',')
                        .map(|n| n.parse())
                        .collect::<Result<Vec<_>, _>>()
                })
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

#[test]
fn part1() {
    let Puzzle { rules, manuals } = std::fs::read_to_string("input/day05.txt")
        .unwrap()
        .parse::<Puzzle>()
        .unwrap();
    assert_eq!(
        4185,
        manuals
            .iter()
            .filter_map(|manual| match rules.validate(manual) {
                Ok(mid) => Some(mid as usize),
                Err(_) => None,
            })
            .sum::<usize>()
    );
}

#[test]
fn part2() {
    let Puzzle { rules, manuals } = std::fs::read_to_string("input/day05.txt")
        .unwrap()
        .parse::<Puzzle>()
        .unwrap();
    assert_eq!(
        4480,
        manuals
            .iter()
            .filter_map(|manual| match rules.validate(manual) {
                Ok(_) => None,
                Err(mid) => Some(mid as usize),
            })
            .sum::<usize>()
    );
}
