use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::VecDeque;

#[aoc_generator(day20)]
fn parse(input: &str) -> Vec<i64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn decrypt(file: &[i64], count: usize) -> i64 {
    let mut decrypted: VecDeque<i64> = file.iter().copied().collect();
    let mut order: VecDeque<_> = (0..file.len()).collect();

    for _ in 0..count {
        for (i, c) in file.iter().copied().enumerate() {
            let (j, _) = order.iter().find_position(|x| **x == i).unwrap();
            assert_eq!(decrypted.remove(j).unwrap(), c);
            let new_order = order.remove(j).unwrap();
            let r = (c + j as i64).rem_euclid(decrypted.len() as i64) as usize;
            decrypted.insert(r, c);
            order.insert(r, new_order);
        }
    }
    decrypted
        .iter()
        .cycle()
        .skip_while(|x| **x != 0)
        .step_by(1000)
        .skip(1)
        .take(3)
        .sum()
}

#[aoc(day20, part1)]
fn part1(file: &[i64]) -> i64 {
    decrypt(file, 1)
}

#[aoc(day20, part2)]
fn part2(file: &[i64]) -> i64 {
    let file: Vec<i64> = file.iter().map(|x| x * 811589153).collect();
    decrypt(&file, 10)
}

#[cfg(test)]
const TEST_INPUT: &'static str = include_str!("../input/2022/day20_example.txt");

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(&TEST_INPUT)), 3);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(&TEST_INPUT)), 1623178306);
}
