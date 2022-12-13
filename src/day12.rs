use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn traversable(a: char, b: char) -> bool {
    let a = match a {
        'S' => 'a',
        'E' => 'z',
        x if x.is_ascii_lowercase() => x,
        x => panic!("unexpected character: {}", x),
    };
    let b = match b {
        'S' => 'a',
        'E' => 'z',
        x if x.is_ascii_lowercase() => x,
        x => panic!("unexpected character: {}", x),
    };
    a as u32 + 1 >= b as u32
}

#[aoc_generator(day12)]
fn parse(input: &str) -> (usize, usize, Vec<char>, Vec<Vec<usize>>) {
    let mut adj = vec![];
    let graph: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let height = graph.len();
    let width = graph[0].len();
    let mut start = None;
    let mut end = None;
    for y in 0..height {
        for (x, c) in graph[y].iter().enumerate() {
            if *c == 'S' {
                start = Some(y * width + x);
            } else if *c == 'E' {
                end = Some(y * width + x);
            }
            let mut neighbors = vec![];
            if x > 0 && traversable(*c, graph[y][x - 1]) {
                neighbors.push(y * width + x - 1);
            }
            if x < width - 1 && traversable(*c, graph[y][x + 1]) {
                neighbors.push(y * width + x + 1);
            }
            if y > 0 && traversable(*c, graph[y - 1][x]) {
                neighbors.push((y - 1) * width + x);
            }
            if y < height - 1 && traversable(*c, graph[y + 1][x]) {
                neighbors.push((y + 1) * width + x);
            }
            adj.push(neighbors);
        }
    }
    (
        start.unwrap(),
        end.unwrap(),
        graph.into_iter().flatten().collect(),
        adj,
    )
}

fn dijkstra(start: usize, end: usize, adj: &Vec<Vec<usize>>) -> Option<usize> {
    let mut dist = vec![usize::MAX; adj.len()];
    let mut prev = vec![None; adj.len()];
    dist[start] = 0;
    let mut q = BinaryHeap::new();
    q.push(Reverse((0, start)));

    while let Some(Reverse((d, v))) = q.pop() {
        if v == end {
            return Some(d);
        }
        for neighbor in &adj[v] {
            if dist[v] + 1 < dist[*neighbor] {
                dist[*neighbor] = dist[v] + 1;
                prev[*neighbor] = Some(v);
                q.push(Reverse((dist[*neighbor], *neighbor)));
            }
        }
    }
    None
}

#[aoc(day12, part1)]
fn part1((start, end, _graph, adj): &(usize, usize, Vec<char>, Vec<Vec<usize>>)) -> Option<usize> {
    dijkstra(*start, *end, adj)
}

#[aoc(day12, part2)]
fn part2((start, end, graph, adj): &(usize, usize, Vec<char>, Vec<Vec<usize>>)) -> usize {
    let mut min = usize::MAX;
    for (v, c) in graph.iter().enumerate() {
        if *c == 'S' || *c == 'a' {
            let dist = dijkstra(v, *end, adj).unwrap_or(usize::MAX);
            if dist < min {
                min = dist;
            }
        }
    }
    min
}

#[cfg(test)]
const TEST_INPUT: &'static str = include_str!("../input/2022/day12_example.txt");

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(&TEST_INPUT)), Some(31));
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(&TEST_INPUT)), 29);
}
