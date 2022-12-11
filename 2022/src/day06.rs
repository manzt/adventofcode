use std::collections::HashSet;

use anyhow::Result;

fn start_of_unique(data: &str, size: usize) -> Option<usize> {
    let mut seen: HashSet<u8> = HashSet::with_capacity(size);
    for (i, wind) in data.as_bytes().windows(size).enumerate() {
        seen.extend(wind);
        if seen.len() == size {
            return Some(i + size);
        }
        seen.clear();
    }
    None
}

fn start_of_packet(data: &str) -> Option<usize> {
    start_of_unique(data, 4)
}

fn start_of_message(data: &str) -> Option<usize> {
    let n = start_of_packet(data)?;
    let offset = start_of_unique(&data[n..], 14)?;
    Some(n + offset)
}

pub fn part1(input: String) -> Result<()> {
    let n = start_of_packet(&input).expect("No start-of-packet marker detected");
    println!("day06 part1: n characters={}", n);
    Ok(())
}

pub fn part2(input: String) -> Result<()> {
    let n = start_of_message(&input).expect("No start-of-message marker detected");
    println!("day06 part2: n characters={}", n);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_of_packet_case1() {
        assert_eq!(start_of_packet("bvwbjplbgvbhsrlpgdmjqwftvncz").unwrap(), 5);
    }

    #[test]
    fn start_of_packet_case2() {
        assert_eq!(start_of_packet("nppdvjthqldpwncqszvftbrmjlhg").unwrap(), 6);
    }

    #[test]
    fn start_of_packet_case3() {
        assert_eq!(
            start_of_packet("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").unwrap(),
            10
        );
    }

    #[test]
    fn start_of_packet_case4() {
        assert_eq!(
            start_of_packet("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").unwrap(),
            11
        );
    }

    #[test]
    fn start_of_message_case1() {
        assert_eq!(
            start_of_message("bvwbjplbgvbhsrlpgdmjqwftvncz").unwrap(),
            23
        );
    }

    #[test]
    fn start_of_message_case2() {
        assert_eq!(
            start_of_message("nppdvjthqldpwncqszvftbrmjlhg").unwrap(),
            23
        );
    }

    #[test]
    fn start_of_message_case3() {
        assert_eq!(
            start_of_message("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").unwrap(),
            29
        );
    }

    #[test]
    fn start_of_message_case4() {
        assert_eq!(
            start_of_message("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").unwrap(),
            26
        );
    }
}
