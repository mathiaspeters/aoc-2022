pub fn day11() {
    println!("Result 11-1: {}", part1());
    println!("Result 11-2: {}", part2());
}

pub fn part1() -> usize {
    process(20, true)
}

pub fn part2() -> usize {
    process(10000, false)
}

fn process(num_rounds: usize, relief: bool) -> usize {
    let mut monkeys: Vec<Monkey> = raw_input()
        .split("\n\n")
        .map(|monkey| Monkey::new(monkey))
        .collect::<Vec<_>>();
    let num_monkeys = monkeys.len();
    let mut lcm = 1;
    monkeys.iter().for_each(|m| lcm *= m.throw.if_val);
    for _ in 0..num_rounds {
        for monkey in 0..num_monkeys {
            let items = monkeys[monkey].inspect_and_throw(relief);
            items.into_iter().for_each(|(m, s)| {
                monkeys[m].items.push(s % lcm);
            });
        }
    }
    monkeys.sort_unstable_by_key(|m| m.inspected);
    monkeys[monkeys.len() - 1].inspected * monkeys[monkeys.len() - 2].inspected
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u128>,
    modifier: StressModifier,
    throw: MonkeyDecider,
    inspected: usize,
}

impl Monkey {
    fn new(m: &str) -> Self {
        let mut lines = m.lines();
        lines.next().unwrap();
        let items = lines
            .next()
            .unwrap()
            .split(": ")
            .nth(1)
            .unwrap()
            .split(", ")
            .map(|i| i.parse::<u128>().unwrap())
            .collect::<Vec<_>>();
        let modifier = StressModifier::new(lines.next().unwrap());
        let throw = MonkeyDecider::new(lines);
        Monkey {
            items,
            modifier,
            throw,
            inspected: 0,
        }
    }

    fn inspect_and_throw(&mut self, relief: bool) -> Vec<(usize, u128)> {
        let mut holding = vec![];
        std::mem::swap(&mut holding, &mut self.items);
        holding
            .into_iter()
            .map(|stress| {
                self.inspected += 1;
                let stress = stress as u128;
                let new_stress = match self.modifier {
                    StressModifier::Add(val) => stress.saturating_add(val as u128),
                    StressModifier::Multiply(val) => stress.saturating_mul(val as u128),
                    StressModifier::Square => stress.saturating_mul(stress),
                };
                let new_stress = if relief {
                    (new_stress as f64 / 3.0) as u128
                } else {
                    new_stress as u128
                };
                let monkey = self.throw.decide(new_stress);
                (monkey, new_stress)
            })
            .collect::<Vec<_>>()
    }
}

#[derive(Debug)]

enum StressModifier {
    Add(usize),
    Multiply(usize),
    Square,
}

impl StressModifier {
    fn new(line: &str) -> Self {
        let mut parts = line.split_ascii_whitespace().rev();
        let value = parts.next().unwrap();
        let modifier = parts.next().unwrap();
        match (modifier, value) {
            ("+", _) => Self::Add(value.parse::<usize>().unwrap()),
            (_, "old") => Self::Square,
            ("*", _) => Self::Multiply(value.parse::<usize>().unwrap()),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]

struct MonkeyDecider {
    if_val: u128,
    then_monkey: usize,
    else_monkey: usize,
}

impl MonkeyDecider {
    fn new(lines: std::str::Lines) -> Self {
        let values = lines
            .take(3)
            .map(|line| {
                line.split_ascii_whitespace()
                    .last()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap()
            })
            .collect::<Vec<_>>();
        Self {
            if_val: values[0] as u128,
            then_monkey: values[1],
            else_monkey: values[2],
        }
    }

    fn decide(&self, stress: u128) -> usize {
        if stress % self.if_val == 0 {
            self.then_monkey
        } else {
            self.else_monkey
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
        assert_eq!(10605, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(2713310158, part2());
    }
}
