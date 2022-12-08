pub fn day8() {
    println!("Result  8-1: {}", part1());
    println!("Result  8-2: {}", part2());
}

pub fn part1() -> usize {
    let trees = raw_input()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let width = trees[0].len();
    let height = trees.len();
    let mut visible = (2 * width) + (2 * height) - 4;
    for row in 1..height - 1 {
        'outer: for column in 1..width - 1 {
            let candidate = trees[row][column];
            let mut is_visible = true;
            'curr: for x in 0..row {
                if candidate <= trees[x][column] {
                    is_visible = false;
                    break 'curr;
                }
            }
            if is_visible {
                visible += 1;
                continue 'outer;
            }
            is_visible = true;
            'curr: for x in row + 1..height {
                if candidate <= trees[x][column] {
                    is_visible = false;
                    break 'curr;
                }
            }
            if is_visible {
                visible += 1;
                continue 'outer;
            }
            is_visible = true;
            'curr: for x in 0..column {
                if candidate <= trees[row][x] {
                    is_visible = false;
                    break 'curr;
                }
            }
            if is_visible {
                visible += 1;
                continue 'outer;
            }
            is_visible = true;
            'curr: for x in column + 1..width {
                if candidate <= trees[row][x] {
                    is_visible = false;
                    break 'curr;
                }
            }
            if is_visible {
                visible += 1;
                continue 'outer;
            }
        }
    }
    visible
}

pub fn part2() -> usize {
    let trees = raw_input()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let width = trees[0].len();
    let height = trees.len();
    let mut max_score = 0;
    for row in 1..height - 1 {
        for column in 1..width - 1 {
            let candidate = trees[row][column];
            let mut total = 1;
            'curr: for (i, x) in (0..row).rev().enumerate() {
                if candidate <= trees[x][column] || x == 0 {
                    total *= i + 1;
                    break 'curr;
                }
            }
            'curr: for (i, x) in (row + 1..height).enumerate() {
                if candidate <= trees[x][column] || x == height - 1 {
                    total *= i + 1;
                    break 'curr;
                }
            }
            'curr: for (i, x) in (0..column).rev().enumerate() {
                if candidate <= trees[row][x] || x == 0 {
                    total *= i + 1;
                    break 'curr;
                }
            }
            'curr: for (i, x) in (column + 1..width).enumerate() {
                if candidate <= trees[row][x] || x == width - 1 {
                    total *= i + 1;
                    break 'curr;
                }
            }
            if total > max_score {
                max_score = total;
            }
        }
    }
    max_score
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
        assert_eq!(21, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(8, part2());
    }
}
