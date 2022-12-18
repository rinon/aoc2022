use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashSet;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Point([i32; 3]);

impl Point {
    fn new((x, y, z): (i32, i32, i32)) -> Point {
        Point([x, y, z])
    }

    fn neighbors(&self) -> impl Iterator<Item = Self> + '_ {
        [
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ]
        .iter()
        .map(|(x, y, z)| Point([self.0[0] + x, self.0[1] + y, self.0[2] + z]))
    }
}

#[aoc_generator(day18)]
fn parse(input: &str) -> HashSet<Point> {
    input
        .lines()
        .flat_map(|l| l.split(",").flat_map(str::parse::<i32>).collect_tuple())
        .map(Point::new)
        .collect()
}

fn exposed(map: &HashSet<Point>) -> usize {
    map.iter()
        .map(|p| p.neighbors().filter(|n| !map.contains(&n)).count())
        .sum()
}

#[aoc(day18, part1)]
fn part1(map: &HashSet<Point>) -> usize {
    exposed(map)
}

#[aoc(day18, part2)]
fn part2(map: &HashSet<Point>) -> usize {
    let min = map.iter().fold([i32::MAX, i32::MAX, i32::MAX], |a, b| {
        [a[0].min(b.0[0]), a[1].min(b.0[1]), a[2].min(b.0[2])]
    });
    let max = map.iter().fold([i32::MIN, i32::MIN, i32::MIN], |a, b| {
        [a[0].max(b.0[0]), a[1].max(b.0[1]), a[2].max(b.0[2])]
    });
    let mut map = map.clone();
    let mut escaping = HashSet::new();
    for x in min[0]..max[0] {
        for y in min[1]..max[1] {
            'next: for z in min[2]..max[2] {
                let cur = Point([x, y, z]);
                if map.contains(&cur) {
                    continue;
                }
                let mut fill = HashSet::new();
                fill.insert(cur.clone());
                let mut s = vec![cur];
                while let Some(p) = s.pop() {
                    for n in p.neighbors() {
                        if escaping.contains(&n) {
                            escaping.extend(fill);
                            continue 'next;
                        }
                        for i in 0..3 {
                            if n.0[i] < min[i] || n.0[i] > max[i] {
                                escaping.extend(fill);
                                continue 'next;
                            }
                        }
                        if !fill.contains(&n) && !map.contains(&n) {
                            s.push(n.clone());
                            fill.insert(n);
                        }
                    }
                }
                map.extend(fill);
            }
        }
    }
    exposed(&map)
}

#[cfg(test)]
const TEST_INPUT: &'static str = include_str!("../input/2022/day18_example.txt");

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(&"1,1,1\n2,1,1")), 10);
    assert_eq!(part1(&parse(&TEST_INPUT)), 64);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(&TEST_INPUT)), 58);
}
