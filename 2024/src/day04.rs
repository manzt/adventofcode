#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

impl Direction {
    fn all() -> [Direction; 8] {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
            Direction::UpRight,
            Direction::UpLeft,
            Direction::DownRight,
            Direction::DownLeft,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Position {
    pub x: isize,
    pub y: isize,
}

impl Position {
    fn step(&self, distance: isize, direction: Direction) -> Position {
        match direction {
            Direction::Up => Position {
                x: self.x,
                y: self.y - distance,
            },
            Direction::Down => Position {
                x: self.x,
                y: self.y + distance,
            },
            Direction::Left => Position {
                x: self.x - distance,
                y: self.y,
            },
            Direction::Right => Position {
                x: self.x + distance,
                y: self.y,
            },
            Direction::UpRight => Position {
                x: self.x + distance,
                y: self.y - distance,
            },
            Direction::UpLeft => Position {
                x: self.x - distance,
                y: self.y - distance,
            },
            Direction::DownRight => Position {
                x: self.x + distance,
                y: self.y + distance,
            },
            Direction::DownLeft => Position {
                x: self.x - distance,
                y: self.y + distance,
            },
        }
    }
}

impl From<(usize, usize)> for Position {
    fn from((x, y): (usize, usize)) -> Self {
        Position {
            x: x as isize,
            y: y as isize,
        }
    }
}

#[derive(Debug)]
struct WordSearch {
    board: Vec<Vec<char>>,
}

impl WordSearch {
    fn get(&self, pos: Position) -> Option<&char> {
        // Make sure the position is within the grid.
        if pos.x < 0 || pos.y < 0 {
            return None;
        }
        self.board
            .get(pos.y as usize)
            .and_then(|row| row.get(pos.x as usize))
    }

    fn coordinates(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.board
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, _)| (x, y)))
    }

    fn matches_in_direction(&self, pos: Position, word: &str, direction: Direction) -> bool {
        word.chars().enumerate().all(|(i, source)| {
            match self.get(pos.step(i as isize, direction)) {
                Some(&target) => source == target,
                None => false,
            }
        })
    }

    /// Find the word in the grid and return the number of times it appears.
    pub fn count_occurrences<S: AsRef<str>>(&self, word: S) -> usize {
        self.coordinates()
            .map(|xy| {
                Direction::all()
                    .iter()
                    .filter(|direction| {
                        self.matches_in_direction(xy.into(), word.as_ref(), **direction)
                    })
                    .count()
            })
            .sum()
    }

    /// Count the number of times the word apprears in the grid in a cross pattern.
    pub fn count_cross_occurances<S: AsRef<str>>(&self, word: S) -> usize {
        self.coordinates()
            .filter(|&xy| {
                let position = xy.into();
                let step_size = word.as_ref().chars().count() as isize - 1;
                let matches: [bool; 4] = [
                    // upper-left -> lower-right
                    self.matches_in_direction(position, word.as_ref(), Direction::DownRight),
                    // upper-right -> lower-left
                    self.matches_in_direction(
                        position.step(step_size, Direction::Right),
                        word.as_ref(),
                        Direction::DownLeft,
                    ),
                    // lower-left -> upper-right
                    self.matches_in_direction(
                        position.step(step_size, Direction::Down),
                        word.as_ref(),
                        Direction::UpRight,
                    ),
                    // lower-right -> upper-left
                    self.matches_in_direction(
                        position.step(step_size, Direction::DownRight),
                        word.as_ref(),
                        Direction::UpLeft,
                    ),
                ];
                // Whether word appears in exactly two directions.
                matches.iter().filter(|m| **m).count() == 2
            })
            .count()
    }
}

impl std::str::FromStr for WordSearch {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s.lines().map(|line| line.chars().collect()).collect();
        Ok(WordSearch { board: grid })
    }
}

#[test]
fn part1() {
    assert_eq!(
        2639,
        std::fs::read_to_string("input/day04.txt")
            .unwrap()
            .parse::<WordSearch>()
            .unwrap()
            .count_occurrences("XMAS")
    );
}

#[test]
fn part2() {
    assert_eq!(
        2005,
        std::fs::read_to_string("input/day04.txt")
            .unwrap()
            .parse::<WordSearch>()
            .unwrap()
            .count_cross_occurances("MAS")
    );
}
