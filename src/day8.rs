use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day8)]
fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().flat_map(|c| c.to_digit(10)).collect())
        .collect()
}

#[aoc(day8, part1)]
fn part1(map: &Vec<Vec<u32>>) -> usize {
    let mut count = 0;
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if map[y][0..x].iter().all(|x| x < c)
                || map[y][x + 1..].iter().all(|x| x < c)
                || map[0..y].iter().all(|col| col[x] < *c)
                || map[y + 1..].iter().all(|col| col[x] < *c)
            {
                count += 1;
            }
        }
    }
    count
}

#[aoc(day8, part2)]
fn part2(map: &Vec<Vec<u32>>) -> usize {
    let mut max = 0;
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if x == 0 || x == map[y].len() - 1 || y == 0 || y == map.len() - 1 {
                continue;
            }
            let mut score = 1;
            score *= map[y][1..x].iter().rev().take_while(|x| *x < c).count() + 1;
            score *= map[y][x + 1..map[y].len() - 1]
                .iter()
                .take_while(|x| *x < c)
                .count()
                + 1;
            score *= map[1..y].iter().rev().take_while(|col| col[x] < *c).count() + 1;
            score *= map[y + 1..map.len() - 1]
                .iter()
                .take_while(|col| col[x] < *c)
                .count()
                + 1;
            if score > max {
                max = score;
            }
        }
    }
    max
}

#[cfg(test)]
const TEST_INPUT: &'static str = include_str!("../input/2022/day8_example.txt");

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(&TEST_INPUT)), 21);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(&TEST_INPUT)), 8);
}
