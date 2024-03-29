pub fn day25() {
    println!("Result 25-1: {}", part1());
    println!("Result 25-2: {}", part2());
}

pub fn part1() -> String {
    "".to_owned()
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
        assert_eq!("2 = -1 = 0", part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2());
    }
}
