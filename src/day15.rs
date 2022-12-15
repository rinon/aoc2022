use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

fn dist(a: (i64, i64), b: (i64, i64)) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

#[derive(Debug, Clone)]
struct Sensor {
    coord: (i64, i64),
    closest_beacon: (i64, i64),
}

impl Sensor {
    fn beacon_dist(&self) -> i64 {
        dist(self.coord, self.closest_beacon)
    }

    fn not_beacon(&self, pos: (i64, i64)) -> bool {
        dist(self.coord, pos) <= self.beacon_dist() && pos != self.closest_beacon
    }
}

#[aoc_generator(day15)]
fn parse(input: &str) -> Vec<Sensor> {
    input
        .lines()
        .map(|line| {
            let line = line.strip_prefix("Sensor at x=").unwrap();
            let (x, y, bx, by) = line
                .chars()
                .group_by(|c| c.is_numeric() || *c == '-')
                .into_iter()
                .flat_map(|(numeric, cs)| {
                    if numeric {
                        cs.collect::<String>().parse::<i64>().ok()
                    } else {
                        None
                    }
                })
                .collect_tuple()
                .unwrap();
            Sensor {
                coord: (x, y),
                closest_beacon: (bx, by),
            }
        })
        .collect()
}

#[cfg(test)]
const TARGET_ROW: i64 = 10;

#[cfg(not(test))]
const TARGET_ROW: i64 = 2000000;

#[aoc(day15, part1)]
fn part1(sensors: &[Sensor]) -> usize {
    let xmin = sensors
        .iter()
        .map(|s| s.coord.0 - s.beacon_dist())
        .min()
        .unwrap();
    let xmax = sensors
        .iter()
        .map(|s| s.coord.0 + s.beacon_dist())
        .max()
        .unwrap();
    (xmin..=xmax)
        .filter(|x| sensors.iter().any(|s| s.not_beacon((*x, TARGET_ROW))))
        .count()
}

#[cfg(test)]
const COORD_MAX: i64 = 20;

#[cfg(not(test))]
const COORD_MAX: i64 = 4000000;

#[aoc(day15, part2)]
fn part2(sensors: &[Sensor]) -> Option<i64> {
    let check = |pos: (i64, i64)| {
        (pos.0 >= 0 && pos.0 <= COORD_MAX)
            && (pos.1 >= 0 && pos.1 <= COORD_MAX)
            && sensors
                .iter()
                .all(|s| !s.not_beacon(pos) && s.closest_beacon != pos)
    };
    for s in sensors {
        let mut pos = (s.coord.0 - s.beacon_dist() - 1, s.coord.1);
        while pos.0 < s.coord.0 {
            if check(pos) {
                return Some(pos.0 * 4000000 + pos.1);
            }
            pos.0 += 1;
            pos.1 -= 1;
        }
        assert_eq!(pos, (s.coord.0, s.coord.1 - s.beacon_dist() - 1));
        while pos.1 < s.coord.1 {
            if check(pos) {
                return Some(pos.0 * 4000000 + pos.1);
            }
            pos.0 += 1;
            pos.1 += 1;
        }
        assert_eq!(pos, (s.coord.0 + s.beacon_dist() + 1, s.coord.1));
        while pos.0 > s.coord.0 {
            if check(pos) {
                return Some(pos.0 * 4000000 + pos.1);
            }
            pos.0 -= 1;
            pos.1 += 1;
        }
        assert_eq!(pos, (s.coord.0, s.coord.1 + s.beacon_dist() + 1));
        while pos.1 > s.coord.1 {
            if check(pos) {
                return Some(pos.0 * 4000000 + pos.1);
            }
            pos.0 -= 1;
            pos.1 -= 1;
        }
        assert_eq!(pos, (s.coord.0 - s.beacon_dist() - 1, s.coord.1));
    }
    None
}

#[cfg(test)]
const TEST_INPUT: &'static str = include_str!("../input/2022/day15_example.txt");

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(&TEST_INPUT)), 26);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(&TEST_INPUT)), Some(56000011));
}
