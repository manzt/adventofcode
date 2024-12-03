use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::anychar,
    combinator::{map, map_res},
    multi::many0,
    sequence::{preceded, tuple},
    Finish, IResult,
};

#[derive(Clone, Copy, PartialEq)]
enum State {
    Enabled,
    Disabled,
}

struct InstructionSet(Vec<Instruction>);

enum Instruction {
    Multiply(u32, u32),
    StateChange(State),
}

impl std::str::FromStr for InstructionSet {
    type Err = nom::error::Error<String>;

    fn from_str(seq: &str) -> Result<Self, Self::Err> {
        map(
            many0(alt((map(parse_instruction, Some), map(anychar, |_| None)))),
            |v| v.into_iter().flatten().collect(),
        )(seq)
        .map_err(|e| e.to_owned())
        .finish()
        .map(|(_, v)| Self(v))
    }
}

/// Try to parse an integer from the input sequence.
///
/// e.g, "123" -> 123
fn parse_integer(seq: &str) -> IResult<&str, u32> {
    map_res(take_while(|c: char| c.is_ascii_digit()), |s: &str| {
        s.parse::<u32>()
    })(seq)
}

/// Try to parse a "mul(a, b)" token from the input sequence.
///
/// e.g, "mul(2, 3)" -> (2, 3)
fn parse_multiply(seq: &str) -> IResult<&str, (u32, u32)> {
    tuple((
        tag("mul("),
        parse_integer,
        preceded(tag(","), parse_integer),
        tag(")"),
    ))(seq)
    .map(|(s, (_, a, b, _))| (s, (a, b)))
}

/// Try to parse a "do()" or "don't()" token from the input sequence.
///
/// e.g, "do()" -> State::Enabled
/// e.g, "don't()" -> State::Disabled
fn parse_state_change(seq: &str) -> IResult<&str, State> {
    alt((
        map(tag("do()"), |_| State::Enabled),
        map(tag("don't()"), |_| State::Disabled),
    ))(seq)
}

/// Try to parse an instruction from the input sequence.
fn parse_instruction(seq: &str) -> IResult<&str, Instruction> {
    alt((
        map(parse_multiply, |(a, b)| Instruction::Multiply(a, b)),
        map(parse_state_change, Instruction::StateChange),
    ))(seq)
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
