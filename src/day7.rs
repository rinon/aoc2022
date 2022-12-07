use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[aoc_generator(day7)]
fn parse(input: &str) -> HashMap<PathBuf, usize> {
    let mut fs = HashMap::new();
    let mut cur_dir: PathBuf = "/".into();

    for line in input.lines() {
        if let Some(cmd) = line.strip_prefix("$ ") {
            if let Some(path) = cmd.strip_prefix("cd ") {
                match path {
                    "/" => {
                        cur_dir = "/".into();
                    }
                    ".." => {
                        cur_dir.pop();
                    }
                    _ => {
                        cur_dir.push(path);
                    }
                }
            } else {
                assert_eq!(cmd, "ls");
            }
        } else if line.starts_with("dir ") {
            continue;
        } else {
            for dir in cur_dir.ancestors() {
                *fs.entry(dir.to_owned()).or_insert(0) += line
                    .split_whitespace()
                    .next()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
            }
        }
    }
    fs
}

#[aoc(day7, part1)]
fn part1(fs: &HashMap<PathBuf, usize>) -> usize {
    fs.values().filter(|size| **size <= 100000).sum()
}

#[aoc(day7, part2)]
fn part2(fs: &HashMap<PathBuf, usize>) -> usize {
    let needed = 30000000 - (70000000 - fs[Path::new("/")]);
    *fs.values().filter(|size| **size > needed).min().unwrap()
}

#[cfg(test)]
const TEST_INPUT: &'static str = include_str!("../input/2022/day7_example.txt");

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(&TEST_INPUT)), 95437);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(&TEST_INPUT)), 24933642);
}
