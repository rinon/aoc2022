use aoc_runner_derive::aoc;
use itertools::Itertools;

fn priority(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        c as u32 - 'a' as u32 + 1
    } else {
        c as u32 - 'A' as u32 + 27
    }
}

#[aoc(day3, part1)]
fn part1(input: &str) -> u32 {
    let mut sum = 0u32;
    for line in input.lines() {
        let (c1, c2) = line.split_at(line.len() / 2);
        for c in c1.chars() {
            if c2.contains(c) {
                sum += priority(c);
                break;
            }
        }
    }
    sum
}

#[aoc(day3, part2)]
fn part2(input: &str) -> u32 {
    let mut sum = 0u32;
    for (p1, p2, p3) in input.lines().tuples() {
        'out: for c in p1.chars() {
            if p2.contains(c) && p3.contains(c) {
                sum += priority(c);
                break 'out;
            }
        }
    }
    sum
}

#[cfg(test)]
const TEST_INPUT: &'static str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

#[test]
fn part1_test() {
    assert_eq!(part1(TEST_INPUT), 157);
}

#[test]
fn part2_test() {
    assert_eq!(part2(TEST_INPUT), 70);
}
