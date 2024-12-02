struct Report(Vec<i8>);

impl Report {
    /// Check whether the report is safe or not.
    ///
    /// 1.) All values must either be increasing or decreasing.
    /// 2.) Values must only differ by at least 1 and at most 3.
    fn is_safe(&self) -> bool {
        match (self.0.first(), self.0.get(1)) {
            (Some(first), Some(second)) => {
                let direction = (second - first).signum();
                self.0.windows(2).all(|w| {
                    let diff = w[1] - w[0];
                    let within_bounds = (1..=3).contains(&diff.abs());
                    let same_direction = diff.signum() == direction;
                    within_bounds && same_direction
                })
            }
            // only one element is always safe
            (Some(_), None) => true,
            _ => false,
        }
    }

    /// Check whether the repor is safe with dampening.
    ///
    /// The problem dampener lets the safety system tolerate _a single bad value_.
    fn is_safe_with_dampening(&self) -> bool {
        // just brute force it by checking the removal of each element
        self.is_safe()
            || self.0.iter().enumerate().any(|(i, _)| {
                let mut report = self.0.clone();
                report.remove(i);
                Self(report).is_safe()
            })
    }
}

impl std::str::FromStr for Report {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Report(
            s.split(' ')
                .map(|part| part.parse())
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(
            std::fs::read_to_string("input/day02.txt")
                .unwrap()
                .lines()
                .map(|line| line.parse::<Report>().unwrap())
                .filter(|report| report.is_safe())
                .count(),
            486
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            std::fs::read_to_string("input/day02.txt")
                .unwrap()
                .lines()
                .map(|line| line.parse::<Report>().unwrap())
                .filter(|report| report.is_safe_with_dampening())
                .count(),
            540
        );
    }
}
