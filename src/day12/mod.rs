pub fn day12() {
    println!("Result 12-1: {}", part1());
    println!("Result 12-2: {}", part2());
}

pub fn part1() -> usize {
    let (map, end) = get_map();
    let start = map
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .enumerate()
                .find_map(|(j, col)| if *col == 0 { Some((i, j)) } else { None })
        })
        .unwrap();
    shortest_path(&map, start, end)
}

pub fn part2() -> usize {
    let (map, end) = get_map();
    let mut starts = vec![];
    map.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, col)| {
            if *col <= 1 {
                starts.push((i, j))
            }
        })
    });
    starts
        .into_iter()
        .map(|start| shortest_path(&map, start, end))
        .min()
        .unwrap()
}

fn shortest_path(map: &[Vec<u8>], start: (usize, usize), end: (usize, usize)) -> usize {
    let max_row = map.len() - 1;
    let max_col = map[0].len() - 1;
    let mut scores = vec![vec![usize::MAX; max_col + 1]; max_row + 1];
    scores[start.0][start.1] = 0;
    let mut open_nodes = vec![start];
    loop {
        let min = open_nodes
            .iter()
            .enumerate()
            .rev()
            .min_by_key(|(_, (row, col))| scores[*row][*col]);
        if min == None {
            return usize::MAX;
        }
        let min = min.unwrap().0;
        let (row, col) = open_nodes.remove(min);
        if (row, col) == end {
            return scores[row][col];
        }
        get_adjacent((row, col), max_row, max_col)
            .into_iter()
            .filter(|(ar, ac)| map[*ar][*ac] <= map[row][col].saturating_add(1))
            .for_each(|(ar, ac)| {
                let new_score = scores[row][col] + 1;
                if scores[ar][ac] == usize::MAX {
                    open_nodes.push((ar, ac));
                }
                if new_score < scores[ar][ac] {
                    scores[ar][ac] = new_score;
                }
            });
    }
}

fn get_map() -> (Vec<Vec<u8>>, (usize, usize)) {
    let mut end = (0, 0);
    let map = raw_input()
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| match c {
                    'S' => 0,
                    'E' => {
                        end = (row, col);
                        'z' as u8 - 'a' as u8 + 1
                    }
                    _ => c as u8 - 'a' as u8 + 1,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    (map, end)
}

fn get_adjacent(coords: (usize, usize), max_row: usize, max_col: usize) -> Vec<(usize, usize)> {
    let (row, col) = coords;
    let mut adjacent = Vec::with_capacity(4);
    if row > 0 {
        adjacent.push((row - 1, col));
    }
    if row < max_row {
        adjacent.push((row + 1, col));
    }
    if col > 0 {
        adjacent.push((row, col - 1));
    }
    if col < max_col {
        adjacent.push((row, col + 1));
    }
    adjacent
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
        assert_eq!(31, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(29, part2());
    }
}
