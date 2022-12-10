use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

enum Dir {
    Left,
    Right,
    Up,
    Down,
}

impl Dir {
    fn mv(&self, (x, y): (i32, i32)) -> (i32, i32) {
        match self {
            Dir::Left => (x - 1, y),
            Dir::Right => (x + 1, y),
            Dir::Up => (x, y + 1),
            Dir::Down => (x, y - 1),
        }
    }
}

impl From<&str> for Dir {
    fn from(c: &str) -> Dir {
        match c {
            "L" => Dir::Left,
            "R" => Dir::Right,
            "U" => Dir::Up,
            "D" => Dir::Down,
            _ => panic!("unexpected character {}", c),
        }
    }
}

#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<(Dir, i32)> {
    input
        .lines()
        .map(|line| {
            let mut words = line.split_whitespace();
            (
                words.next().unwrap().into(),
                words.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

#[aoc(day9, part1)]
fn part1(instructions: &[(Dir, i32)]) -> usize {
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    for (dir, count) in instructions {
        for _ in 0..*count {
            let new_head = dir.mv(head);
            if (tail.0 - new_head.0).abs() > 1 || (tail.1 - new_head.1).abs() > 1 {
                tail = head;
            }
            head = new_head;
            visited.insert(tail);
        }
    }
    visited.len()
}

#[aoc(day9, part2)]
fn part2(instructions: &[(Dir, i32)]) -> usize {
    let mut rope = [(0, 0); 10];
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    for (dir, count) in instructions {
        for _ in 0..*count {
            rope[0] = dir.mv(rope[0]);
            let mut next = rope[0];
            for cur in rope[1..].iter_mut() {
                let x_delta = if cur.0 - next.0 > 0 {
                    -1
                } else if next.0 - cur.0 > 0 {
                    1
                } else {
                    0
                };
                let y_delta = if cur.1 - next.1 > 0 {
                    -1
                } else if next.1 - cur.1 > 0 {
                    1
                } else {
                    0
                };
                if (cur.0 - next.0).abs() + (cur.1 - next.1).abs() >= 3 {
                    *cur = (cur.0 + x_delta, cur.1 + y_delta);
                } else if (cur.0 - next.0).abs() > 1 {
                    *cur = (cur.0 + x_delta, cur.1);
                } else if (cur.1 - next.1).abs() > 1 {
                    *cur = (cur.0, cur.1 + y_delta);
                }
                next = *cur;
            }
            visited.insert(rope[rope.len() - 1]);
        }
    }
    visited.len()
}

#[test]
fn part1_test() {
    assert_eq!(
        part1(&parse(
            &"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"
        )),
        13
    );
}

#[test]
fn part2_test() {
    assert_eq!(
        part2(&parse(
            &"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"
        )),
        36
    );
}
