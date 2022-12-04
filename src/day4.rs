use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::ops::RangeInclusive;

type Assignment = RangeInclusive<u32>;

#[aoc_generator(day4)]
fn parse(input: &str) -> Vec<(Assignment, Assignment)> {
    input
        .lines()
        .map(|line| {
            let (a, b) = line
                .split(',')
                .flat_map(|s| {
                    s.split('-')
                        .flat_map(str::parse::<u32>)
                        .next_tuple::<(_, _)>()
                })
                .next_tuple()
                .unwrap();
            (RangeInclusive::new(a.0, a.1), RangeInclusive::new(b.0, b.1))
        })
        .collect()
}

#[aoc(day4, part1)]
fn part1(input: &[(Assignment, Assignment)]) -> usize {
    input.iter().filter(|(a, b)| {
        (a.contains(b.start()) && a.contains(b.end())) || 
        (b.contains(a.start()) && b.contains(a.end()))
    }).count()
}

#[aoc(day4, part2)]
fn part2(input: &[(Assignment, Assignment)]) -> usize {
    input.iter().filter(|(a, b)| {
        a.contains(b.start()) || a.contains(b.end()) || 
        b.contains(a.start()) || b.contains(a.end())
    }).count()
}

#[cfg(test)]
const TEST_INPUT: &'static str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(TEST_INPUT)), 2);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(TEST_INPUT)), 4);
}
