fn load_input() -> Vec<Vec<i32>> {
    let input = std::fs::read_to_string("input.txt").unwrap();
    input
        .lines()
        .map(|s| {
            s.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn is_safe_report(report: Vec<i32>) -> bool {
    let mut increasing: bool = true;
    let mut decreasing: bool = true;

    for i in 1..report.len() {
        if report[i] > report[i - 1] {
            decreasing = false;
        }
        if report[i] < report[i - 1] {
            increasing = false;
        }
        let diff: i32 = (report[i] - report[i - 1]).abs();
        if diff < 1 || diff > 3 {
            return false;
        }
    }
    increasing || decreasing
}

fn part_one(reports: &Vec<Vec<i32>>) -> i32 {
    reports
        .iter()
        .filter(|r| is_safe_report(r.to_vec()))
        .count() as i32
}

fn part_two(reports: &Vec<Vec<i32>>) -> i32 {
    let mut count: i32 = 0;
    for report in reports.iter() {
        for i in 0..report.len() {
            let mut new_report: Vec<i32> = report.to_vec();
            new_report.remove(i);
            if is_safe_report(new_report) {
                count += 1;
                break;
            }
        }
    }
    count
}

fn main() {
    let input: Vec<Vec<i32>> = load_input();
    let result: i32 = part_one(&input);
    println!("Part One: {}", result);
    let result: i32 = part_two(&input);
    println!("Part Two: {}", result);
}
