#[derive(Debug)]
struct Equation {
    result: u64,
    factors: Vec<u64>,
}

fn load_data(filename: &str) -> Vec<Equation> {
    let input = std::fs::read_to_string(filename).expect("Failed to read input file");
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(":");
            let result: u64 = parts
                .next()
                .expect("Couldn't get split(':')[0]")
                .parse::<u64>()
                .expect("Couldn't parse to i64 the split(':')[0]");
            let factors: Vec<u64> = parts
                .next()
                .expect("Couldn't get split(':')[1]")
                .split_whitespace()
                .map(|n| {
                    n.parse::<u64>()
                        .expect("Couldn't parse to i64 the split(':')[1]")
                })
                .collect();
            Equation { result, factors }
        })
        .collect()
}

fn part_one(equations: &Vec<Equation>) -> u64 {
    fn is_valid(eq: &Equation) -> bool {
        let facts: Vec<u64> = eq.factors.clone();
        let mut factors: std::slice::Iter<'_, u64> = facts.iter();
        let mut possibles: Vec<u64> = vec![factors.next().unwrap().clone()];
        while let Some(curr) = factors.next() {
            let mut temp: Vec<u64> = vec![];
            for p in possibles {
                let next_values: Vec<u64> = vec![
                    p + curr,
                    p * curr,
                ];
                temp.extend(next_values.iter().filter(|v| **v <= eq.result).cloned());
            }
            possibles = temp;
        }
        possibles.contains(&eq.result)
    }
    equations
        .iter()
        .filter(|eq| is_valid(&eq))
        .map(|eq| eq.result)
        .sum()
}

fn part_two(equations: &Vec<Equation>) -> u64 {
    fn is_valid(eq: &Equation) -> bool {
        let facts: Vec<u64> = eq.factors.clone();
        let mut factors: std::slice::Iter<'_, u64> = facts.iter();
        let mut possibles: Vec<u64> = vec![factors.next().unwrap().clone()];
        while let Some(curr) = factors.next() {
            let mut temp: Vec<u64> = vec![];
            for p in possibles {
                let next_values: Vec<u64> = vec![
                    p + curr,
                    p * curr,
                    format!("{}{}", p, curr).parse::<u64>().unwrap(),
                ];
                temp.extend(next_values.iter().filter(|v| **v <= eq.result).cloned());
            }
            possibles = temp;
        }
        possibles.contains(&eq.result)
    }
    equations
        .iter()
        .filter(|eq| is_valid(&eq))
        .map(|eq| eq.result)
        .sum()
}

fn main() {
    let equations: Vec<Equation> = load_data("input.txt");
    println!("Part One: {}", part_one(&equations));
    println!("Part Two: {}", part_two(&equations));
}
