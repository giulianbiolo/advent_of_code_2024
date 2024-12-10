use std::collections::HashMap;

type Map = Vec<Vec<u32>>;

fn load_data(filename: &str) -> Map {
    let data = std::fs::read_to_string(filename).expect("Failed to read file");
    data.lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '0'..='9' => c.to_digit(10).unwrap(),
                    '.' => 15,
                    _ => panic!("Invalid character"),
                })
                .collect()
        })
        .collect()
}

fn part_one(map: &Map) -> u32 {
    let mut reachable_goals: HashMap<(u32, u32), Vec<(u32, u32)>> = HashMap::<(u32, u32), Vec<(u32, u32)>>::new();

    fn traverse(map: &Map, reachable_goals: &mut HashMap<(u32, u32), Vec<(u32, u32)>>, x: usize, y: usize, height: u32, starting: (u32, u32)) -> u32 {
        if height == 9 {
            reachable_goals.entry(starting).or_insert(Vec::new()).push((x as u32, y as u32));
            return 1;
        }
        let mut paths: u32 = 0;
        
        if x > 0 && map[y][x - 1] == height + 1 {
            paths += traverse(map, reachable_goals, x - 1, y, height + 1, starting);
        }
        if x < map[0].len() - 1 && map[y][x + 1] == height + 1 {
            paths += traverse(map, reachable_goals,x + 1, y, height + 1, starting);
        }
        if y > 0 && map[y - 1][x] == height + 1 {
            paths += traverse(map, reachable_goals, x, y - 1, height + 1, starting);
        }
        if y < map.len() - 1 && map[y + 1][x] == height + 1 {
            paths += traverse(map, reachable_goals, x, y + 1, height + 1, starting);
        }
        paths
    }

    let trailheads: Vec<(usize, usize)> = map
        .iter()
        .enumerate()
        .filter_map(|(y, row)| {
            Some(
                row.iter()
                    .enumerate()
                    .filter_map(|(x, &cell)| if cell == 0 { Some((x, y)) } else { None })
                    .collect::<Vec<_>>(),
            )
        })
        .flatten()
        .collect::<Vec<_>>();

    // now traverse each of those trailheads recursively finding the paths.
    for (x, y) in trailheads {
        traverse(map, &mut reachable_goals, x, y, 0, (x as u32, y as u32));
    }

    reachable_goals.iter_mut().for_each(|(_, v)| {
        v.sort();
        v.dedup();
    });

    reachable_goals.iter().fold(0, |acc, (_, v)| acc + v.len() as u32)
}

fn part_two(map: &Map) -> u32 {
    fn traverse(map: &Map, x: usize, y: usize, height: u32) -> u32 {
        if height == 9 {
            return 1;
        }
        let mut paths: u32 = 0;
        
        if x > 0 && map[y][x - 1] == height + 1 {
            paths += traverse(map, x - 1, y, height + 1);
        }
        if x < map[0].len() - 1 && map[y][x + 1] == height + 1 {
            paths += traverse(map,x + 1, y, height + 1);
        }
        if y > 0 && map[y - 1][x] == height + 1 {
            paths += traverse(map, x, y - 1, height + 1);
        }
        if y < map.len() - 1 && map[y + 1][x] == height + 1 {
            paths += traverse(map, x, y + 1, height + 1);
        }
        paths
    }

    let trailheads: Vec<(usize, usize)> = map
        .iter()
        .enumerate()
        .filter_map(|(y, row)| {
            Some(
                row.iter()
                    .enumerate()
                    .filter_map(|(x, &cell)| if cell == 0 { Some((x, y)) } else { None })
                    .collect::<Vec<_>>(),
            )
        })
        .flatten()
        .collect::<Vec<_>>();

    trailheads.iter()
        .map(|(x, y)| traverse(map, *x, *y, 0))
        .sum()
}


fn main() {
    let map = load_data("input.txt");
    println!("Part One: {}", part_one(&map));
    println!("Part Two: {}", part_two(&map));
}
