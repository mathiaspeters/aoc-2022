pub fn day23() {
    println!("Result 23-1: {}", part1());
    println!("Result 23-2: {}", part2());
}

pub fn part1() -> usize {
    raw_input().len()
}

pub fn part2() -> usize {
    raw_input().len()
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
        assert_eq!(110, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2());
    }
}
