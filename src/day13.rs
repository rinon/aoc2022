use std::cmp::Ordering;
use std::num::ParseIntError;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Clone, Debug, Eq)]
enum Packet {
    List(Vec<Packet>),
    Int(u32),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::List(l), Packet::List(r)) => l.cmp(r),
            (Packet::Int(l), Packet::Int(r)) => l.cmp(r),
            (l, Packet::List(r)) => vec![l.clone()].cmp(r),
            (Packet::List(l), r) => l.cmp(&vec![r.clone()]),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Packet {
    fn parse(input: &str) -> Result<(Packet, &str), ParseIntError> {
        if input.starts_with("[") {
            let mut packets = vec![];
            let mut input = &input[1..];
            while let Ok((p, res)) = Packet::parse(input) {
                input = res.strip_prefix(",").unwrap_or(res);
                packets.push(p);
            }
            input = input.strip_prefix("]").unwrap();
            Ok((Packet::List(packets), input))
        } else {
            let i = input.find(&[',', ']']).unwrap_or(input.len());
            let (input, res) = input.split_at(i);
            input.parse::<u32>().map(|i| (Packet::Int(i), res))
        }
    }
}

#[aoc_generator(day13)]
fn parse(input: &str) -> Vec<Packet> {
    input
        .lines()
        .filter(|l| l.len() > 0)
        .map(|l| Packet::parse(l).unwrap().0)
        .collect()
}

#[aoc(day13, part1)]
fn part1(input: &[Packet]) -> usize {
    input
        .iter()
        .tuples()
        .enumerate()
        .filter_map(|(i, (l, r))| if l < r { Some(i + 1) } else { None })
        .sum()
}

#[aoc(day13, part2)]
fn part2(input: &[Packet]) -> usize {
    let mut packets = input.to_vec();
    let dividers = [
        Packet::List(vec![Packet::List(vec![Packet::Int(2)])]),
        Packet::List(vec![Packet::List(vec![Packet::Int(6)])]),
    ];
    packets.extend(dividers.iter().cloned());
    packets.sort();
    (packets.iter().position(|p| *p == dividers[0]).unwrap() + 1)
        * (packets.iter().position(|p| *p == dividers[1]).unwrap() + 1)
}

#[cfg(test)]
const TEST_INPUT: &'static str = include_str!("../input/2022/day13_example.txt");

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(&TEST_INPUT)), 13);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(&TEST_INPUT)), 140);
}
