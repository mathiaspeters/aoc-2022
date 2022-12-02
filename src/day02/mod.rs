pub fn day2() {
    println!("Result  2-1: {}", part1());
    println!("Result  2-2: {}", part2());
}

pub fn part1() -> usize {
    raw_input()
        .lines()
        .map(|l| {
            let choices = l.split(' ').map(|c| Choice::from(c)).collect::<Vec<_>>();
            choices[1].match_score(choices[0])
        })
        .sum()
}

pub fn part2() -> usize {
    raw_input()
        .lines()
        .map(|l| {
            let s = l.split(' ').collect::<Vec<_>>();
            Outcome::from(s[1]).match_score(Choice::from(s[0]))
        })
        .sum()
}

#[derive(Copy, Clone)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl From<&str> for Outcome {
    fn from(s: &str) -> Self {
        match s {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("Unrecognized character"),
        }
    }
}

impl Outcome {
    fn match_score(&self, other: Choice) -> usize {
        let my_choice = match (self, other) {
            (Outcome::Lose, Choice::Rock)
            | (Outcome::Draw, Choice::Scissors)
            | (Outcome::Win, Choice::Paper) => Choice::Scissors,
            (Outcome::Lose, Choice::Scissors)
            | (Outcome::Draw, Choice::Paper)
            | (Outcome::Win, Choice::Rock) => Choice::Paper,
            (Outcome::Lose, Choice::Paper)
            | (Outcome::Draw, Choice::Rock)
            | (Outcome::Win, Choice::Scissors) => Choice::Rock,
        };
        my_choice.match_score(other)
    }
}

#[derive(Copy, Clone)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for Choice {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("Unrecognized character"),
        }
    }
}

impl Choice {
    fn score(&self) -> usize {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }

    fn match_score(&self, other: Self) -> usize {
        let choice_score = self.score();
        let outcome_score = match (self, other) {
            (Choice::Rock, Choice::Paper)
            | (Choice::Paper, Choice::Scissors)
            | (Choice::Scissors, Choice::Rock) => 0,
            (Choice::Rock, Choice::Rock)
            | (Choice::Paper, Choice::Paper)
            | (Choice::Scissors, Choice::Scissors) => 3,
            (Choice::Rock, Choice::Scissors)
            | (Choice::Paper, Choice::Rock)
            | (Choice::Scissors, Choice::Paper) => 6,
        };
        choice_score + outcome_score
    }
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
        assert_eq!(15, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(12, part2());
    }
}
