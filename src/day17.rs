use aoc_runner_derive::aoc;

const ROCKS: &'static [[[bool; 4]; 4]; 5] = &[
    [[true, true, true, true], [false; 4], [false; 4], [false; 4]],
    [
        [false, true, false, false],
        [true, true, true, false],
        [false, true, false, false],
        [false; 4],
    ],
    [
        [true, true, true, false],
        [false, false, true, false],
        [false, false, true, false],
        [false; 4],
    ],
    [
        [true, false, false, false],
        [true, false, false, false],
        [true, false, false, false],
        [true, false, false, false],
    ],
    [
        [true, true, false, false],
        [true, true, false, false],
        [false; 4],
        [false; 4],
    ],
];

fn simulate(jets: &str, iterations: usize) -> usize {
    let mut jets = jets.trim().chars().enumerate().cycle();
    let mut tower: Vec<[bool; 7]> = vec![];
    let mut floor = 0;
    let mut rockjets = vec![];
    let mut skip = 0;
    let mut i = 0;
    for (rocki, rock) in ROCKS.iter().enumerate().cycle() {
        let mut first_jeti = None;
        let (mut x, mut y) = (2, floor + 4);
        'next: loop {
            if y == 0 {
                break;
            }
            for yi in 0..4 {
                for xi in 0..4 {
                    if !rock[yi][xi] {
                        continue;
                    }
                    if tower.len() > y - 1 + yi && tower[y - 1 + yi][xi + x] {
                        break 'next;
                    }
                }
            }
            y -= 1;
            let (jeti, jet) = jets.next().unwrap();
            if first_jeti == None {
                first_jeti = Some(jeti);
            }
            let tx = match jet {
                '>' => x + 1,
                '<' if x == 0 => {
                    continue 'next;
                }
                '<' => x - 1,
                _ => panic!("unexpected character {}", jet),
            };
            for yi in 0..4 {
                for xi in 0..4 {
                    if !rock[yi][xi] {
                        continue;
                    }
                    if tx + xi >= 7 || (tower.len() > y + yi && tower[y + yi][tx + xi]) {
                        continue 'next;
                    }
                }
            }
            x = tx;
        }

        let cur = (rocki, first_jeti.unwrap(), floor);
        let mut matches = rockjets
            .iter()
            .enumerate()
            .filter(|(_, (ri, ji, _))| rocki == *ri && first_jeti.unwrap() == *ji);
        if let Some((rocka, (_, _, rowa))) = matches.next() {
            if let Some((rockb, (_, _, rowb))) = matches.next() {
                if tower[*rowa..*rowb] == tower[*rowb..] {
                    let rock_count = rockb - rocka;
                    let row_count = rowb - rowa;
                    // eprintln!("rock period: {}, row count: {}", rock_count, row_count);
                    // eprintln!("rock offset: {}, row offset: {}", rocka, rowa);
                    // eprintln!("skipping {} rocks", ((iterations - i) / rock_count) * rock_count);
                    // eprintln!("starting i {}, floor {}", i, floor);
                    skip = ((iterations - i) / rock_count) * row_count;
                    i += ((iterations - i) / rock_count) * rock_count;
                    // eprintln!("new i {}", i);
                    if i == iterations {
                        return floor + skip;
                    }
                    rockjets.clear();
                }
            }
        }
        rockjets.push(cur);

        for yi in 0..4 {
            for xi in 0..4 {
                if !rock[yi][xi] {
                    continue;
                }
                while tower.len() <= y + yi {
                    tower.push([false; 7]);
                }
                assert!(!tower[y + yi][x + xi]);
                tower[y + yi][x + xi] = tower[y + yi][x + xi] || rock[yi][xi];
                floor = floor.max(y + yi + 1);
            }
        }

        i += 1;
        if i == iterations {
            return floor + skip;
        }
    }
    unreachable!();
}

#[aoc(day17, part1)]
fn part1(jets: &str) -> usize {
    simulate(jets, 2022)
}

#[aoc(day17, part2)]
fn part2(jets: &str) -> usize {
    simulate(jets, 1000000000000)
}

#[cfg(test)]
const TEST_INPUT: &'static str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

#[test]
fn part1_test() {
    assert_eq!(part1(&TEST_INPUT), 3068);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&TEST_INPUT), 1514285714288);
}
