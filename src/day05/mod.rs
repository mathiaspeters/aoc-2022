pub fn day5() {
    println!("Result  5-1: {}", part1());
    println!("Result  5-2: {}", part2());
}

pub fn part1() -> String {
    let mut split = raw_input().split("\r\n\r\n");
    let mut stacks = Stacks::build(split.next().unwrap());
    split
        .next()
        .unwrap()
        .lines()
        .for_each(|line| stacks.do_action(line, false));
    stacks.get_top_crates()
}

pub fn part2() -> String {
    let mut split = raw_input().split("\r\n\r\n");
    let mut stacks = Stacks::build(split.next().unwrap());
    split
        .next()
        .unwrap()
        .lines()
        .for_each(|line| stacks.do_action(line, true));
    stacks.get_top_crates()
}

#[derive(Debug)]
struct Stacks {
    stacks: Vec<Stack>,
}

impl Stacks {
    fn build(stacks: &str) -> Self {
        let lines = stacks.lines().rev().collect::<Vec<_>>();
        let stack_count = lines[0]
            .split_ascii_whitespace()
            .rev()
            .filter(|s| !s.is_empty())
            .next()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let mut stacks = vec![vec![]; stack_count];
        for (i, c) in lines[0].chars().enumerate() {
            if c.is_numeric() {
                let column = c.to_digit(10).unwrap() as usize - 1;
                for line in lines.iter().skip(1) {
                    let cc = line.chars().nth(i).unwrap();
                    if cc.is_alphabetic() {
                        stacks[column].push(cc);
                    }
                }
            }
        }
        let stacks = stacks
            .into_iter()
            .map(|s| Stack { items: s })
            .collect::<Vec<_>>();
        Self { stacks }
    }

    fn do_action(&mut self, action: &str, move_multiple: bool) {
        let mut split = action.split_ascii_whitespace();
        let amount = split.nth(1).unwrap().parse::<usize>().unwrap();
        let from = split.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        let to = split.nth(1).unwrap().parse::<usize>().unwrap() - 1;

        if move_multiple {
            let cs = self.stacks[from].take_multiple(amount);
            self.stacks[to].put_multiple(cs);
        } else {
            for _ in 0..amount {
                let c = self.stacks[from].take();
                self.stacks[to].put(c);
            }
        }
    }

    fn get_top_crates(&self) -> String {
        let mut res = String::new();
        for stack in &self.stacks {
            res.push(stack.get());
        }
        res
    }
}

#[derive(Debug)]
struct Stack {
    items: Vec<char>,
}

impl Stack {
    fn get(&self) -> char {
        *self.items.last().unwrap()
    }

    fn take(&mut self) -> char {
        self.items.pop().unwrap()
    }

    fn take_multiple(&mut self, amount: usize) -> Vec<char> {
        let mut res = Vec::with_capacity(amount);
        for _ in 0..amount {
            res.insert(0, self.items.pop().unwrap());
        }
        res
    }

    fn put(&mut self, item: char) {
        self.items.push(item);
    }

    fn put_multiple(&mut self, items: Vec<char>) {
        self.items.extend_from_slice(&items);
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
        assert_eq!("CMZ", part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!("MCD", part2());
    }
}
