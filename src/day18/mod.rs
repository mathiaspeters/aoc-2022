use std::collections::HashMap;

pub fn day18() {
    println!("Result 18-1: {}", part1());
    println!("Result 18-2: {}", part2());
}

pub fn part1() -> usize {
    let mut cubes = raw_input()
        .lines()
        .map(|line| {
            let mut coords = line.split(',').map(|p| p.parse::<u8>().unwrap());
            Cube::new(
                coords.next().unwrap(),
                coords.next().unwrap(),
                coords.next().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    let cube_count = cubes.len();
    for i in 0..cube_count {
        for j in i + 1..cube_count {
            let mut other = cubes[j];
            cubes[i].update(&mut other);
            cubes[j] = other;
        }
    }
    cubes.into_iter().map(|c| c.sides.count_ones()).sum::<u32>() as usize
}

pub fn part2() -> usize {
    let mut cubes = raw_input()
        .lines()
        .map(|line| {
            let mut coords = line.split(',').map(|p| p.parse::<u8>().unwrap());
            Cube::new(
                coords.next().unwrap(),
                coords.next().unwrap(),
                coords.next().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    let cube_count = cubes.len();
    for i in 0..cube_count {
        for j in i + 1..cube_count {
            let mut other = cubes[j];
            cubes[i].update(&mut other);
            cubes[j] = other;
        }
    }
    let mut adjacent = HashMap::new();
    cubes.iter().for_each(|c| c.get_adjacent(&mut adjacent));
    let adjacent = adjacent
        .into_iter()
        .filter(|((x, y, z), v)| *v >= 5 && !cubes.contains(&Cube::new(*x, *y, *z)))
        .collect::<Vec<_>>();
    dbg!(adjacent);
    cubes.into_iter().map(|c| c.sides.count_ones()).sum::<u32>() as usize
}

#[derive(Copy, Clone, Debug, Eq)]
struct Cube {
    x: u8,
    y: u8,
    z: u8,
    sides: u8,
}

impl PartialEq for Cube {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Cube {
    fn new(x: u8, y: u8, z: u8) -> Self {
        Self {
            x,
            y,
            z,
            sides: 0b00111111,
        }
    }

    fn get_adjacent(&self, all: &mut HashMap<(u8, u8, u8), u8>) {
        let top = 0b00000001;
        let bottom = 0b00000010;
        let front = 0b00000100;
        let back = 0b00001000;
        let left = 0b00010000;
        let right = 0b00100000;

        if self.sides & top > 0 {
            *all.entry((self.x, self.y, self.z + 1)).or_insert(0) += 1;
        }
        if self.sides & bottom > 0 {
            *all.entry((self.x, self.y, self.z - 1)).or_insert(0) += 1;
        }

        if self.sides & front > 0 {
            *all.entry((self.x, self.y + 1, self.z)).or_insert(0) += 1;
        }
        if self.sides & back > 0 {
            *all.entry((self.x, self.y - 1, self.z)).or_insert(0) += 1;
        }

        if self.sides & left > 0 {
            *all.entry((self.x + 1, self.y, self.z)).or_insert(0) += 1;
        }
        if self.sides & right > 0 {
            *all.entry((self.x - 1, self.y, self.z)).or_insert(0) += 1;
        }
    }

    fn update(&mut self, other: &mut Cube) {
        let top = 0b00111110;
        let bottom = 0b00111101;
        let front = 0b00111011;
        let back = 0b00110111;
        let left = 0b00101111;
        let right = 0b00011111;
        if self.x == other.x && self.y == other.y {
            if self.z == other.z {
                self.sides &= 0;
                other.sides &= 0;
            } else if self.z == other.z + 1 {
                self.sides &= top;
                other.sides &= bottom;
            } else if self.z == other.z - 1 {
                self.sides &= bottom;
                other.sides &= top;
            }
        } else if self.z == other.z && self.y == other.y {
            if self.x == other.x {
                self.sides &= 0;
                other.sides &= 0;
            } else if self.x == other.x + 1 {
                self.sides &= left;
                other.sides &= right;
            } else if self.x == other.x - 1 {
                self.sides &= right;
                other.sides &= left;
            }
        } else {
            if self.x == other.x && self.z == other.z {
                if self.y == other.y {
                    self.sides &= 0;
                    other.sides &= 0;
                } else if self.y == other.y + 1 {
                    self.sides &= front;
                    other.sides &= back;
                } else if self.y == other.y - 1 {
                    self.sides &= back;
                    other.sides &= front;
                }
            }
        }
    }
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
        assert_eq!(64, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(58, part2());
    }
}
