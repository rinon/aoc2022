use aoc_runner_derive::aoc;

#[aoc(day10, part1)]
fn part1(instructions: &str) -> i32 {
    let mut instructions = instructions.lines().map(str::split_whitespace);
    let mut x = 1;
    let mut next_x = 1;
    let mut signal = 0;
    let mut busy = 0;
    for cycle in 1..=220 {
        if busy == 0 {
            x = next_x;
        }
        if (cycle + 20) % 40 == 0 {
            signal += cycle * x;
        }
        if busy == 0 {
            let mut inst = instructions.next().unwrap();
            match inst.next().unwrap() {
                "addx" => {
                    next_x = x + inst.next().unwrap().parse::<i32>().unwrap();
                    busy = 1;
                }
                "noop" => {
                }
                inst => panic!("unexpected instruction {}", inst),
            }
        } else {
            busy -= 1;
        }
    }
    signal
}

#[aoc(day10, part2)]
fn part2(instructions: &str) -> String {
    let mut instructions = instructions.lines().map(str::split_whitespace);
    let mut crt = [[' '; 40]; 6];
    let mut x = 1i32;
    let mut next_x = 1;
    let mut busy = 0;
    for cycle in 1..=240 {
        if busy == 0 {
            x = next_x;
        }
        let row = (cycle - 1) / 40;
        let col = (cycle - 1) % 40;
        let pixel = &mut crt[row as usize][col as usize];
        if x - 1 <= col && col <= x + 1 {
            *pixel = '#';
        } else {
            *pixel = '.';
        }
        if busy == 0 {
            let mut inst = instructions.next().unwrap();
            match inst.next().unwrap() {
                "addx" => {
                    next_x = x + inst.next().unwrap().parse::<i32>().unwrap();
                    busy = 1;
                }
                "noop" => {
                }
                inst => panic!("unexpected instruction {}", inst),
            }
        } else {
            busy -= 1;
        }
    }
    let mut out = String::new();
    for row in &crt {
        out.push('\n');
        for c in row {
            out.push(*c);
        }
    }
    out
}

#[cfg(test)]
const TEST_INPUT: &'static str = include_str!("../input/2022/day10_example.txt");

#[test]
fn part1_test() {
    assert_eq!(part1(&TEST_INPUT), 13140);
}

#[test]
fn part2_test() {
    assert_eq!(&part2(&TEST_INPUT), "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
");
}
