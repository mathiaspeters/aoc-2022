use std::collections::HashMap;

pub fn day21() {
    println!("Result 21-1: {}", part1());
    println!("Result 21-2: {}", part2());
}

pub fn part1() -> usize {
    let (dictionary, mut monkeys) = init();
    let (result, _) = solve(dictionary["root"], &mut monkeys, &dictionary);
    result
}

pub fn part2() -> usize {
    let (dictionary, mut monkeys) = init();
    monkeys[dictionary["humn"]] = Monkey::Literal(301);
    let Monkey::Calculation(_, left, right) = monkeys[dictionary["root"]] else {panic!("Incorrect root")};
    let (l_result, l_has_humn) = solve(left, &mut monkeys, &dictionary);
    let (r_result, _) = solve(right, &mut monkeys, &dictionary);
    dbg!(l_result, r_result);
    if l_has_humn {
        r_result - l_result
    } else {
        l_result - r_result
    }
}

fn init() -> (HashMap<&'static str, usize>, Vec<Monkey>) {
    let dictionary = raw_input()
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let name = line.split(": ").next().unwrap();
            (name, i)
        })
        .collect::<HashMap<&'static str, usize>>();
    let monkeys = raw_input()
        .lines()
        .map(|line| {
            let definition = line.split(": ").nth(1).unwrap();
            let parts = definition.split_ascii_whitespace().collect::<Vec<_>>();
            if parts.len() == 3 {
                let operation = match parts[1] {
                    "+" => Operation::Add,
                    "-" => Operation::Subtract,
                    "*" => Operation::Multiply,
                    "/" => Operation::Divide,
                    _ => panic!("Unknown operation"),
                };
                Monkey::Calculation(operation, dictionary[parts[0]], dictionary[parts[2]])
            } else {
                Monkey::Literal(parts[0].parse::<isize>().unwrap())
            }
        })
        .collect::<Vec<_>>();
    (dictionary, monkeys)
}

fn solve(
    key: usize,
    monkeys: &mut Vec<Monkey>,
    dictionary: &HashMap<&str, usize>,
) -> (usize, bool) {
    let mut has_humn = false;
    let humn_index = dictionary["humn"];
    let mut to_solve = vec![key];
    while !to_solve.is_empty() {
        let curr = to_solve[to_solve.len() - 1];
        has_humn = has_humn || curr == humn_index;
        match monkeys[curr] {
            Monkey::Literal(_) => {
                to_solve.pop();
            }
            Monkey::Calculation(op, left, right) => {
                has_humn = has_humn || left == humn_index;
                has_humn = has_humn || right == humn_index;
                match (monkeys[left], monkeys[right]) {
                    (Monkey::Literal(lval), Monkey::Literal(rval)) => {
                        monkeys[curr] = Monkey::Literal(match op {
                            Operation::Add => lval + rval,
                            Operation::Subtract => lval - rval,
                            Operation::Multiply => lval * rval,
                            Operation::Divide => lval / rval,
                        });
                    }
                    (Monkey::Literal(_), Monkey::Calculation(_, _, _)) => to_solve.push(right),
                    (Monkey::Calculation(_, _, _), Monkey::Literal(_)) => to_solve.push(left),
                    (Monkey::Calculation(_, _, _), Monkey::Calculation(_, _, _)) => {
                        to_solve.push(left);
                        to_solve.push(right);
                    }
                }
            }
        }
    }
    if let Monkey::Literal(value) = monkeys[key] {
        (value as usize, has_humn)
    } else {
        panic!("Calculation failed")
    }
}

#[derive(Copy, Clone, Debug)]
enum Monkey {
    Literal(isize),
    Calculation(Operation, usize, usize),
}

#[derive(Copy, Clone, Debug)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
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
        assert_eq!(152, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(301, part2());
    }
}
