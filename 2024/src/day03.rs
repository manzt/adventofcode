struct InstructionSet(Vec<Instruction>);

enum Instruction {
    Multiply(u32, u32),
    StateChange(State),
}

#[derive(Clone, Copy, PartialEq)]
enum State {
    Enabled,
    Disabled,
}

impl std::str::FromStr for InstructionSet {
    type Err = std::convert::Infallible;

    fn from_str(seq: &str) -> Result<Self, Self::Err> {
        let mut cmds = vec![];
        let mut seq: &str = seq;
        while !seq.is_empty() {
            if let Some(((a, b), rest)) = parse_multiply(seq) {
                cmds.push(Instruction::Multiply(a, b));
                seq = rest;
            } else if let Some((s, rest)) = parse_state_change(seq) {
                cmds.push(Instruction::StateChange(s));
                seq = rest;
            } else {
                // invalid token, skip it
                seq = &seq[1..];
            }
        }
        Ok(InstructionSet(cmds))
    }
}

/// Try to parse a "mul(a, b)" token from the input sequence.
///
/// e.g, "mul(2, 3)" -> (2, 3)
fn parse_multiply(seq: &str) -> Option<((u32, u32), &str)> {
    let input = seq.strip_prefix("mul(")?;
    let (left, input) = input.split_once(",")?;
    let a = left.parse().ok()?;
    let (right, input) = input.split_once(")")?;
    let b = right.parse().ok()?;
    Some(((a, b), input))
}

/// Try to parse a "do()" or "don't()" token from the input sequence.
///
/// e.g, "do()" -> State::Enabled
/// e.g, "don't()" -> State::Disabled
fn parse_state_change(seq: &str) -> Option<(State, &str)> {
    seq.strip_prefix("do()")
        .map(|rest| (State::Enabled, rest))
        .or_else(|| {
            seq.strip_prefix("don't()")
                .map(|rest| (State::Disabled, rest))
        })
}

impl InstructionSet {
    fn evaluate(&self) -> Vec<u32> {
        self.0
            .iter()
            .map(|t| match t {
                Instruction::Multiply(a, b) => a * b,
                _ => 0,
            })
            .collect()
    }

    fn evaluate_with_state(&self) -> Vec<u32> {
        let mut state = State::Enabled;
        let mut v = vec![];
        for token in &self.0 {
            match token {
                Instruction::Multiply(a, b) if state == State::Enabled => {
                    v.push(a * b);
                }
                Instruction::StateChange(s) => state = *s,
                _ => {}
            }
        }
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(
            183380722,
            std::fs::read_to_string("input/day03.txt")
                .unwrap()
                .parse::<InstructionSet>()
                .unwrap()
                .evaluate()
                .into_iter()
                .sum::<u32>()
        )
    }

    #[test]
    fn part2() {
        assert_eq!(
            82733683,
            std::fs::read_to_string("input/day03.txt")
                .unwrap()
                .parse::<InstructionSet>()
                .unwrap()
                .evaluate_with_state()
                .into_iter()
                .sum::<u32>()
        )
    }
}
