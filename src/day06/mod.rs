pub fn day6() {
    println!("Result  6-1: {}", part1());
    println!("Result  6-2: {}", part2());
}

pub fn part1() -> usize {
    let input = raw_input().chars().collect::<Vec<_>>();
    for i in 3..input.len() {
        if all_different(&input[i - 3..=i]) {
            return i + 1;
        }
    }
    panic!("No result");
}

pub fn part2() -> usize {
    let input = raw_input().chars().collect::<Vec<_>>();
    for i in 13..input.len() {
        if all_different(&input[i - 13..=i]) {
            return i + 1;
        }
    }
    panic!("No result");
}

fn all_different(s: &[char]) -> bool {
    for i in 0..s.len() {
        for j in i + 1..s.len() {
            if s[i] == s[j] {
                return false;
            }
        }
    }
    true
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
        assert_eq!(7, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(19, part2());
    }
}
