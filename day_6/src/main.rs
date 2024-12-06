use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug, Clone, Copy)]
enum Tile {
    Blank,
    Wall,
    Guard(Direction),
}

impl Tile {
    fn from_char(c: char) -> Tile {
        match c {
            '.' => Tile::Blank,
            '#' => Tile::Wall,
            '^' => Tile::Guard(Direction::Up),
            'v' => Tile::Guard(Direction::Down),
            '<' => Tile::Guard(Direction::Left),
            '>' => Tile::Guard(Direction::Right),
            _ => panic!("Invalid tile character"),
        }
    }
    fn to_char(&self) -> char {
        match self {
            Tile::Blank => '.',
            Tile::Wall => '#',
            Tile::Guard(Direction::Up) => '^',
            Tile::Guard(Direction::Down) => 'v',
            Tile::Guard(Direction::Left) => '<',
            Tile::Guard(Direction::Right) => '>',
        }
    }
}

type Map = Vec<Vec<Tile>>;
fn pretty_print(map: &Map) {
    for row in map {
        for tile in row {
            print!("{}", tile.to_char());
        }
        println!();
    }
}

fn load_data(filename: &str) -> Map {
    let input = std::fs::read_to_string(filename).expect("Failed to read input file");
    let mut map = Vec::new();
    for line in input.lines() {
        let row: Vec<Tile> = line.chars().map(|c| Tile::from_char(c)).collect();
        map.push(row);
    }
    map
}

fn part_one(map: &Map) -> usize {
    let mut visited: Vec<Vec<i32>> = vec![vec![0; map[0].len()]; map.len()];
    // ? Set the current guard location as visited
    let mut guard_pos: (usize, usize) = map
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .position(|tile| match tile {
                    Tile::Guard(_) => true,
                    _ => false,
                })
                .map(|x| (y, x))
        })
        .unwrap();
    visited[guard_pos.0][guard_pos.1] = 1;
    // ? Get the direction of the guard
    let mut guard_dir: &Direction = match &map[guard_pos.0][guard_pos.1] {
        Tile::Guard(dir) => dir,
        _ => panic!("Invalid guard position"),
    };

    loop {
        let next_pos: (isize, isize) = match guard_dir {
            Direction::Up => (guard_pos.0 as isize - 1, guard_pos.1 as isize),
            Direction::Down => (guard_pos.0 as isize + 1, guard_pos.1 as isize),
            Direction::Left => (guard_pos.0 as isize, guard_pos.1 as isize - 1),
            Direction::Right => (guard_pos.0 as isize, guard_pos.1 as isize + 1),
        };

        if next_pos.0 < 0
            || next_pos.0 >= map.len() as isize
            || next_pos.1 < 0
            || next_pos.1 >= map[0].len() as isize
        {
            break;
        }

        let next_pos: (usize, usize) = (next_pos.0 as usize, next_pos.1 as usize);
        match map[next_pos.0][next_pos.1] {
            Tile::Wall => {
                guard_dir = match guard_dir {
                    Direction::Up => &Direction::Right,
                    Direction::Down => &Direction::Left,
                    Direction::Left => &Direction::Up,
                    Direction::Right => &Direction::Down,
                };
                // ? We don't want to mark the wall as visited
            }
            _ => {
                visited[next_pos.0][next_pos.1] = 1;
                guard_pos = next_pos;
            }
        }
    }

    visited
        .iter()
        .map(|row| row.iter().filter(|&&x| x == 1).count())
        .sum()
}

fn part_two(map:&Map) -> usize {
    let mut loops: usize = 0;

    for r in 0..map.len() {
        for c in 0..map[0].len() {
            match map[r][c] { Tile::Wall => continue, Tile::Guard(_) => continue, _ => () }; // ? Skip walls and guard positions
            let mut position_set: HashSet<(usize, usize, Direction)> = HashSet::new();

            let mut visited: Vec<Vec<i32>> = vec![vec![0; map[0].len()]; map.len()];
            let mut guard_pos: (usize, usize) = map
                .iter()
                .enumerate()
                .find_map(|(y, row)| {
                    row.iter()
                        .position(|tile| match tile {
                            Tile::Guard(_) => true,
                            _ => false,
                        })
                        .map(|x| (y, x))
                })
                .unwrap();
            visited[guard_pos.0][guard_pos.1] = 1;
            let mut guard_dir: &Direction = match &map[guard_pos.0][guard_pos.1] {
                Tile::Guard(dir) => dir,
                _ => panic!("Invalid guard position"),
            };
            loop {
                if position_set.contains(&(guard_pos.0, guard_pos.1, *guard_dir)) {
                    loops += 1;
                    break;
                }
                position_set.insert((guard_pos.0, guard_pos.1, *guard_dir));
                let next_pos: (isize, isize) = match guard_dir {
                    Direction::Up => (guard_pos.0 as isize - 1, guard_pos.1 as isize),
                    Direction::Down => (guard_pos.0 as isize + 1, guard_pos.1 as isize),
                    Direction::Left => (guard_pos.0 as isize, guard_pos.1 as isize - 1),
                    Direction::Right => (guard_pos.0 as isize, guard_pos.1 as isize + 1),
                };
                if next_pos.0 < 0
                    || next_pos.0 >= map.len() as isize
                    || next_pos.1 < 0
                    || next_pos.1 >= map[0].len() as isize
                {
                    break;
                }
                let next_pos: (usize, usize) = (next_pos.0 as usize, next_pos.1 as usize);
                if (next_pos.0, next_pos.1) == (r, c) {
                    guard_dir = match guard_dir {
                        Direction::Up => &Direction::Right,
                        Direction::Down => &Direction::Left,
                        Direction::Left => &Direction::Up,
                        Direction::Right => &Direction::Down,
                    };
                    continue;
                }
                match map[next_pos.0][next_pos.1] {
                    Tile::Wall => {
                        guard_dir = match guard_dir {
                            Direction::Up => &Direction::Right,
                            Direction::Down => &Direction::Left,
                            Direction::Left => &Direction::Up,
                            Direction::Right => &Direction::Down,
                        };
                    }
                    _ => {
                        visited[next_pos.0][next_pos.1] = 1;
                        guard_pos = next_pos;
                    }
                }
            }
        }
    }

    loops
}

fn main() {
    let map: Map = load_data("input.txt");
    pretty_print(&map);
    println!("Part One: {}", part_one(&map));
    println!("Part Two: {}", part_two(&map));
}
