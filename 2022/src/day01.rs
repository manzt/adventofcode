use std::num::ParseIntError;
use std::str::FromStr;

struct Elf {
    calories: Vec<u32>,
}

impl FromStr for Elf {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let calories: Result<Vec<u32>, _> = s.split('\n').map(|s| s.parse()).collect();
        Ok(Elf {
            calories: calories?,
        })
    }
}

pub fn part1(input: String) {
    let elves: Result<Vec<Elf>, _> = input.split("\n\n").map(|s| s.parse()).collect();
    let max = elves.unwrap().iter().fold(0u32, |max, elf| {
        let total: u32 = elf.calories.iter().sum();
        if total > max {
            total
        } else {
            max
        }
    });
    println!("day01 part1: calories={}", max);
}

pub fn part2(input: String) {
    let elves: Result<Vec<Elf>, _> = input.split("\n\n").map(|s| s.parse()).collect();
    let mut totals: Vec<u32> = elves
        .unwrap()
        .iter()
        .map(|elf| elf.calories.iter().sum())
        .collect();
    totals.sort_by(|a, b| b.cmp(a));
    let sum_of_top_3: u32 = totals.iter().take(3).sum();
    println!("day01 part2: calories={}", sum_of_top_3);
}
