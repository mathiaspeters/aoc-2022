pub fn day1() {
    println!("Result  1-1: {}", part1());
    println!("Result  1-2: {}", part2());
}

pub fn part1() -> usize {
    get_elves(1)[0]
}

pub fn part2() -> usize {
    get_elves(3).into_iter().sum()
}

fn get_elves(to_return: usize) -> Vec<usize> {
    let mut elves = Vec::new();
    let mut current_elf = 0;
    raw_input().lines().for_each(|line| {
        if line.is_empty() {
            elves.push(current_elf);
            current_elf = 0;
        } else {
            current_elf += line.parse::<usize>().unwrap();
        }
    });
    elves.push(current_elf);
    elves.sort_unstable();
    elves.into_iter().rev().take(to_return).collect::<Vec<_>>()
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
        assert_eq!(24000, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(45000, part2());
    }
}
