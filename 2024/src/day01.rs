#[cfg(test)]
mod tests {
    use anyhow::Result;
    use std::collections::HashMap;

    #[test]
    fn part1() -> Result<()> {
        let mut a: Vec<isize> = vec![];
        let mut b: Vec<isize> = vec![];

        for line in std::fs::read_to_string("input/day01.txt")?.lines() {
            let (aitem, bitem) = line
                .split_once(char::is_whitespace)
                .ok_or_else(|| anyhow::anyhow!("Invalid input"))?;
            a.push(aitem.trim().parse()?);
            b.push(bitem.trim().parse()?);
        }

        // sort location ids
        a.sort_unstable();
        b.sort_unstable();

        // total diff for each location
        let diff: isize = a.iter().zip(b.iter()).map(|(a, b)| (a - b).abs()).sum();

        assert_eq!(diff, 1388114);
        Ok(())
    }

    #[test]
    fn part2() -> Result<()> {
        let mut a: HashMap<isize, isize> = HashMap::new();
        let mut b: HashMap<isize, isize> = HashMap::new();

        for line in std::fs::read_to_string("input/day01.txt")?.lines() {
            let (aitem, bitem) = line
                .split_once(char::is_whitespace)
                .ok_or_else(|| anyhow::anyhow!("Invalid input"))?;
            *a.entry(aitem.trim().parse()?).or_insert(0) += 1;
            *b.entry(bitem.trim().parse()?).or_insert(0) += 1;
        }

        let similarity: isize = a
            .iter()
            .map(|(num, times_in_left)| {
                let times_in_right = b.get(num).unwrap_or(&0);
                times_in_left * num * times_in_right
            })
            .sum();

        assert_eq!(similarity, 23529853);
        Ok(())
    }
}
