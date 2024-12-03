use std::fs;
use regex::Regex;


#[derive(Debug)]
enum MatchKind {
    Mul,
    Do,
    Dont,
}

#[derive(Debug)]
struct Match {
    kind: MatchKind,
    start: usize,
    a: u32,
    b: u32,
}

fn load_input(filename: &str) -> String {
    let contents: String = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    contents
}

fn part_one(memory: &str) -> u32 {
    let mul_re: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut result = 0;
    for cap in mul_re.captures_iter(memory) {
        let a: u32 = cap[1].parse::<u32>().unwrap();
        let b: u32 = cap[2].parse::<u32>().unwrap();
        result += a * b;
    }
    result
}

fn part_two(memory: &str) -> u32 {
    let mul_re: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let do_re: Regex = Regex::new(r"do\(\)").unwrap();
    let dont_re: Regex = Regex::new(r"don't\(\)").unwrap();

    let mut matches: Vec<Match> = Vec::new();

    for cap in mul_re.captures_iter(memory) {
        let start: usize = cap.get(0).unwrap().start();
        let a: u32 = cap[1].parse::<u32>().unwrap();
        let b: u32 = cap[2].parse::<u32>().unwrap();
        matches.push(Match { kind: MatchKind::Mul, start, a, b });
    }
    for cap in do_re.captures_iter(memory) {
        let start: usize = cap.get(0).unwrap().start();
        matches.push(Match { kind: MatchKind::Do, start, a: 0, b: 0 });
    }
    for cap in dont_re.captures_iter(memory) {
        let start: usize = cap.get(0).unwrap().start();
        matches.push(Match { kind: MatchKind::Dont, start, a: 0, b: 0 });
    }

    matches.sort_by(|a, b| a.start.cmp(&b.start));

    let mut result: u32 = 0;
    let mut active: bool = true;
    for m in matches {
        match m.kind {
            MatchKind::Mul => {
                if active {
                    result += m.a * m.b;
                }
            },
            MatchKind::Do => {
                active = true;
            },
            MatchKind::Dont => {
                active = false;
            },
        }
    }

    result
}

fn main() {
    let input: String = load_input("input.txt");
    let result: u32 = part_one(&input);
    println!("Part One: {}", result);
    let result: u32 = part_two(&input);
    println!("Part Two: {}", result);
}
