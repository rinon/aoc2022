use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::BTreeMap;

#[aoc_generator(day5)]
fn parse(input: &str) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let mut stacks: BTreeMap<usize, Vec<char>> = BTreeMap::new();
    let mut lines = input.lines();
    for line in &mut lines {
        let mut found = false;
        for (i, c) in line.char_indices() {
            if (i + 3) % 4 == 0 && c.is_ascii_alphabetic() {
                stacks.entry(i / 4).or_default().push(c);
                found = true;
            }
        }
        if !found {
            break;
        }
    }
    lines.next().unwrap();
    let moves: Vec<_> = lines
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .step_by(2)
                .flat_map(str::parse::<usize>)
                .collect_tuple()
                .unwrap()
        })
        .collect();
    (stacks.into_values().collect(), moves)
}

#[aoc(day5, part1)]
fn part1((stacks, moves): &(Vec<Vec<char>>, Vec<(usize, usize, usize)>)) -> String {
    let mut stacks = stacks.clone();
    for m in moves {
        let moving: Vec<_> = stacks[m.1 - 1].drain(..m.0).collect();
        for x in moving {
            stacks[m.2 - 1].insert(0, x);
        }
    }
    stacks.into_iter().map(|s| s[0]).collect()
}

#[aoc(day5, part2)]
fn part2((stacks, moves): &(Vec<Vec<char>>, Vec<(usize, usize, usize)>)) -> String {
    let mut stacks = stacks.clone();
    for m in moves {
        let moving: Vec<_> = stacks[m.1 - 1].drain(..m.0).collect();
        stacks[m.2 - 1].splice(0..0, moving);
    }
    stacks.into_iter().map(|s| s[0]).collect()
}

#[cfg(test)]
const TEST_INPUT: &'static str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(TEST_INPUT)), "CMZ");
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(TEST_INPUT)), "MCD");
}
