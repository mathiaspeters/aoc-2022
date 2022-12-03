pub fn day3() {
    println!("Result  3-1: {}", part1());
    println!("Result  3-2: {}", part2());
}

pub fn part1() -> usize {
    raw_input()
        .lines()
        .map(|line| (&line[..line.len() / 2], &line[line.len() / 2..]))
        .map(|(fc, sc)| fc.chars().filter(|c| sc.contains(*c)).nth(0))
        .map(|c| c.map(|c| char_to_priority(c)).unwrap_or(0))
        .sum()
}

pub fn part2() -> usize {
    let lines = raw_input().lines().collect::<Vec<_>>();
    lines
        .chunks(3)
        .map(|group| {
            group[0]
                .chars()
                .filter(|c| group[1].contains(*c) && group[2].contains(*c))
                .nth(0)
        })
        .map(|c| c.map(|c| char_to_priority(c)).unwrap_or(0))
        .sum()
}

fn char_to_priority(c: char) -> usize {
    let val = c as u8;
    let res = if val >= 97 {
        val - 96
    } else if val >= 65 {
        val - 38
    } else {
        0
    };
    res as usize
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
        assert_eq!(157, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(70, part2());
    }
}
