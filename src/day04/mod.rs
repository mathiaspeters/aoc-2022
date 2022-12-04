pub fn day4() {
    println!("Result  4-1: {}", part1());
    println!("Result  4-2: {}", part2());
}

pub fn part1() -> usize {
    raw_input()
        .lines()
        .filter(|line| {
            let rc = line
                .split(['-', ','])
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            (rc[0] <= rc[2] && rc[1] >= rc[3]) || (rc[0] >= rc[2] && rc[1] <= rc[3])
        })
        .count()
}

pub fn part2() -> usize {
    raw_input()
        .lines()
        .filter(|line| {
            let rc = line
                .split(['-', ','])
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            (rc[0] >= rc[2] && rc[0] <= rc[3])
                || (rc[1] >= rc[2] && rc[1] <= rc[3])
                || (rc[0] <= rc[2] && rc[1] >= rc[3])
        })
        .count()
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
        assert_eq!(2, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(4, part2());
    }
}
