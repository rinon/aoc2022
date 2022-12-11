use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use num::integer::lcm;

struct Monkey {
    starting_items: Vec<u64>,
    operation: String,
    test_div: u64,
    dest_true: usize,
    dest_false: usize,
}

impl Monkey {
    fn inspect(&self, old: u64, modulo: Option<u64>) -> (usize, u64) {
        let (x, op, y) = self.operation.split_whitespace().collect_tuple().unwrap();
        let x = x.parse().unwrap_or(old);
        let y = y.parse().unwrap_or(old);
        let mut new = match op {
            "+" => x + y,
            "-" => x - y,
            "*" => x * y,
            "/" => x / y,
            _ => panic!("unexpected op {}", op),
        };
        if let Some(modulo) = modulo {
            new %= modulo;
        } else {
            new /= 3;
        }
        if new % self.test_div == 0 {
            (self.dest_true, new)
        } else {
            (self.dest_false, new)
        }
    }
}

#[aoc_generator(day11)]
fn parse(input: &str) -> Vec<Monkey> {
    let mut lines = input.lines();
    let mut monkeys = vec![];
    while let Some(line) = lines.next() {
        assert_eq!(line.split_whitespace().next(), Some("Monkey"));

        let starting_items = lines
            .next()
            .unwrap()
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(", ")
            .flat_map(str::parse::<u64>)
            .collect();

        let operation = lines
            .next()
            .unwrap()
            .strip_prefix("  Operation: new = ")
            .unwrap()
            .to_string();

        let test_div = lines
            .next()
            .unwrap()
            .strip_prefix("  Test: divisible by ")
            .unwrap()
            .parse()
            .unwrap();

        let dest_true = lines
            .next()
            .unwrap()
            .strip_prefix("    If true: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap();

        let dest_false = lines
            .next()
            .unwrap()
            .strip_prefix("    If false: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap();

        monkeys.push(Monkey {
            starting_items,
            operation,
            test_div,
            dest_true,
            dest_false,
        });

        lines.next();
    }
    monkeys
}

fn solve(monkeys: &[Monkey], decrease_worry: bool, rounds: usize) -> u64 {
    let mut items: Vec<_> = monkeys.iter().map(|m| m.starting_items.clone()).collect();
    let mut inspections = vec![0; monkeys.len()];
    let modulo = if decrease_worry {
        None
    } else {
        monkeys.iter().map(|m| m.test_div).reduce(lcm)
    };
    for _round in 0..rounds {
        for (m, monkey) in monkeys.iter().enumerate() {
            for item in &items[m].drain(..).collect_vec() {
                inspections[m] += 1;
                let (dest, item) = monkey.inspect(*item, modulo);
                items[dest].push(item);
            }
        }
    }
    inspections.sort();
    inspections.pop().unwrap() * inspections.pop().unwrap()
}

#[aoc(day11, part1)]
fn part1(monkeys: &[Monkey]) -> u64 {
    solve(monkeys, true, 20)
}

#[aoc(day11, part2)]
fn part2(monkeys: &[Monkey]) -> u64 {
    solve(monkeys, false, 10000)
}

#[cfg(test)]
const TEST_INPUT: &'static str = include_str!("../input/2022/day11_example.txt");

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(&TEST_INPUT)), 10605);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(&TEST_INPUT)), 2713310158);
}
