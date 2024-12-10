use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy)]
enum Tile {
    Empty,
    Antenna(char),
    Antinode,
}
impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Tile::Empty => write!(f, "."),
            Tile::Antenna(c) => write!(f, "{}", c),
            Tile::Antinode => write!(f, "#"),
        }
    }
}
impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Tile::Empty => write!(f, "."),
            Tile::Antenna(c) => write!(f, "{}", c),
            Tile::Antinode => write!(f, "#"),
        }
    }
}
impl Tile {
    fn is_empty(&self) -> bool {
        match self {
            Tile::Empty => true,
            _ => false,
        }
    }
    fn is_antenna(&self) -> bool {
        match self {
            Tile::Antenna(_) => true,
            _ => false,
        }
    }
    fn get_antenna(&self) -> char {
        match self {
            Tile::Antenna(c) => *c,
            _ => panic!("Tile is not an antenna"),
        }
    }
}
impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Tile::Empty, Tile::Empty) => true,
            (Tile::Antenna(c1), Tile::Antenna(c2)) => c1 == c2,
            (Tile::Antinode, Tile::Antinode) => true,
            _ => false,
        }
    }
}
impl Eq for Tile {}
type Map = Vec<Vec<Tile>>;
fn distance(p1: (usize, usize), p2: (usize, usize)) -> usize {
    ((p1.0 as i32 - p2.0 as i32).abs() + (p1.1 as i32 - p2.1 as i32).abs()) as usize
}

fn load_data(filename: &str) -> Map {
    let input = std::fs::read_to_string(filename).expect("Failed to read input file");
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Empty,
                    _ => Tile::Antenna(c),
                })
                .collect()
        })
        .collect()
}

fn part_one(map: &Map) -> usize {
    // ? Given a character of an antenna, i want to get a list of (x,y) coordinates where those antennas are located at.
    let mut final_map: Map = map.clone();
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (x, row) in map.iter().enumerate() {
        for (y, tile) in row.iter().enumerate() {
            if tile.is_antenna() {
                let antenna = tile.get_antenna();
                let coords = antennas.entry(antenna).or_insert(vec![]);
                coords.push((x, y));
            }
        }
    }
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    for (x, row) in map.iter().enumerate() {
        for (y, tile) in row.iter().enumerate() {
            // ? For each tile we compute the distance_antennas map where each character of an antenna is mapped to a list of distances from that tile to the antennas.
            let distances: HashMap<char, Vec<usize>> = antennas
                .iter()
                .map(|(antenna, coords)| {
                    let dists: Vec<usize> = coords
                        .iter()
                        .map(|coord| distance((x, y), *coord))
                        .collect();
                    (*antenna, dists)
                })
                .collect();
            
            // ? now distances = {'0': [1, 2, 3], '1': [2, 3, 4], '2': [3, 4, 5], ...}
            for (antenna, dists) in distances.iter() {
                for (i, d1) in dists.iter().enumerate() {
                    for (j, d2) in dists.iter().skip(i + 1).enumerate() {
                        if d1 * 2 == *d2 || d2 * 2 == *d1 {
                            // ? if the two antennas are in line with one another and with the tile then it is in fact an antinode.
                            let (x1, y1) = antennas[antenna][i];
                            let (x2, y2) = antennas[antenna][j + i + 1];
                            if (x1 as f64 - x as f64) / (y1 as f64 - y as f64) == (x2 as f64 - x as f64) / (y2 as f64 - y as f64) {
                                antinodes.insert((x, y));
                                if final_map[x][y].is_empty() {
                                    final_map[x][y] = Tile::Antinode;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    antinodes.len()
}

fn part_two(map: &Map) -> usize {
    let mut final_map: Map = map.clone();
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (x, row) in map.iter().enumerate() {
        for (y, tile) in row.iter().enumerate() {
            if tile.is_antenna() {
                let antenna = tile.get_antenna();
                let coords = antennas.entry(antenna).or_insert(vec![]);
                coords.push((x, y));
            }
        }
    }
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    for (x, row) in map.iter().enumerate() {
        for (y, tile) in row.iter().enumerate() {
            let distances: HashMap<char, Vec<usize>> = antennas
                .iter()
                .map(|(antenna, coords)| {
                    let dists: Vec<usize> = coords
                        .iter()
                        .map(|coord| distance((x, y), *coord))
                        .collect();
                    (*antenna, dists)
                })
                .collect();

            // ? now distances = {'0': [1, 2, 3], '1': [2, 3, 4], '2': [3, 4, 5], ...}
            for (antenna, dists) in distances.iter() {
                for (i, d1) in dists.iter().enumerate() {
                    for (j, d2) in dists.iter().skip(i + 1).enumerate() {
                        // ? if the two antennas are in line with one another and with the tile then it is in fact an antinode.
                        let (x1, y1) = antennas[antenna][i];
                        let (x2, y2) = antennas[antenna][j + i + 1];
                        // ? Also consider the antenna itself as a possible antinode.
                        if (x1 as f64 - x as f64) / (y1 as f64 - y as f64) == (x2 as f64 - x as f64) / (y2 as f64 - y as f64) {
                            antinodes.insert((x, y));
                            if final_map[x][y].is_empty() {
                                final_map[x][y] = Tile::Antinode;
                            }
                        }
                    }
                }
            }

            // ? Also count all the occurrences of antennas that are not unique as antinodes.
            for (antenna, dists) in distances.iter() {
                if dists.len() > 1 {
                    antennas[antenna].iter().for_each(|coord| {
                        antinodes.insert(*coord);
                        if final_map[coord.0][coord.1].is_empty() {
                            final_map[coord.0][coord.1] = Tile::Antinode;
                        }
                    });
                }
            }
        }
    }

    antinodes.len()
}

fn main() {
    let map: Map = load_data("input.txt");
    println!("Part one: {}", part_one(&map));
    println!("Part two: {}", part_two(&map));
}
