use std::collections::HashSet;

pub fn day15() {
    println!("Result 15-1: {}", part1());
    println!("Result 15-2: {}", part2());
}

pub fn part1() -> usize {
    let (raw_input, y_check) = raw_input();
    let sensors = raw_input
        .lines()
        .map(|line| {
            let now = line.replace(|c: char| !c.is_ascii_digit() && c != '=' && c != '-', "");
            let mut now = now.split('=').skip(1).map(|s| s.parse::<isize>().unwrap());
            Sensor {
                pos: (now.next().unwrap(), now.next().unwrap()),
                nearest_beacon: (now.next().unwrap(), now.next().unwrap()),
            }
        })
        .collect::<Vec<_>>();
    let mut relevant_beacons: HashSet<isize> = HashSet::new();
    let mut ranges = vec![];
    sensors.into_iter().for_each(|sensor| {
        if sensor.nearest_beacon.1 == y_check {
            relevant_beacons.insert(sensor.nearest_beacon.0);
        }
        let max_distance = (sensor.pos.0 - sensor.nearest_beacon.0).abs()
            + (sensor.pos.1 - sensor.nearest_beacon.1).abs();
        let distance_from_row = (sensor.pos.1 - y_check).abs();
        if (sensor.pos.1 - y_check).abs() <= max_distance {
            for i in 1..=(max_distance - distance_from_row) {
                ranges.push((sensor.pos.0 - i, sensor.pos.0 + i));
            }
        }
    });
    let start = ranges
        .iter()
        .min_by_key(|(s, _)| *s)
        .map(|(s, _)| *s)
        .unwrap();
    let end = ranges
        .iter()
        .max_by_key(|(_, e)| *e)
        .map(|(_, e)| *e)
        .unwrap();
    (end - start + 1) as usize - relevant_beacons.len()
}

pub fn part2() -> usize {
    let (raw_input, y_check) = raw_input();
    let sensors = raw_input
        .lines()
        .map(|line| {
            let now = line.replace(|c: char| !c.is_ascii_digit() && c != '=' && c != '-', "");
            let mut now = now.split('=').skip(1).map(|s| s.parse::<isize>().unwrap());
            Sensor {
                pos: (now.next().unwrap(), now.next().unwrap()),
                nearest_beacon: (now.next().unwrap(), now.next().unwrap()),
            }
        })
        .collect::<Vec<_>>();
    let max_row_col = y_check * 2;
    for yc in 0..=max_row_col {
        //if yc % 10 == 0 {
        println!("Checking row {yc}");
        //}
        let mut ranges = vec![];
        sensors.iter().for_each(|sensor| {
            let max_distance = (sensor.pos.0 - sensor.nearest_beacon.0).abs()
                + (sensor.pos.1 - sensor.nearest_beacon.1).abs();
            let distance_from_row = (sensor.pos.1 - yc).abs();
            if (sensor.pos.1 - yc).abs() <= max_distance {
                for i in 0..=(max_distance - distance_from_row) {
                    let start = sensor.pos.0 - i;
                    let end = sensor.pos.0 + i;
                    if start <= max_row_col && end >= 0 {
                        ranges.push(((start).max(0), (end).min(max_row_col)));
                    }
                }
            }
        });
        /*let mut r = *ranges.iter().find(|(s, _)| *s == 0).unwrap();
        loop {
            let nr = ranges.iter().find(|(s, _)| *s <= r.1);
            match nr {
                Some(nr) if nr.1 == max_row_col => continue 'outer,
                Some(nr) => r = *nr,
                None => {
                    let x = r.1 + 1;
                    println!("x: {x}, y: {yc}");
                    return (x * 4_000_000 + yc) as usize;
                }
            }
        }*/
        //ranges.sort_unstable_by_key(|(_, e)| *e);
        for i in 0..=max_row_col {
            if ranges.iter().find(|(s, e)| i >= *s && i <= *e).is_none() {
                println!("x: {i}, y: {yc}");
                return (i * 4_000_000 + yc) as usize;
            }
        }
    }
    0
}

#[derive(Debug, Copy, Clone)]
struct Sensor {
    pos: (isize, isize),
    nearest_beacon: (isize, isize),
}

#[cfg(not(test))]
fn raw_input() -> (&'static str, isize) {
    (include_str!("input"), 2_000_000)
}

#[cfg(test)]
fn raw_input() -> (&'static str, isize) {
    (include_str!("testinput"), 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(26, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(56000011, part2());
    }
}
