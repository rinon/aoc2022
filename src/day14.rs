use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashSet;

#[aoc_generator(day14)]
fn parse(input: &str) -> HashSet<(u32, u32)> {
    let mut map = HashSet::new();
    for line in input.lines() {
        for (a, b) in line
            .split(" -> ")
            .map(|coord| {
                coord
                    .split(",")
                    .flat_map(str::parse::<u32>)
                    .collect_tuple::<(_, _)>()
                    .unwrap()
            })
            .tuple_windows()
        {
            if a.0 == b.0 {
                let range = if a.1 < b.1 { a.1..=b.1 } else { b.1..=a.1 };
                for i in range {
                    map.insert((a.0, i));
                }
            } else if a.1 == b.1 {
                let range = if a.0 < b.0 { a.0..=b.0 } else { b.0..=a.0 };
                for i in range {
                    map.insert((i, a.1));
                }
            } else {
                panic!("rock {} not a straight line", line);
            }
        }
    }
    map
}

#[aoc(day14, part1)]
fn part1(map: &HashSet<(u32, u32)>) -> usize {
    let mut map = map.clone();
    let floor = *map.iter().map(|(_x, y)| y).max().unwrap();
    let mut count = 0;
    let mut pos = (500, 0);
    while pos.1 < floor {
        if !map.contains(&(pos.0, pos.1 + 1)) {
            pos = (pos.0, pos.1 + 1);
            continue;
        }
        if !map.contains(&(pos.0 - 1, pos.1 + 1)) {
            pos = (pos.0 - 1, pos.1 + 1);
            continue;
        }
        if !map.contains(&(pos.0 + 1, pos.1 + 1)) {
            pos = (pos.0 + 1, pos.1 + 1);
            continue;
        }
        count += 1;
        map.insert(pos);
        pos = (500, 0);
    }
    count
}

#[aoc(day14, part2)]
fn part2(map: &HashSet<(u32, u32)>) -> usize {
    let mut map = map.clone();
    let floor = *map.iter().map(|(_x, y)| y).max().unwrap() + 1;
    let mut count = 0;
    let mut pos = (500, 0);
    while !map.contains(&(500, 0)) {
        while pos.1 < floor {
            if !map.contains(&(pos.0, pos.1 + 1)) {
                pos = (pos.0, pos.1 + 1);
            } else if !map.contains(&(pos.0 - 1, pos.1 + 1)) {
                pos = (pos.0 - 1, pos.1 + 1);
            } else if !map.contains(&(pos.0 + 1, pos.1 + 1)) {
                pos = (pos.0 + 1, pos.1 + 1);
            } else {
                break;
            }
        }
        count += 1;
        map.insert(pos);
        pos = (500, 0);
    }
    count
}

#[cfg(test)]
const TEST_INPUT: &'static str = include_str!("../input/2022/day14_example.txt");

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(&TEST_INPUT)), 24);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(&TEST_INPUT)), 93);
}
