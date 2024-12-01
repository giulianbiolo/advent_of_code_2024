use std::fs;

fn read_input(filename: &str) -> (Vec<i32>, Vec<i32>) {
    fs::read_to_string(filename)
        .expect("Unable to read file")
        .lines()
        .map(|line| {
            let mut numbers: std::str::SplitWhitespace<'_> = line.split_whitespace();
            let n1: i32 = numbers.next().unwrap_or("-1").parse::<i32>().unwrap_or(-1);
            let n2: i32 = numbers.next().unwrap_or("-1").parse::<i32>().unwrap_or(-1);
            (n1, n2)
        })
        .filter_map(|(n1, n2)| {
            if n1 == -1 || n2 == -1 {
                None
            } else {
                Some((n1 as i32, n2 as i32))
            }
        })
        .unzip()
}

fn part_one(n1: &Vec<i32>, n2: &Vec<i32>) -> u32 {
    let mut sum: u32 = 0;
    for (i, j) in n1.iter().zip(n2.iter()) {
        sum += (i - j).abs() as u32;
    }
    sum
}

fn part_two(n1: &Vec<i32>, n2: &Vec<i32>) -> u32 {
    // ? We first build an HashMap over n2 to store the number of occurrences of each number
    let mut n2_map: std::collections::HashMap<i32, u32> = std::collections::HashMap::new();
    for i in n2.iter() {
        let count = n2_map.entry(*i).or_insert(0);
        *count += 1;
    }
    // ? Then we iterate over n1 and sum the product of each number by its number of occurrences in n2
    let mut sum: u32 = 0;
    for i in n1.iter() {
        let count = n2_map.entry(*i).or_insert(0);
        sum += (*i as u32) * (*count as u32);
    }
    sum
}

fn main() {
    let (mut n1, mut n2): (Vec<i32>, Vec<i32>) = read_input("input.txt");
    n1.sort(); n2.sort();
    let part_one: u32 = part_one(&n1, &n2);
    println!("Part One: {}", part_one);
    let part_two: u32 = part_two(&n1, &n2);
    println!("Part Two: {}", part_two);
}
