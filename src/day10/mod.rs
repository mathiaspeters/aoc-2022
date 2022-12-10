pub fn day10() {
    println!("Result 10-1: {}", part1());
    println!("Result 10-2: \n{}", part2());
}

pub fn part1() -> isize {
    let mut cpu = CPU::new(vec![20, 60, 100, 140, 180, 220]);
    raw_input().lines().for_each(|line| cpu.instr(line));
    cpu.signal_sum
}

pub fn part2() -> String {
    let mut crt = CRT::new();
    raw_input().lines().for_each(|line| crt.instr(line));
    crt.show()
}

struct CPU {
    checkpoints: Vec<isize>,
    signal_sum: isize,
    register: isize,
    cycle: isize,
}

impl CPU {
    fn new(checkpoints: Vec<isize>) -> Self {
        Self {
            checkpoints,
            signal_sum: 0,
            register: 1,
            cycle: 0,
        }
    }

    fn instr(&mut self, instr: &str) {
        if let Some(val) = instr.strip_prefix("addx ") {
            self.instr_impl(0);
            let val = val.parse::<isize>().unwrap();
            self.instr_impl(val);
        } else {
            self.instr_impl(0);
        }
    }

    fn instr_impl(&mut self, val: isize) {
        self.cycle += 1;
        if self.checkpoints.contains(&self.cycle) {
            self.signal_sum += self.cycle * self.register;
        }
        self.register += val;
    }
}

struct CRT {
    pixels: Vec<Vec<bool>>,
    row: usize,
    column: usize,
    sprite_pos: isize,
}

impl CRT {
    fn new() -> Self {
        Self {
            pixels: vec![vec![false; 40]; 6],
            row: 0,
            column: 0,
            sprite_pos: 1,
        }
    }

    fn instr(&mut self, instr: &str) {
        if let Some(val) = instr.strip_prefix("addx ") {
            self.instr_impl(0);
            let val = val.parse::<isize>().unwrap();
            self.instr_impl(val);
        } else {
            self.instr_impl(0);
        }
    }

    fn instr_impl(&mut self, val: isize) {
        if self.column as isize >= self.sprite_pos - 1
            && self.column as isize <= self.sprite_pos + 1
        {
            self.pixels[self.row][self.column] = true;
        }
        self.column += 1;
        if self.column == 40 {
            self.column = 0;
            self.row += 1;
        }
        self.sprite_pos += val;
    }

    fn show(&self) -> String {
        self.pixels
            .iter()
            .map(|row| {
                row.iter()
                    .map(|b| if *b { '#' } else { '.' })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
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
        assert_eq!(13140, part1());
    }

    #[test]
    fn test_part2() {
        let expected = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
            .to_owned();
        assert_eq!(expected, part2());
    }
}
