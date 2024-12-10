struct Equation {
    result: u64,
    factors: Vec<u64>
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
        let combinations = 2_u64.pow(eq.factors.len() as u32);
        for i in 0..combinations as u64 {
            let mut sum: u64 = 0_u64;
            for j in 0..eq.factors.len() {
                match i as u64 / 2_u64.pow(j as u32) % 2 {
                    0 => sum += eq.factors[j],
                    1 => sum *= eq.factors[j],
                    _ => panic!("Invalid operation"),
                }
            }
            if sum as u64 == eq.result {
                return true;
            }
        }
        false
    }
    equations
        .iter()
        .filter(|eq| is_valid(&eq))
        .map(|eq| eq.result as u64)
        .sum()
}

fn part_two(equations: &Vec<Equation>) -> u64 {
    fn is_valid(eq: &Equation) -> bool {
        let combinations = 3_u64.pow(eq.factors.len() as u32);
        for i in 0..combinations as u64 {
            let mut sum: u64 = eq.factors[0];
            let mut operations: String = String::new();
            for j in 1..eq.factors.len() {
                match i as u64 / 3_u64.pow(j as u32) % 3 {
                    0 => {sum += eq.factors[j]; operations.push_str("+");},
                    1 => {sum *= eq.factors[j]; operations.push_str("*");},
                    2 => {
                        let mut s: String = sum.to_string();
                        s.push_str(&eq.factors[j].to_string());
                        sum = s.parse::<u64>().unwrap();
                        operations.push_str("|");
                    }
                    _ => panic!("Invalid operation"),
                }
            }
            if sum as u64 == eq.result {
                println!("Equation: {:?} : {} = {}", eq.factors, operations, sum);
                return true;
            }
        }
        false
    }
    equations
        .iter()
        .filter(|eq| is_valid(&eq))
        .map(|eq| eq.result as u64)
        .sum()
}

fn main() {
    let equations: Vec<Equation> = load_data("input.txt");
    println!("Part One: {}", part_one(&equations));
    println!("Part Two: {}", part_two(&equations));
}
