use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Tile {
    Open,
    Wall,
    None,
}

impl TryFrom<char> for Tile {
    type Error = String;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Tile::Open),
            '#' => Ok(Tile::Wall),
            ' ' => Ok(Tile::None),
            _ => Err(format!("Could not parse {} as a Tile", value)),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Facing {
    Right = 0,
    Down,
    Left,
    Up,
}

impl Facing {
    fn rotate(self, dir: Rotation) -> Self {
        match (self, dir) {
            (_, Rotation::None) => self,
            (Facing::Right, Rotation::Clockwise) | (Facing::Left, Rotation::CounterClockwise) => {
                Facing::Down
            }
            (Facing::Down, Rotation::Clockwise) | (Facing::Up, Rotation::CounterClockwise) => {
                Facing::Left
            }
            (Facing::Left, Rotation::Clockwise) | (Facing::Right, Rotation::CounterClockwise) => {
                Facing::Up
            }
            (Facing::Up, Rotation::Clockwise) | (Facing::Down, Rotation::CounterClockwise) => {
                Facing::Right
            }
        }
    }
}

#[derive(Clone, Debug)]
struct MapPosition {
    x: usize,
    y: usize,
    dir: Facing,
}

impl MapPosition {
    fn password(&self) -> usize {
        1000 * (self.y + 1) + 4 * (self.x + 1) + self.dir as usize
    }
}

#[derive(Clone, Debug)]
struct Map {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
    cube: Option<usize>,
}

impl Map {
    fn new(width: usize) -> Map {
        Map {
            tiles: vec![],
            width,
            height: 0,
            cube: None,
        }
    }

    fn into_cube(self) -> Self {
        let mut side_size = usize::MAX;
        for y in 0..self.height {
            let mut width = 0;
            for x in 0..self.width {
                if self.get((x, y)) != Tile::None {
                    width += 1;
                }
            }
            side_size = side_size.min(width);
        }
        dbg!(side_size);
        Self {
            cube: Some(side_size),
            ..self
        }
    }

    fn insert(&mut self, pos: (usize, usize), tile: Tile) {
        if pos.1 >= self.height {
            self.height = pos.1 + 1;
        }
        let idx = self.idx(pos);
        if self.tiles.len() <= idx {
            self.tiles.resize(idx + 1, Tile::None);
        }
        self.tiles[idx] = tile;
    }

    fn get(&self, pos: (usize, usize)) -> Tile {
        self.tiles.get(self.idx(pos)).copied().unwrap_or(Tile::None)
    }

    fn idx(&self, (x, y): (usize, usize)) -> usize {
        y as usize * self.width + x as usize
    }

    fn do_move(&self, mut pos: MapPosition, mv: &Move) -> MapPosition {
        for _ in 0..mv.num {
            let mut new_pos = pos.clone();
            let (x, y) = (pos.x as isize, pos.y as isize);
            let (x, y) = match pos.dir {
                Facing::Right => (x + 1, y),
                Facing::Down => (x, y + 1),
                Facing::Left => (x - 1, y),
                Facing::Up => (x, y - 1),
            };
            if x < 0 || y < 0 {
                self.wrap(&mut new_pos);
            } else {
                new_pos.x = x.try_into().unwrap();
                new_pos.y = y.try_into().unwrap();
                if self.get((new_pos.x, new_pos.y)) == Tile::None {
                    self.wrap(&mut new_pos);
                }
            }
            match self.get((new_pos.x, new_pos.y)) {
                Tile::Open => {
                    pos = new_pos;
                }
                Tile::Wall => break,
                Tile::None => panic!("Could not move to {:?}", (x, y)),
            }
        }
        pos.dir = pos.dir.rotate(mv.dir);
        pos
    }

    fn wrap(&self, pos: &mut MapPosition) {
        if let Some(4) = self.cube {
            self.cube_wrap_test(pos);
        } else if let Some(50) = self.cube {
            self.cube_wrap(pos);
        } else {
            self.rect_wrap(pos);
        }
    }

    fn rect_wrap(&self, pos: &mut MapPosition) {
        match pos.dir {
            Facing::Up => {
                pos.y = (0..self.height)
                    .rev()
                    .find(|i| self.get((pos.x, *i)) != Tile::None)
                    .unwrap();
            }
            Facing::Down => {
                pos.y = (0..self.height)
                    .find(|i| self.get((pos.x, *i)) != Tile::None)
                    .unwrap();
            }
            Facing::Left => {
                pos.x = (0..self.width)
                    .rev()
                    .find(|i| self.get((*i, pos.y)) != Tile::None)
                    .unwrap();
            }
            Facing::Right => {
                pos.x = (0..self.width)
                    .find(|i| self.get((*i, pos.y)) != Tile::None)
                    .unwrap();
            }
        }
    }

    fn cube_wrap(&self, pos: &mut MapPosition) {
        let size = self.cube.unwrap();
        match pos.dir {
            Facing::Up if pos.x < size => {
                // 4 -> 3
                pos.y = pos.x + size;
                pos.x = size;
                pos.dir = Facing::Right;
            }
            Facing::Up if pos.x < 2 * size => {
                // 1 -> 6
                pos.y = pos.x - size + 3 * size;
                pos.x = 0;
                pos.dir = Facing::Right;
            }
            Facing::Up => {
                // 2 -> 6
                pos.x = pos.x - 2 * size;
                pos.y = 4 * size - 1;
                pos.dir = Facing::Up;
            }
            Facing::Down if pos.x < size => {
                // 6 -> 2
                pos.x = pos.x + 2 * size;
                pos.y = 0;
                pos.dir = Facing::Down;
            }
            Facing::Down if pos.x < 2 * size => {
                // 5 -> 6
                pos.y = pos.x - size + 3 * size;
                pos.x = size - 1;
                pos.dir = Facing::Left;
            }
            Facing::Down => {
                // 2 -> 3
                pos.y = pos.x - 2 * size + size;
                pos.x = 2 * size - 1;
                pos.dir = Facing::Left;
            }
            Facing::Left if pos.y < size => {
                // 1 -> 4
                pos.y = size - 1 - pos.y + 2 * size;
                pos.x = 0;
                pos.dir = Facing::Right;
            }
            Facing::Left if pos.y < 2 * size => {
                // 3 -> 4
                pos.x = pos.y - size;
                pos.y = 2 * size;
                pos.dir = Facing::Down;
            }
            Facing::Left if pos.y < 3 * size => {
                // 4 -> 1
                pos.y = 3 * size - 1 - pos.y;
                pos.x = size;
                pos.dir = Facing::Right;
            }
            Facing::Left => {
                // 6 -> 1
                pos.x = pos.y - 3 * size + size;
                pos.y = 0;
                pos.dir = Facing::Down;
            }
            Facing::Right if pos.y < size => {
                // 2 -> 5
                pos.y = size - 1 - pos.y + 2 * size;
                pos.x = 2 * size - 1;
                pos.dir = Facing::Left;
            }
            Facing::Right if pos.y < 2 * size => {
                // 3 -> 2
                pos.x = pos.y - size + 2 * size;
                pos.y = size - 1;
                pos.dir = Facing::Up;
            }
            Facing::Right if pos.y < 3 * size => {
                // 5 -> 2
                pos.y = 3 * size - 1 - pos.y;
                pos.x = 3 * size - 1;
                pos.dir = Facing::Left;
            }
            Facing::Right => {
                // 6 -> 5
                pos.x = pos.y - 3 * size + size;
                pos.y = 3 * size - 1;
                pos.dir = Facing::Up;
            }
        }
    }
    fn cube_wrap_test(&self, pos: &mut MapPosition) {
        let size = self.cube.unwrap();
        match pos.dir {
            Facing::Up if pos.x < size => {
                // 2 -> 1
                pos.x = size - 1 - pos.x + 2 * size;
                pos.y = 0;
                pos.dir = Facing::Down;
            }
            Facing::Up if pos.x < 2 * size => {
                // 3 -> 1
                pos.y = pos.x - size;
                pos.x = 2 * size;
                pos.dir = Facing::Right;
            }
            Facing::Up if pos.x < 3 * size => {
                // 1 -> 2
                pos.x = 3 * size - 1 - pos.x;
                pos.y = size;
                pos.dir = Facing::Down;
            }
            Facing::Up => {
                // 6 -> 4
                pos.y = 4 * size - 1 - pos.x;
                pos.x = 3 * size - 1;
                pos.dir = Facing::Left;
            }
            Facing::Down if pos.x < size => {
                // 2 -> 5
                pos.x = size - 1 - pos.x + 2 * size;
                pos.y = 3 * size - 1;
                pos.dir = Facing::Up;
            }
            Facing::Down if pos.x < 2 * size => {
                // 3 -> 5
                pos.y = 2 * size - 1 - pos.x + 2 * size;
                pos.x = 2 * size;
                pos.dir = Facing::Right;
            }
            Facing::Down if pos.x < 3 * size => {
                // 5 -> 2
                pos.x = 3 * size - 1 - pos.x;
                pos.y = 2 * size - 1;
                pos.dir = Facing::Up;
            }
            Facing::Down => {
                // 6 -> 2
                pos.y = 4 * size - 1 - pos.x + size;
                pos.x = 0;
                pos.dir = Facing::Right;
            }
            Facing::Left if pos.y < size => {
                // 1 -> 3
                pos.x = pos.y + size;
                pos.y = size;
                pos.dir = Facing::Down;
            }
            Facing::Left if pos.y < 2 * size => {
                // 2 -> 6
                pos.x = 2 * size - 1 - pos.y + 3 * size;
                pos.y = 3 * size - 1;
                pos.dir = Facing::Up;
            }
            Facing::Left => {
                // 5 -> 3
                pos.x = 3 * size - 1 - pos.y + size;
                pos.y = 2 * size - 1;
                pos.dir = Facing::Up;
            }
            Facing::Right if pos.y < size => {
                // 1 -> 6
                pos.x = 4 * size - 1;
                pos.y = size - 1 - pos.y + 3 * size;
                pos.dir = Facing::Left;
            }
            Facing::Right if pos.y < 2 * size => {
                // 4 -> 6
                pos.x = 2 * size - 1 - pos.y + 3 * size;
                pos.y = 2 * size;
                pos.dir = Facing::Down;
            }
            Facing::Right => {
                // 6 -> 1
                pos.x = 3 * size - 1;
                pos.y = 3 * size - 1 - pos.y;
                pos.dir = Facing::Left;
            }
        }
    }

    fn start(&self) -> MapPosition {
        let x = self
            .tiles
            .iter()
            .find_position(|x| **x == Tile::Open)
            .unwrap()
            .0;
        MapPosition {
            x,
            y: 0,
            dir: Facing::Right,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Rotation {
    Clockwise,
    CounterClockwise,
    None,
}

impl TryFrom<char> for Rotation {
    type Error = String;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'R' => Ok(Rotation::Clockwise),
            'L' => Ok(Rotation::CounterClockwise),
            _ => Err(format!("Could not parse {} as a Rotation", value)),
        }
    }
}

#[derive(Clone, Debug)]
struct Move {
    num: u32,
    dir: Rotation,
}

#[aoc_generator(day22)]
fn parse(input: &str) -> (Map, Vec<Move>) {
    let width = input
        .lines()
        .filter_map(|l| {
            if l.starts_with(&[' ', '.', '#']) {
                Some(l.len())
            } else {
                None
            }
        })
        .max()
        .unwrap();
    let mut map = Map::new(width);
    let mut lines = input.lines().enumerate();
    for (y, line) in &mut lines {
        if line.len() == 0 {
            break;
        }

        for (x, c) in line.chars().enumerate() {
            map.insert((x, y), c.try_into().unwrap());
        }
    }
    let (_, line) = lines.next().unwrap();
    let moves = line
        .chars()
        .group_by(|c| c.is_numeric())
        .into_iter()
        .chunks(2)
        .into_iter()
        .map(|chunk| {
            let mut chunk = chunk.into_iter();
            let num = chunk.next().unwrap().1.collect::<String>().parse().unwrap();
            let dir = chunk.next().map_or(Rotation::None, |(_, c)| {
                c.into_iter().next().unwrap().try_into().unwrap()
            });
            Move { num, dir }
        })
        .collect();
    (map, moves)
}

#[aoc(day22, part1)]
fn part1((map, moves): &(Map, Vec<Move>)) -> usize {
    let mut position = map.start();
    // dbg!(&position);
    for m in moves {
        // dbg!(m);
        position = map.do_move(position, m);
        // dbg!(&position);
    }
    position.password()
}

#[aoc(day22, part2)]
fn part2((map, moves): &(Map, Vec<Move>)) -> usize {
    let map = map.clone().into_cube();
    let mut position = map.start();
    // dbg!(&position);
    for m in moves {
        // dbg!(m);
        position = map.do_move(position, m);
        // dbg!(&position);
    }
    position.password()
}

#[cfg(test)]
const TEST_INPUT: &'static str = include_str!("../input/2022/day22_example.txt");

#[test]
fn part1_test() {
    assert_eq!(part1(&parse(&TEST_INPUT)), 6032);
}

#[test]
fn part2_test() {
    assert_eq!(part2(&parse(&TEST_INPUT)), 5031);
}
