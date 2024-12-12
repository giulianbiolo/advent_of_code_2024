use std::collections::HashMap;

#[derive(Debug, Clone, Eq)]
struct Region {
    symbol: char,
    positions: Vec<(usize, usize)>,
}
impl PartialEq for Region {
    fn eq(&self, other: &Self) -> bool {
        self.symbol == other.symbol
            && self.positions.len() == other.positions.len()
            && self
                .positions
                .iter()
                .all(|(x, y)| other.positions.contains(&(*x, *y)))
    }
}
impl Region {
    fn area(&self) -> u64 {
        self.positions.len() as u64
    }
    fn perimeter(&self) -> u64 {
        let mut perimeter: u64 = 0;
        for (x, y) in &self.positions {
            if !self.positions.contains(&(x + 1, *y)) {
                perimeter += 1;
            }
            if !self.positions.contains(&(x - 1, *y)) {
                perimeter += 1;
            }
            if !self.positions.contains(&(*x, y + 1)) {
                perimeter += 1;
            }
            if !self.positions.contains(&(*x, y - 1)) {
                perimeter += 1;
            }
        }
        perimeter
    }
}
type Bucket = Vec<Region>;
type Map = HashMap<char, Bucket>;

fn load_data(filename: &str) -> Map {
    let mut regions: Vec<Vec<char>> = Vec::new();
    let data: String = std::fs::read_to_string(filename).expect("Error reading input file");
    for (y, line) in data.lines().enumerate() {
        for (_, c) in line.chars().enumerate() {
            if y >= regions.len() {
                regions.push(Vec::new());
            }
            regions[y].push(c);
        }
    }

    let mut map: Map = HashMap::new();

    // depth first search in the regions matrix to find the various regions
    let mut visited: Vec<(usize, usize)> = Vec::new();
    let max_x: usize = regions[0].len();
    let max_y: usize = regions.len();

    fn dfs(
        x: usize,
        y: usize,
        symb: char,
        visited: &mut Vec<(usize, usize)>,
        max_x: usize,
        max_y: usize,
        regions: Vec<Vec<char>>,
    ) -> Vec<(usize, usize)> {
        let mut stack: Vec<(usize, usize)> = vec![(x, y)];
        let mut current_region: Vec<(usize, usize)> = Vec::new();
        while stack.len() > 0 {
            let (x, y) = stack.pop().unwrap();
            if visited.contains(&(x, y)) {
                continue;
            }
            visited.push((x, y));
            current_region.push((x, y));
            for (dx, dy) in vec![(1, 0), (-1, 0), (0, -1), (0, 1)] {
                let new_x = x as i32 + dx;
                let new_y = y as i32 + dy;
                if new_x >= 0 && new_x < max_x as i32 && new_y >= 0 && new_y < max_y as i32 {
                    if regions[new_y as usize][new_x as usize] == symb
                        && !visited.contains(&(new_x as usize, new_y as usize))
                    {
                        stack.push((new_x as usize, new_y as usize));
                    }
                }
            }
        }
        current_region
    }

    for y in 0..max_y {
        for x in 0..max_x {
            let symb = regions[y][x];
            if visited.contains(&(x, y)) {
                continue;
            }
            let new_region: Vec<(usize, usize)> =
                dfs(x, y, symb, &mut visited, max_x, max_y, regions.clone());
            let region: Region = Region {
                symbol: symb,
                positions: new_region,
            };
            if map.contains_key(&symb) {
                map.get_mut(&symb).unwrap().push(region);
            } else {
                map.insert(symb, vec![region]);
            }
        }
    }

    map
}

fn part_one(map: &Map) -> u64 {
    map.values()
        .map(|bucket| {
            let mut fence_price: u64 = 0;
            for region in bucket {
                fence_price += region.area() * region.perimeter();
            }
            fence_price
        })
        .sum()
}

fn main() {
    let map: Map = load_data("input.txt");
    println!("Part One: {}", part_one(&map));
}
