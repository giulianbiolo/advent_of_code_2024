use std::collections::HashMap;

fn load_data(filename: &str) -> HashMap<u64, u64> {
    let data = std::fs::read_to_string(filename).expect("Failed to read file");
    data.split_whitespace()
        .map(|num| num.parse::<u64>().expect("Failed to parse number"))
        .map(|num| (num, 1))
        .collect()
}

fn single_blink(stone: u64) -> [i64; 2] {
    // ? Case when stone is 0
    if stone == 0 {
        return [1, -1];
    }
    // ? Case when stone has an even number of digits
    let digits_str: String = stone.to_string();
    let digits: std::str::Chars<'_> = digits_str.chars();
    let ndigits: usize = digits.clone().count();
    if ndigits % 2 == 0 {
        let left_stone: String = digits.clone().take(ndigits / 2).collect::<String>();
        let right_stone: String = digits.clone().skip(ndigits / 2).collect::<String>();
        let left_stone: u64 = left_stone.parse().expect("Couldn't parse left_stone");
        let right_stone: u64 = right_stone.parse().expect("Couldn't parse right_stone");
        return [left_stone as i64, right_stone as i64];
    }
    // ? Case when stone has an odd number of digits and is not 0
    return [stone as i64 * 2024, -1];
}

fn part_one(data: &HashMap<u64, u64>) -> u64 {
    let data: &mut HashMap<u64, u64> = &mut data.clone();

    for _ in 0..25 {
        let mut new_data: HashMap<u64, u64> = HashMap::new();
        for (stone, count) in data.iter() {
            let stones: [i64; 2] = single_blink(*stone);
            for stone in stones {
                if stone == -1 {
                    continue;
                }
                *new_data.entry(stone as u64).or_insert(0) += count;
            }
        }
        *data = new_data;
    }

    data.values().sum()
}

fn part_two(data: &HashMap<u64, u64>) -> u64 {
    let data: &mut HashMap<u64, u64> = &mut data.clone();

    for _ in 0..75 {
        let mut new_data: HashMap<u64, u64> = HashMap::new();
        for (stone, count) in data.iter() {
            let stones: [i64; 2] = single_blink(*stone);
            for stone in stones {
                if stone == -1 {
                    continue;
                }
                *new_data.entry(stone as u64).or_insert(0) += count;
            }
        }
        *data = new_data;
    }

    data.values().sum()
}

fn main() {
    let data: HashMap<u64, u64> = load_data("input.txt");
    println!("Part One: {}", part_one(&data));
    println!("Part Two: {}", part_two(&data));
}
