use aoc_runner_derive::aoc;
use itertools::Itertools;

fn solve(input: &[char], len: usize) -> usize {
    for (i, c) in input.windows(len).enumerate() {
        let mut c = c.to_owned();
        c.sort();
        if c.iter().dedup().count() == len {
            return i + len;
        }
    }
    0
}

#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    let input = input.chars().collect_vec();
    solve(&input, 4)
}

#[aoc(day6, part2)]
fn part2(input: &str) -> usize {
    let input = input.chars().collect_vec();
    solve(&input, 14)
}

#[cfg(test)]
const TEST_INPUT: &'static str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

#[test]
fn part1_test() {
    assert_eq!(part1(&TEST_INPUT), 7);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&TEST_INPUT), 19);
}
