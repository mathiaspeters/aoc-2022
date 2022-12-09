use std::collections::HashSet;

pub fn day9() {
    println!("Result  9-1: {}", part1());
    println!("Result  9-2: {}", part2());
}

pub fn part1() -> usize {
    program(2)
}

pub fn part2() -> usize {
    program(10)
}

fn program(part_count: usize) -> usize {
    let mut state_machine = StateMachine::new(part_count);
    raw_input().lines().for_each(|line| {
        let mut line = line.split_ascii_whitespace();
        let d = line.next().unwrap();
        let d = match d {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => panic!("Unknown direction"),
        };
        let steps = line.next().unwrap().parse::<u8>().unwrap();
        state_machine.make_move(d, steps);
    });
    state_machine.tail_positions.len()
}

struct StateMachine {
    parts: Vec<(i32, i32)>,
    tail_positions: HashSet<(i32, i32)>,
}

impl StateMachine {
    fn new(part_count: usize) -> Self {
        Self {
            parts: vec![(0, 0); part_count],
            tail_positions: HashSet::new(),
        }
    }

    fn make_move(&mut self, d: Direction, steps: u8) {
        for _ in 0..steps {
            self.move_head(d);
            for i in 1..self.parts.len() {
                self.move_part(i);
            }
            self.tail_positions.insert(*self.parts.last().unwrap());
        }
    }

    fn move_head(&mut self, d: Direction) {
        Self::move_pos(&mut self.parts[0], d);
    }

    fn move_part(&mut self, i: usize) {
        let x = if self.parts[i - 1].0 > self.parts[i].0 + 1 {
            RelativePosition::MoreMove
        } else if self.parts[i - 1].0 > self.parts[i].0 {
            RelativePosition::More
        } else if self.parts[i - 1].0 < self.parts[i].0 - 1 {
            RelativePosition::LessMove
        } else if self.parts[i - 1].0 < self.parts[i].0 {
            RelativePosition::Less
        } else {
            RelativePosition::Equal
        };
        let y = if self.parts[i - 1].1 > self.parts[i].1 + 1 {
            RelativePosition::MoreMove
        } else if self.parts[i - 1].1 > self.parts[i].1 {
            RelativePosition::More
        } else if self.parts[i - 1].1 < self.parts[i].1 - 1 {
            RelativePosition::LessMove
        } else if self.parts[i - 1].1 < self.parts[i].1 {
            RelativePosition::Less
        } else {
            RelativePosition::Equal
        };
        let steps = match (x, y) {
            (RelativePosition::MoreMove, RelativePosition::More)
            | (RelativePosition::MoreMove, RelativePosition::MoreMove) => {
                vec![Direction::Right, Direction::Up]
            }
            (RelativePosition::MoreMove, RelativePosition::Less)
            | (RelativePosition::MoreMove, RelativePosition::LessMove) => {
                vec![Direction::Right, Direction::Down]
            }
            (RelativePosition::MoreMove, RelativePosition::Equal) => vec![Direction::Right],
            (RelativePosition::LessMove, RelativePosition::More) => {
                vec![Direction::Left, Direction::Up]
            }
            (RelativePosition::LessMove, RelativePosition::Less) => {
                vec![Direction::Left, Direction::Down]
            }
            (RelativePosition::LessMove, RelativePosition::Equal) => vec![Direction::Left],
            (RelativePosition::More, RelativePosition::MoreMove) => {
                vec![Direction::Right, Direction::Up]
            }
            (RelativePosition::Less, RelativePosition::MoreMove)
            | (RelativePosition::LessMove, RelativePosition::MoreMove) => {
                vec![Direction::Left, Direction::Up]
            }
            (RelativePosition::Equal, RelativePosition::MoreMove) => vec![Direction::Up],
            (RelativePosition::More, RelativePosition::LessMove) => {
                vec![Direction::Right, Direction::Down]
            }
            (RelativePosition::Less, RelativePosition::LessMove)
            | (RelativePosition::LessMove, RelativePosition::LessMove) => {
                vec![Direction::Left, Direction::Down]
            }
            (RelativePosition::Equal, RelativePosition::LessMove) => vec![Direction::Down],
            (RelativePosition::More, RelativePosition::More)
            | (RelativePosition::More, RelativePosition::Less)
            | (RelativePosition::More, RelativePosition::Equal)
            | (RelativePosition::Less, RelativePosition::More)
            | (RelativePosition::Less, RelativePosition::Less)
            | (RelativePosition::Less, RelativePosition::Equal)
            | (RelativePosition::Equal, RelativePosition::More)
            | (RelativePosition::Equal, RelativePosition::Less)
            | (RelativePosition::Equal, RelativePosition::Equal) => vec![],
        };
        steps
            .into_iter()
            .for_each(|d| Self::move_pos(&mut self.parts[i], d));
    }

    fn move_pos(pos: &mut (i32, i32), d: Direction) {
        match d {
            Direction::Right => pos.0 += 1,
            Direction::Left => pos.0 -= 1,
            Direction::Up => pos.1 += 1,
            Direction::Down => pos.1 -= 1,
        }
    }
}

#[derive(Copy, Clone)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Copy, Clone)]
enum RelativePosition {
    MoreMove,
    LessMove,
    More,
    Less,
    Equal,
}

#[cfg(not(test))]
fn raw_input() -> &'static str {
    include_str!("input")
}

#[cfg(test)]
fn raw_input() -> &'static str {
    include_str!("testinput")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(13, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(1, part2());
    }
}
