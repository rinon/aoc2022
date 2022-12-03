use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| line.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

#[aoc(day1, part1)]
fn part1(input: &Vec<Vec<u32>>) -> u32 {
    input.iter().map(|x| x.iter().sum()).max().unwrap()
}

#[aoc(day1, part2)]
fn part2(input: &Vec<Vec<u32>>) -> u32 {
    let mut totals: Vec<u32> = input.iter().map(|x| x.iter().sum()).collect();
    totals.sort();
    totals[totals.len() - 3..].iter().sum()
}