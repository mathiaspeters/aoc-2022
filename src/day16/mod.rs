use std::collections::HashMap;

pub fn day16() {
    println!("Result 16-1: {}", part1());
    println!("Result 16-2: {}", part2());
}

pub fn part1() -> usize {
    let mut game_data = GameData::new();
    game_data.update_connections();
    let mut results = vec![];
    let mut states = game_data.relevant_valves.iter().map(|rv| {
        let minutes = game_data.connections[game_data.start_valve][*rv] as u8;
        State::new(minutes, *rv)
    }).collect::<Vec<_>>();
    let mut tmp_states = vec![];
    while !states.is_empty() {
        tmp_states.clear();
        for state in states.iter() {
            if state.minutes >= 30 {
                results.push(state.score as usize);
            }
            if state.open_valves.len() == game_data.relevant_valves.len() {
                results.push(state.score as usize);
            } else {
                if !state.open_valves.contains(&state.cv) {
                    let mut ns = state.clone();
                    ns.minutes += 1;
                    ns.score += 30_u16.saturating_sub(ns.minutes as u16) * game_data.valves[ns.cv];
                    ns.open_valves.push(ns.cv);
                    tmp_states.push(ns);
                } else {
                    for rv in game_data.relevant_valves.iter() {
                        if !state.open_valves.contains(rv) {
                            let mut ns = state.clone();
                            ns.minutes += game_data.connections[ns.cv][*rv] as u8;
                            ns.cv = *rv;
                            tmp_states.push(ns);
                        }
                    }
                }
            }
        }
        std::mem::swap(&mut states, &mut tmp_states);
    }
    results.into_iter().max().unwrap_or(0)
}

pub fn part2() -> usize {
    raw_input().len()
}

#[derive(Clone, Debug)]
struct State {
    minutes: u8,
    score: u16,
    cv: usize,
    open_valves: Vec<usize>,
}

impl State {
    fn new(minutes: u8, cv: usize) -> Self {
        Self {
            minutes,
            cv,
            score: 0,
            open_valves: vec![],
        }

    }
}

#[derive(Debug)]
struct GameData {
    valves: Vec<u16>,
    connections: Vec<Vec<usize>>,
    start_valve: usize,
    relevant_valves: Vec<usize>,
}

impl GameData {
    fn new() -> Self {
        let dictionary = raw_input()
            .lines()
            .enumerate()
            .map(|(i, line)| (line.split_ascii_whitespace().nth(1).unwrap(), i))
            .collect::<HashMap<&str, usize>>();
        let mut valves = Vec::with_capacity(dictionary.len());
        let mut connections = Vec::with_capacity(dictionary.len());
        let mut relevant_valves = vec![];
        raw_input().lines().enumerate().for_each(|(i, line)| {
            let line = line.split('=').nth(1).unwrap();
            let line = line.replace(
                |c: char| c == ';' || c == ',' || (c.is_alphabetic() && c.is_lowercase()),
                "",
            );
            let mut parts = line.split_ascii_whitespace().filter(|s| !s.is_empty());
            let flow_rate = parts.next().unwrap().parse::<u16>().unwrap();
            valves.push(flow_rate);
            if flow_rate > 0 {
                relevant_valves.push(i);
            }
            let connection = parts.map(|p| dictionary[p]).collect::<Vec<_>>();
            connections.push(connection);
        });
        let start_valve = dictionary["AA"];
        Self {
            valves,
            connections,
            start_valve,
            relevant_valves,
        }
    }

    fn update_connections(&mut self) {
        let new_connections = self
            .valves
            .iter()
            .enumerate()
            .map(|(i, _)| {
                if i == self.start_valve || self.relevant_valves.contains(&i) {
                    self.find_paths(i)
                } else {
                    vec![]
                }
            })
            .collect::<Vec<_>>();
        self.connections = new_connections;
    }

    fn find_paths(&self, origin: usize) -> Vec<usize> {
        let mut to_visit = vec![(origin, 0)];
        let mut visited = (0..self.valves.len()).map(|_| false).collect::<Vec<_>>();
        let mut result = (0..self.valves.len())
            .map(|_| usize::MAX)
            .collect::<Vec<_>>();
        while !to_visit.is_empty() {
            let (i, steps) = to_visit.pop().unwrap();
            visited[i] = true;
            result[i] = steps.min(result[i]);
            for j in &self.connections[i] {
                if !visited[*j] && !to_visit.iter().any(|(c, _)| *c == *j) {
                    to_visit.push((*j, steps + 1));
                }
            }
        }
        result
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
        assert_eq!(1651, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2());
    }
}
