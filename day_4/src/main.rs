use regex::Regex;
use diagonal::{diagonal_pos_pos, diagonal_pos_neg};


type Map = Vec<Vec<u8>>;


fn parse_input(input: &str) -> Map {
    std::fs::read_to_string(input)
        .expect("Failed to read input file")
        .trim()
        .split('\n')
        .map(|line| line.chars().filter(|x| *x != '\r').map(|x| x as u8).collect())
        .collect()
}

fn part_one(map: &Map) -> usize {
    let mut counter: usize = 0;

    let xmas_re: Regex = Regex::new(r"XMAS").unwrap();
    let samx_re: Regex = Regex::new(r"SAMX").unwrap();
    
    // ? Horizontal
    for x in 0..map.len() {
        counter += xmas_re.find_iter(&map[x].iter().map(|x| *x as char).collect::<String>()).count();
        counter += samx_re.find_iter(&map[x].iter().map(|x| *x as char).collect::<String>()).count();
    }
    // ? Vertical
    for y in 0..map[0].len() {
        let mut vertical: String = String::new();
        for x in 0..map.len() {
            vertical.push(map[x][y] as char);
        }
        counter += xmas_re.find_iter(&vertical).count();
        counter += samx_re.find_iter(&vertical).count();
    }

    // ? Diagonals
    let pos_diagonals: Vec<Vec<&u8>> = diagonal_pos_pos(&map);
    let neg_diagonals: Vec<Vec<&u8>> = diagonal_pos_neg(&map);
    let diagonals: Map = pos_diagonals.iter().chain(neg_diagonals.iter()).map(|x| x.iter().map(|x| **x).collect()).collect();

    for x in 0..diagonals.len() {
        counter += xmas_re.find_iter(&diagonals[x].iter().map(|x| *x as char).collect::<String>()).count();
        counter += samx_re.find_iter(&diagonals[x].iter().map(|x| *x as char).collect::<String>()).count();
    }

    counter
}

fn part_two(map: &Map) -> usize {
    let mut counter: usize = 0;

    for x in 1..map.len() - 1 {
        for y in 1..map[0].len() - 1 {
            let diagonals: Vec<Vec<&u8>> = vec![
                vec![&map[x - 1][y - 1], &map[x][y], &map[x + 1][y + 1]],
                vec![&map[x - 1][y + 1], &map[x][y], &map[x + 1][y - 1]],
            ];
            if diagonals.iter().all(|x| {
                let diag: String = x.iter().map(|x| **x as char).collect();
                diag == "MAS" || diag == "SAM"
            }) { counter += 1; }
        }
    }

    counter
}

fn main() {
    let map: Map = parse_input("input.txt");
    println!("Part One: {}", part_one(&map));
    println!("Part Two: {}", part_two(&map));
}
