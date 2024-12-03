use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, u32},
    combinator::{iterator, map},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

#[derive(Clone, Copy, PartialEq)]
enum State {
    Enabled,
    Disabled,
}

struct InstructionSet(Vec<Instruction>);

enum Instruction {
    Multiply((u32, u32)),
    StateChange(State),
}

impl std::str::FromStr for InstructionSet {
    type Err = ();
    fn from_str(seq: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            iterator(seq, corrupted_instruction).flatten().collect(),
        ))
    }
}

/// Try to parse a "mul(a, b)" token from the input sequence.
///
/// e.g, "mul(2, 3)" -> (2, 3)
fn multiply(seq: &str) -> IResult<&str, (u32, u32)> {
    delimited(tag("mul("), separated_pair(u32, tag(","), u32), tag(")")).parse(seq)
}

/// Try to parse a "do()" or "don't()" token from the input sequence.
///
/// e.g, "do()" -> State::Enabled
/// e.g, "don't()" -> State::Disabled
fn state_change(seq: &str) -> IResult<&str, State> {
    alt((
        map(tag("do()"), |_| State::Enabled),
        map(tag("don't()"), |_| State::Disabled),
    ))
    .parse(seq)
}

/// Try to parse an instruction from the input sequence.
fn instruction(seq: &str) -> IResult<&str, Instruction> {
    alt((
        map(multiply, Instruction::Multiply),
        map(state_change, Instruction::StateChange),
    ))
    .parse(seq)
}

/// Try to parse a corrupted instruction from the input sequence.
fn corrupted_instruction(seq: &str) -> IResult<&str, Option<Instruction>> {
    alt((map(instruction, Some), map(anychar, |_| None))).parse(seq)
}

impl InstructionSet {
    fn evaluate(&self) -> Vec<u32> {
        self.0
            .iter()
            .filter_map(|t| match t {
                Instruction::Multiply((a, b)) => Some(a * b),
                _ => None,
            })
            .collect()
    }
    fn evaluate_with_state(&self) -> Vec<u32> {
        let mut state = State::Enabled;
        self.0
            .iter()
            .filter_map(|t| match t {
                Instruction::Multiply((a, b)) if state == State::Enabled => Some(a * b),
                Instruction::StateChange(s) => {
                    state = *s;
                    None
                }
                _ => None,
            })
            .collect()
    }
}

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
