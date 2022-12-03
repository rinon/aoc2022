use aoc_runner_derive::aoc;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn parse(s: char) -> Shape {
        match s {
            'A' | 'X' => Shape::Rock,
            'B' | 'Y' => Shape::Paper,
            'C' | 'Z' => Shape::Scissors,
            other => panic!("Unexpected other character {}", other),
        }
    }

    fn value(self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn win(self) -> Shape {
        match self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }

    fn lose(self) -> Shape {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }

    fn score(self, other: Self) -> u32 {
        if self.win() == other {
            6
        } else if self == other {
            3
        } else {
            0
        }
    }
}

#[aoc(day2, part1)]
fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let mut chars = l.chars();
            (
                Shape::parse(chars.next().unwrap()),
                Shape::parse(chars.nth(1).unwrap()),
            )
        })
        .map(|(them, us)| us.value() + us.score(them))
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let mut chars = l.chars();
            (Shape::parse(chars.next().unwrap()), chars.nth(1).unwrap())
        })
        .map(|(them, outcome)| {
            let us = match outcome {
                'X' => them.win(),
                'Y' => them,
                'Z' => them.lose(),
                _ => panic!(),
            };
            us.value() + us.score(them)
        })
        .sum()
}

#[cfg(test)]
const TEST_INPUT: &'static str = "A Y
B X
C Z";


#[test]
fn part1_test() {
    assert_eq!(part1(TEST_INPUT), 15);
}

#[test]
fn part2_test() {
    assert_eq!(part2(TEST_INPUT), 12);
}
