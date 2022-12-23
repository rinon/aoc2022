use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

type Pos = [i32; 2];

#[derive(Clone)]
struct Map {
    dirs: [(usize, i32); 4],
    elves: HashSet<Pos>,
}

impl Map {
    fn print(&self) {
        let bounds = self.bounds();
        for y in bounds[0][1]..=bounds[1][1] {
            for x in bounds[0][0]..=bounds[1][0] {
                if self.elves.contains(&[x, y]) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }

    fn bounds(&self) -> [Pos; 2] {
        self.elves.iter().fold(
            [[i32::MAX, i32::MAX], [i32::MIN, i32::MIN]],
            |[[x0, y0], [x1, y1]], [x, y]| [[x0.min(*x), y0.min(*y)], [x1.max(*x), y1.max(*y)]],
        )
    }

    fn do_round(&mut self) -> bool {
        let mut proposed = HashMap::new();
        let last_elves = self.elves.iter().copied().collect_vec();
        for elf in last_elves {
            // dbg!(elf);
            let mut found = false;
            'found: for x in (elf[0] - 1)..=(elf[0] + 1) {
                for y in (elf[1] - 1)..=(elf[1] + 1) {
                    if [x, y] != elf && self.elves.contains(&[x, y]) {
                        found = true;
                        break 'found;
                    }
                }
            }
            if !found {
                continue;
            }
            'out: for (i, di) in self.dirs {
                let mut new_pos = elf;
                new_pos[i] += di;
                // dbg!(new_pos);
                let no_elf = (-1..=1).all(|j| {
                    let mut test = new_pos;
                    test[1 - i] += j;
                    // dbg!(test);
                    !self.elves.contains(&test)
                });
                // dbg!(no_elf);
                if no_elf {
                    proposed
                        .entry(new_pos)
                        .and_modify(|p| *p = None)
                        .or_insert(Some(elf));
                    break 'out;
                }
            }
        }

        self.dirs.rotate_left(1);

        if proposed.len() == 0 {
            return false;
        }

        for (pos, elf) in proposed.drain() {
            if let Some(elf) = elf {
                self.elves.remove(&elf);
                self.elves.insert(pos);
            }
        }

        true
    }
}

#[aoc_generator(day23)]
fn parse(input: &str) -> Map {
    let mut elves = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                elves.insert([x as i32, y as i32]);
            }
        }
    }
    Map {
        dirs: [(1, -1), (1, 1), (0, -1), (0, 1)],
        elves,
    }
}

#[aoc(day23, part1)]
fn part1(input: &Map) -> i32 {
    let mut map = input.clone();
    map.print();
    for _ in 0..10 {
        map.do_round();
        map.print();
    }
    let bounds = map.bounds();
    let mut empty = 0;
    for x in bounds[0][0]..=bounds[1][0] {
        for y in bounds[0][1]..=bounds[1][1] {
            if !map.elves.contains(&[x, y]) {
                empty += 1;
            }
        }
    }
    empty
}

#[aoc(day23, part2)]
fn part2(input: &Map) -> usize {
    let mut map = input.clone();
    for round in 1.. {
        if !map.do_round() {
            return round;
        }
    }
    0
}

#[cfg(test)]
const TEST_INPUT: &'static str = include_str!("../input/2022/day23_example.txt");

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(&TEST_INPUT)), 110);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(&TEST_INPUT)), 20);
}
