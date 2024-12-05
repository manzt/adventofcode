use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct RuleSet {
    inner: HashMap<u8, HashSet<u8>>,
}

impl RuleSet {
    fn sorted(&self, manual: &[u8]) -> Vec<u8> {
        let mut sorted = manual.to_vec();
        sorted.sort_by(|a, b| {
            self.inner
                .get(a)
                .and_then(|inner| inner.get(b))
                .map(|_| Ordering::Less)
                .unwrap_or(Ordering::Greater)
        });
        sorted
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
        Ok(RuleSet { inner: rules })
    }
}

#[derive(Debug)]
pub struct Puzzle {
    pub ruleset: RuleSet,
    pub manuals: Vec<Vec<u8>>,
}

impl std::str::FromStr for Puzzle {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (rules, manuals) = s
            .split_once("\n\n")
            .ok_or_else(|| anyhow::anyhow!("Invalid input, missing '\\n\\n'"))?;
        Ok(Puzzle {
            ruleset: rules.parse()?,
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

fn is_same_sequence(a: &[u8], b: &[u8]) -> bool {
    a.iter().zip(b.iter()).all(|(a, b)| a == b)
}

#[test]
fn part1() {
    let Puzzle { ruleset, manuals } = std::fs::read_to_string("input/day05.txt")
        .unwrap()
        .parse::<Puzzle>()
        .unwrap();
    assert_eq!(
        4185,
        manuals
            .iter()
            .map(|manual| (manual, ruleset.sorted(manual)))
            .filter(|(manual, sorted)| is_same_sequence(manual, sorted))
            .map(|(_, sorted)| sorted[sorted.len() / 2] as usize)
            .sum::<usize>()
    );
}

#[test]
fn part2() {
    let Puzzle { ruleset, manuals } = std::fs::read_to_string("input/day05.txt")
        .unwrap()
        .parse::<Puzzle>()
        .unwrap();
    assert_eq!(
        4480,
        manuals
            .iter()
            .map(|manual| (manual, ruleset.sorted(manual)))
            .filter(|(manual, sorted)| !is_same_sequence(manual, sorted))
            .map(|(_, sorted)| sorted[sorted.len() / 2] as usize)
            .sum::<usize>()
    );
}
