use regex::Regex;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}
const MAP_SIZE: Point = Point { x: 101, y: 103 };
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Robot {
    pos: Point,
    vel: Point,
}

fn load_data(filename: &str) -> Vec<Robot> {
    let mut robots: Vec<Robot> = Vec::new();
    let data: String = std::fs::read_to_string(filename).expect("Unable to read file");
    for line in data.lines() {
        let robot_re: Regex = Regex::new(r"p=(-?\d+),(-?\d+)\s+v=(-?\d+),(-?\d+)").unwrap();
        let caps: regex::Captures<'_> = robot_re.captures(line).unwrap();
        let pos: Point = Point {
            x: caps[1].parse().unwrap(),
            y: caps[2].parse().unwrap(),
        };
        let vel: Point = Point {
            x: caps[3].parse().unwrap(),
            y: caps[4].parse().unwrap(),
        };
        robots.push(Robot { pos, vel });
    }
    robots
}

fn sobel(map: &Vec<Vec<u64>>) -> Vec<Vec<char>> {
    let kernel_x: [[i64; 3]; 3] = [[-1, 0, 1], [-2, 0, 2], [-1, 0, 1]];
    let kernel_y: [[i64; 3]; 3] = [[-1, -2, -1], [0, 0, 0], [1, 2, 1]];
    let mut new_map: Vec<Vec<char>> = vec![vec!['.'; MAP_SIZE.x as usize]; MAP_SIZE.y as usize];
    let threshold: f64 = 4.0;

    for y in 0..MAP_SIZE.y {
        for x in 0..MAP_SIZE.x {
            if map[y as usize][x as usize] == 0 {
                continue;
            }
            let mut gx: i64 = 0;
            let mut gy: i64 = 0;
            for dy in -1..=1 {
                for dx in -1..=1 {
                    let nx: i64 = x + dx;
                    let ny: i64 = y + dy;
                    if nx < 0 || nx >= MAP_SIZE.x || ny < 0 || ny >= MAP_SIZE.y {
                        continue;
                    }
                    gx += kernel_x[(dy + 1) as usize][(dx + 1) as usize]
                        * map[ny as usize][nx as usize] as i64;
                    gy += kernel_y[(dy + 1) as usize][(dx + 1) as usize]
                        * map[ny as usize][nx as usize] as i64;
                }
            }
            let g: f64 = ((gx * gx + gy * gy) as f64).sqrt();
            if g > threshold {
                new_map[y as usize][x as usize] = '#';
            } else {
                new_map[y as usize][x as usize] = '.';
            }
        }
    }

    new_map
}

fn part_one(robots: &Vec<Robot>) -> u64 {
    let mut robots: Vec<Robot> = robots.clone();
    let mut map: Vec<Vec<u64>>;
    for _ in 0..100 {
        map = vec![vec![0; MAP_SIZE.x as usize]; MAP_SIZE.y as usize];
        for robot in robots.iter() {
            map[robot.pos.y as usize][robot.pos.x as usize] += 1;
        }
        for robot in robots.iter_mut() {
            robot.pos.x += robot.vel.x;
            robot.pos.y += robot.vel.y;
            if robot.pos.x < 0 {
                robot.pos.x += MAP_SIZE.x;
            }
            if robot.pos.x >= MAP_SIZE.x {
                robot.pos.x -= MAP_SIZE.x;
            }
            if robot.pos.y < 0 {
                robot.pos.y += MAP_SIZE.y;
            }
            if robot.pos.y >= MAP_SIZE.y {
                robot.pos.y -= MAP_SIZE.y;
            }
        }
    }
    let mut robots_in_quadrant: [u64; 4] = [0; 4];
    for robot in robots.iter() {
        if robot.pos.x == MAP_SIZE.x / 2 || robot.pos.y == MAP_SIZE.y / 2 {
            continue;
        }
        let quadrant: usize = if robot.pos.x < MAP_SIZE.x / 2 {
            if robot.pos.y < MAP_SIZE.y / 2 {
                0
            } else {
                2
            }
        } else {
            if robot.pos.y < MAP_SIZE.y / 2 {
                1
            } else {
                3
            }
        };
        robots_in_quadrant[quadrant] += 1;
    }
    robots_in_quadrant
        .iter()
        .fold(1, |acc, x| acc * (*x as u64))
}

fn part_two(robots: &Vec<Robot>) -> u64 {
    let mut robots: Vec<Robot> = robots.clone();
    let mut map: Vec<Vec<u64>>;
    let mut max_sobel: u64 = 0;
    let mut max_sobel_iter: u64 = 0;
    for it in 0..10000 {
        map = vec![vec![0; MAP_SIZE.x as usize]; MAP_SIZE.y as usize];
        for robot in robots.iter() {
            map[robot.pos.y as usize][robot.pos.x as usize] += 1;
        }
        let sobel_map: Vec<Vec<char>> = sobel(&map);
        let mut sobel: u64 = 0;
        for y in 0..MAP_SIZE.y {
            for x in 0..MAP_SIZE.x {
                if sobel_map[y as usize][x as usize] == '#' {
                    sobel += 1;
                }
            }
        }
        if sobel > max_sobel {
            max_sobel = sobel;
            max_sobel_iter = it;
        }
        for robot in robots.iter_mut() {
            robot.pos.x += robot.vel.x;
            robot.pos.y += robot.vel.y;
            if robot.pos.x < 0 {
                robot.pos.x += MAP_SIZE.x;
            }
            if robot.pos.x >= MAP_SIZE.x {
                robot.pos.x -= MAP_SIZE.x;
            }
            if robot.pos.y < 0 {
                robot.pos.y += MAP_SIZE.y;
            }
            if robot.pos.y >= MAP_SIZE.y {
                robot.pos.y -= MAP_SIZE.y;
            }
        }
    }

    max_sobel_iter
}

fn main() {
    let robots: Vec<Robot> = load_data("input.txt");
    println!("Part One: {}", part_one(&robots));
    println!("Part Two: {}", part_two(&robots));
}
