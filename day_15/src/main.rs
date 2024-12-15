#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Empty,
    Box,
    Robot,
}
impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let c = match self {
            Tile::Wall => '#',
            Tile::Empty => '.',
            Tile::Box => 'O',
            Tile::Robot => '@',
        };
        write!(f, "{}", c)
    }
}
type Map = Vec<Vec<Tile>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn to_offset(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Robot {
    x: usize,
    y: usize,
    path: Vec<Direction>,
}

fn load_data(filename: &str) -> (Map, Robot) {
    let data: String = std::fs::read_to_string(filename).expect("Failed to read data file!");
    let mut map: Vec<Vec<Tile>> = Vec::new();
    let mut lines: std::str::Lines<'_> = data.lines();
    let mut robot_pos: (i32, i32) = (0, 0);
    loop {
        let line: &str = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        let mut row: Vec<Tile> = Vec::new();
        for c in line.chars() {
            let tile = match c {
                '#' => Tile::Wall,
                '.' => Tile::Empty,
                'O' => Tile::Box,
                '@' => Tile::Robot,
                _ => panic!("Invalid tile character!"),
            };
            if tile == Tile::Robot {
                robot_pos = (row.len() as i32, map.len() as i32);
            }
            row.push(tile);
        }
        map.push(row);
    }

    let res: String = lines.collect();
    let path: Vec<Direction> = res
        .chars()
        .map(|c| match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Invalid direction character!"),
        })
        .collect();

    let robot: Robot = Robot {
        x: robot_pos.0 as usize,
        y: robot_pos.1 as usize,
        path,
    };

    (map, robot)
}

fn part_one(map: &Map, robot: &Robot) -> u64 {
    // ? Simulate the movement of the robot inside of the map following the path.
    // ? It moves around boxes but not walls.
    let mut map: Map = map.clone();
    let mut robot: Robot = robot.clone();

    for dir in robot.path.iter() {
        let (dx, dy) = dir.to_offset();
        let (nx, ny) = (robot.x as isize + dx, robot.y as isize + dy);
        let (nx, ny) = (nx as usize, ny as usize);
        match map[ny][nx] {
            Tile::Wall => continue,
            Tile::Empty => {
                map[robot.y][robot.x] = Tile::Empty;
                robot.x = nx;
                robot.y = ny;
                map[ny][nx] = Tile::Robot;
            }
            Tile::Box => {
                let mut boxes: Vec<(usize, usize)> = Vec::new();
                let mut bx: isize = nx as isize;
                let mut by: isize = ny as isize;
                let mut wall: bool = false;
                loop {
                    bx = bx as isize + dx;
                    by = by as isize + dy;
                    if bx < 0
                        || by < 0
                        || by >= map.len() as isize
                        || bx >= map[by as usize].len() as isize
                    {
                        break;
                    }
                    let (bx, by) = (bx as usize, by as usize);
                    match map[by][bx] {
                        Tile::Wall => {
                            wall = true;
                            break;
                        }
                        Tile::Empty => {
                            boxes.push((bx, by));
                            break;
                        }
                        Tile::Box => {
                            boxes.push((bx, by));
                        }
                        _ => break,
                    }
                }
                if boxes.is_empty() || wall {
                    continue;
                }
                map[robot.y][robot.x] = Tile::Empty;
                robot.x = robot.x + dx as usize;
                robot.y = robot.y + dy as usize;
                map[robot.y][robot.x] = Tile::Robot;
                for (bx, by) in boxes.iter() {
                    map[*by][*bx] = Tile::Box;
                }
            }
            _ => continue,
        }
    }
    map.iter()
        .enumerate()
        .map(|(r, row)| {
            row.iter()
                .enumerate()
                .map(|(c, tile)| {
                    if *tile == Tile::Box {
                        return (100 * r + c) as u64;
                    }
                    return 0;
                })
                .sum::<u64>()
        })
        .sum()
}

fn main() {
    let (map, robot) = load_data("input.txt");
    println!("Part One: {}", part_one(&map, &robot));
}
