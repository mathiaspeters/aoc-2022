use std::collections::HashMap;

pub fn day7() {
    println!("Result  7-1: {}", part1());
    println!("Result  7-2: {}", part2());
}

pub fn part1() -> usize {
    let sizes = get_folder_sizes();
    sizes
        .into_iter()
        .filter(|v| *v <= MAX_FOLDER_SIZE)
        .sum::<usize>()
}

pub fn part2() -> usize {
    let mut sizes = get_folder_sizes();
    sizes.sort_unstable();
    let free = TOTAL_SPACE - sizes.last().unwrap();
    let needed = NEEDED_SPACE - free;
    for size in sizes {
        if size > needed {
            return size;
        }
    }
    panic!()
}

fn get_folder_sizes() -> Vec<usize> {
    let mut sizes = HashMap::new();
    let mut open_dirs = vec!["/"];
    raw_input().lines().for_each(|line| {
        if let Some(directory) = line.strip_prefix("$ cd ") {
            if directory == ".." {
                open_dirs.pop();
            } else if directory == "/" {
                open_dirs.clear();
            } else {
                open_dirs.push(directory);
            }
            if open_dirs.is_empty() {
                open_dirs.push("/");
            }
        } else if line == "$ ls" {
            // do nothing
        } else if let Some(_) = line.strip_prefix("dir ") {
            // do nothing
        } else {
            let size = line
                .split_ascii_whitespace()
                .next()
                .unwrap()
                .parse::<usize>()
                .unwrap();
            for i in 0..open_dirs.len() {
                let key = open_dirs[..i + 1].join("/");
                *sizes.entry(key).or_insert_with(|| 0) += size;
            }
        }
    });
    sizes.into_values().collect()
}

const MAX_FOLDER_SIZE: usize = 100_000;
const TOTAL_SPACE: usize = 70_000_000;
const NEEDED_SPACE: usize = 30_000_000;

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
        assert_eq!(95437, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(24933642, part2());
    }
}
