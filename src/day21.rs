use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Node {
    Num(i64),
    Op {
        left: String,
        op: String,
        right: String,
    },
}

impl Node {
    fn left(&self) -> &str {
        if let Node::Op {
            left,
            ..
        } = self
        {
            return left;
        } else {
            panic!("Node {:?} does not have a left child", self);
        }
    }

    fn right(&self) -> &str {
        if let Node::Op {
            right,
            ..
        } = self
        {
            return right;
        } else {
            panic!("Node {:?} does not have a right child", self);
        }
    }
}

#[aoc_generator(day21)]
fn parse(input: &str) -> HashMap<String, Node> {
    input
        .lines()
        .map(|line| {
            let (name, line) = line.split_once(": ").unwrap();
            let name = name.to_string();
            if let Ok(num) = line.parse() {
                (name, Node::Num(num))
            } else {
                let (left, op, right) = line
                    .split_whitespace()
                    .map(str::to_string)
                    .collect_tuple()
                    .unwrap();
                (
                    name,
                    Node::Op {
                        left,
                        op,
                        right,
                    },
                )
            }
        })
        .collect()
}

#[aoc(day21, part1)]
fn part1(tree: &HashMap<String, Node>) -> i64 {
    let mut tree = tree.clone();
    let mut nums: HashMap<String, i64> = tree
        .iter()
        .filter_map(|(n, v)| {
            if let Node::Num(num) = v {
                Some((n.clone(), *num))
            } else {
                None
            }
        })
        .collect();
    tree.retain(|_n, v| if let Node::Op { .. } = v { true } else { false });
    while !nums.contains_key("root") {
        let mut found = None;
        for (name, node) in &tree {
            if let Node::Op {
                left,
                op,
                right,
            } = node
            {
                if let (Some(x), Some(y)) = (nums.get(&*left), nums.get(&*right)) {
                    let new = match &**op {
                        "+" => x + y,
                        "-" => x - y,
                        "*" => x * y,
                        "/" => x / y,
                        _ => panic!("unexpected op {}", op),
                    };
                    nums.insert(name.clone(), new);
                    found = Some(name.clone());
                    break;
                }
            }
        }
        if let Some(name) = found {
            tree.remove(&*name);
        } else {
            panic!("Could not resolve a monkey");
        }
    }
    nums["root"]
}

#[aoc(day21, part2)]
fn part2(tree: &HashMap<String, Node>) -> i64 {
    let mut tree = tree.clone();
    let mut nums: HashMap<String, i64> = tree
        .iter()
        .filter_map(|(n, v)| {
            if let Node::Num(num) = v {
                Some((n.clone(), *num))
            } else {
                None
            }
        })
        .collect();
    nums.remove("humn");
    tree.retain(|_n, v| if let Node::Op { .. } = v { true } else { false });
    let mut found = Some(String::new());
    while found.is_some() {
        found = None;
        for (name, node) in &tree {
            if let Node::Op {
                left,
                op,
                right,
            } = node
            {
                if let (Some(x), Some(y)) = (nums.get(&*left), nums.get(&*right)) {
                    let new = match &**op {
                        "+" => x + y,
                        "-" => x - y,
                        "*" => x * y,
                        "/" => x / y,
                        _ => panic!("unexpected op {}", op),
                    };
                    nums.insert(name.clone(), new);
                    found = Some(name.clone());
                    break;
                }
            }
        }
        if let Some(name) = &found {
            tree.remove(&**name);
        }
    }
    let (mut cur_node, mut cur_value) = if let Some(x) = nums.get(tree["root"].left()) {
        (tree["root"].right(), *x)
    } else if let Some(x) = nums.get(tree["root"].right()) {
        (tree["root"].left(), *x)
    } else {
        panic!();
    };
    while cur_node != "humn" {
        if let Node::Op { left, op, right } = &tree[cur_node] {
            if let Some(x) = nums.get(left) {
                cur_node = tree[cur_node].right();
                cur_value = match &**op {
                    "+" => cur_value - *x,
                    "-" => *x - cur_value,
                    "*" => cur_value / *x,
                    "/" => *x / cur_value,
                    _ => panic!("unexpected op {}", op),
                };
            } else if let Some(x) = nums.get(right) {
                cur_node = tree[cur_node].left();
                cur_value = match &**op {
                    "+" => cur_value - *x,
                    "-" => cur_value + *x,
                    "*" => cur_value / *x,
                    "/" => cur_value * *x,
                    _ => panic!("unexpected op {}", op),
                };
            } else {
                panic!();
            };
        } else {
            panic!();
        }
    }
    cur_value
}

#[cfg(test)]
const TEST_INPUT: &'static str = include_str!("../input/2022/day21_example.txt");

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(&TEST_INPUT)), 152);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(&TEST_INPUT)), 301);
}
