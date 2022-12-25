pub fn day13() {
    println!("Result 13-1: {}", part1());
    println!("Result 13-2: {}", part2());
}

pub fn part1() -> usize {
    raw_input()
        .split("\n\n")
        .map(|packets| PacketPair::new(packets.lines()))
        .enumerate()
        .filter(|(_, packets)| packets.is_right_order())
        .map(|(i, _)| i + 1)
        .sum()
}

pub fn part2() -> usize {
    let mut packets = raw_input()
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| Packet::new(line))
        .collect::<Vec<_>>();
    packets.push(Packet::new_divider("[[2]]"));
    packets.push(Packet::new_divider("[[6]]"));
    packets.sort_unstable_by(|p1, p2| {
        let right_order = p1.is_right_order(p2);
        match right_order {
            Some(true) => std::cmp::Ordering::Less,
            Some(false) => std::cmp::Ordering::Greater,
            None => std::cmp::Ordering::Equal,
        }
    });
    let mut indices = packets
        .into_iter()
        .enumerate()
        .filter(|(_, packet)| packet.is_divider)
        .map(|(i, _)| i + 1);
    indices.next().unwrap() * indices.next().unwrap()
}

#[derive(Debug)]
struct PacketPair {
    left: Packet,
    right: Packet,
}

impl PacketPair {
    fn new(mut lines: std::str::Lines) -> Self {
        Self {
            left: Packet::new(lines.next().unwrap()),
            right: Packet::new(lines.next().unwrap()),
        }
    }

    fn is_right_order(&self) -> bool {
        self.left.is_right_order(&self.right).unwrap_or(true)
    }
}

#[derive(Debug)]
struct Packet {
    data: Vec<Datum>,
    is_divider: bool,
}

impl Packet {
    fn new(line: &str) -> Self {
        let tokens = Self::tokenize(line);
        Self {
            data: Self::collect(&tokens[1..tokens.len() - 1]),
            is_divider: false,
        }
    }
    fn new_divider(line: &str) -> Self {
        let tokens = Self::tokenize(line);
        Self {
            data: Self::collect(&tokens[1..tokens.len() - 1]),
            is_divider: true,
        }
    }

    fn collect(tokens: &[Token]) -> Vec<Datum> {
        let mut output = vec![];
        let mut add = true;
        let mut nested_count = 0;
        let mut start = 0;
        for (i, token) in tokens.iter().enumerate() {
            match token {
                Token::Open => {
                    if add {
                        add = false;
                        start = i;
                    } else {
                        nested_count += 1;
                    }
                }
                Token::Close => {
                    nested_count -= 1;
                    if nested_count < 0 {
                        nested_count = 0;
                        add = true;
                        let nt = Self::collect(&tokens[start + 1..i]);
                        output.push(Datum::List(nt));
                    }
                }
                Token::Num(num) => {
                    if add {
                        output.push(Datum::Number(*num))
                    }
                }
            }
        }
        output
    }

    fn tokenize(line: &str) -> Vec<Token> {
        let mut data = vec![];
        let mut curr = String::new();
        for c in line.chars() {
            if c.is_ascii_digit() {
                curr.push(c);
            } else {
                if !curr.is_empty() {
                    let num = curr.as_str().parse::<u8>().unwrap();
                    data.push(Token::Num(num));
                    curr.clear();
                }
                if c == ',' {
                    // do nothing
                } else if c == '[' {
                    data.push(Token::Open);
                } else if c == ']' {
                    data.push(Token::Close);
                } else {
                    panic!("Unknown character {}", c)
                }
            }
        }
        data
    }

    fn is_right_order(&self, other: &Packet) -> Option<bool> {
        Datum::check_list(&self.data, &other.data)
    }
}

#[derive(Debug)]
enum Datum {
    Number(u8),
    List(Vec<Datum>),
}

impl Datum {
    fn is_right_order(&self, other: &Datum) -> Option<bool> {
        match (self, other) {
            (Datum::Number(l), Datum::Number(r)) => {
                if l < r {
                    Some(true)
                } else if r < l {
                    Some(false)
                } else {
                    None
                }
            }
            (Datum::Number(l), Datum::List(_)) => {
                let l = Self::List(vec![Self::Number(*l)]);
                l.is_right_order(other)
            }
            (Datum::List(_), Datum::Number(r)) => {
                let r = Self::List(vec![Self::Number(*r)]);
                self.is_right_order(&r)
            }
            (Datum::List(l), Datum::List(r)) => Self::check_list(l, r),
        }
    }

    fn check_list(left: &[Datum], right: &[Datum]) -> Option<bool> {
        for i in 0..left.len().min(right.len()) {
            if let Some(res) = left[i].is_right_order(&right[i]) {
                return Some(res);
            }
        }
        if left.len() < right.len() {
            Some(true)
        } else if left.len() > right.len() {
            Some(false)
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Token {
    Open,
    Close,
    Num(u8),
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
        assert_eq!(13, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(140, part2());
    }
}
