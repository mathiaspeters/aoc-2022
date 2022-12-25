use std::collections::HashSet;

pub fn day14() {
    println!("Result 14-1: {}", part1());
    println!("Result 14-2: {}", part2());
}

pub fn part1() -> usize {
    let (mut occupied, sand_source) = get_occupied();
    let mut sand_count = 0;
    let mut path = vec![sand_source];

    let min_x = *occupied
        .iter()
        .min_by_key(|(x, _, _)| x)
        .map(|(x, _, _)| x)
        .unwrap();
    let max_x = *occupied
        .iter()
        .max_by_key(|(x, _, _)| x)
        .map(|(x, _, _)| x)
        .unwrap();
    let max_y = *occupied
        .iter()
        .max_by_key(|(_, y, _)| y)
        .map(|(_, y, _)| y)
        .unwrap();

    loop {
        let mut pos = path.pop().unwrap();
        let mut searching = true;
        while searching {
            if !occupied.iter().any(|p1| p1.0 == pos.0 && p1.1 == pos.1 + 1) {
                path.push(pos);
                pos = (pos.0, pos.1 + 1);
            } else if !occupied
                .iter()
                .any(|p1| p1.0 == pos.0 - 1 && p1.1 == pos.1 + 1)
            {
                path.push(pos);
                pos = (pos.0 - 1, pos.1 + 1);
            } else if !occupied
                .iter()
                .any(|p1| p1.0 == pos.0 + 1 && p1.1 == pos.1 + 1)
            {
                path.push(pos);
                pos = (pos.0 + 1, pos.1 + 1);
            } else {
                searching = false;
                path.pop();
            }
            if pos.0 < min_x || pos.0 > max_x || pos.1 > max_y {
                return sand_count;
            }
        }
        occupied.insert((pos.0, pos.1, true));
        sand_count += 1;
    }
}

pub fn part2() -> usize {
    let (mut occupied, sand_source) = get_occupied();
    let mut sand_count = 0;
    let mut path = vec![sand_source];

    let max_y = *occupied
        .iter()
        .max_by_key(|(_, y, _)| y)
        .map(|(_, y, _)| y)
        .unwrap();

    loop {
        let mut pos = path.pop().unwrap_or(sand_source);
        let mut searching = true;
        while searching {
            if pos.1 + 1 == max_y + 2 {
                searching = false;
                path.pop();
            } else if !occupied.iter().any(|p1| p1.0 == pos.0 && p1.1 == pos.1 + 1) {
                path.push(pos);
                pos = (pos.0, pos.1 + 1);
            } else if !occupied
                .iter()
                .any(|p1| p1.0 == pos.0 - 1 && p1.1 == pos.1 + 1)
            {
                path.push(pos);
                pos = (pos.0 - 1, pos.1 + 1);
            } else if !occupied
                .iter()
                .any(|p1| p1.0 == pos.0 + 1 && p1.1 == pos.1 + 1)
            {
                path.push(pos);
                pos = (pos.0 + 1, pos.1 + 1);
            } else {
                searching = false;
                path.pop();
            }
        }
        occupied.insert((pos.0, pos.1, true));
        sand_count += 1;
        if pos.0 == 500 && pos.1 == 0 {
            print_occupied(&occupied);
            return sand_count;
        }
    }
}

fn get_occupied() -> (HashSet<(i32, i32, bool)>, (i32, i32)) {
    let coords = raw_input()
        .lines()
        .flat_map(|line| {
            let coords = line
                .split(" -> ")
                .map(|c| {
                    let mut xy = c.split(',').map(|c| c.parse::<i32>().unwrap());
                    (xy.next().unwrap(), xy.next().unwrap())
                })
                .collect::<Vec<_>>();
            coords
                .windows(2)
                .flat_map(|w| {
                    let (x1, y1) = w[0];
                    let (x2, y2) = w[1];
                    if x1 == x2 {
                        (y1.min(y2)..=y2.max(y1))
                            .map(|y| (x1, y, false))
                            .collect::<Vec<_>>()
                    } else {
                        (x1.min(x2)..=x2.max(x1))
                            .map(|x| (x, y1, false))
                            .collect::<Vec<_>>()
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<HashSet<_>>();
    let sand_source = (500, 0);
    (coords, sand_source)
}

#[derive(Copy, Clone)]
enum Cell {
    Air,
    Rock,
    Sand,
}

fn print_grid(grid: &[Vec<Cell>]) {
    let grid_str = grid
        .iter()
        .map(|row| {
            row.iter()
                .map(|cell| match cell {
                    Cell::Air => '.',
                    Cell::Rock => '#',
                    Cell::Sand => 'o',
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n");
    println!("{grid_str}");
}

fn print_occupied(occupied: &HashSet<(i32, i32, bool)>) {
    let min_x = *occupied
        .iter()
        .min_by_key(|(x, _, _)| x)
        .map(|(x, _, _)| x)
        .unwrap();
    let max_x = *occupied
        .iter()
        .max_by_key(|(x, _, _)| x)
        .map(|(x, _, _)| x)
        .unwrap();
    let max_x = max_x - min_x;
    let max_y = *occupied
        .iter()
        .max_by_key(|(_, y, _)| y)
        .map(|(_, y, _)| y)
        .unwrap();

    let mut grid = vec![vec![Cell::Air; 1 + max_x as usize]; 1 + max_y as usize];

    occupied.iter().for_each(|(x, y, is_sand)| {
        grid[*y as usize][(x - min_x) as usize] = if *is_sand { Cell::Sand } else { Cell::Rock };
    });

    print_grid(&grid);
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
        assert_eq!(24, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(93, part2());
    }
}
