type Update = Vec<i32>;

#[derive(Debug, Clone, Copy)]
struct Rule {
    pred: i32,
    succ: i32,
}
impl Rule {
    fn new(pred: i32, succ: i32) -> Rule {
        Rule { pred, succ }
    }
    fn index(self, update: &Update) -> (i32, i32) {
        let mut pred_index: i32 = -1;
        let mut succ_index: i32 = -1;
        for i in 0..update.len() {
            if update[i] == self.pred {
                pred_index = i as i32;
            }
            if update[i] == self.succ {
                succ_index = i as i32;
            }
        }
        (pred_index, succ_index)
    }
}

fn load_data(filename: &str) -> (Vec<Rule>, Vec<Update>) {
    let data: String = std::fs::read_to_string(filename).expect("Error reading input file");
    let mut lines: std::str::Lines<'_> = data.lines();
    let mut rules: Vec<Rule> = Vec::new();
    let mut updates: Vec<Update> = Vec::new();
    while let Some(line) = lines.next() {
        if line.trim().is_empty() {
            break;
        }
        let mut parts: std::str::Split<'_, &str> = line.split("|");
        let pred: i32 = parts.next().unwrap().parse::<i32>().unwrap();
        let succ: i32 = parts.next().unwrap().parse::<i32>().unwrap();
        rules.push(Rule::new(pred, succ));
    }
    while let Some(line) = lines.next() {
        let update: Update = line.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
        updates.push(update);
    }
    (rules, updates)
}

fn part_one(rules: &Vec<Rule>, updates: &Vec<Update>) -> i32 {
    let mut count: i32 = 0;

    for update in updates {
        let mut valid: bool = true;
        for rule in rules {
            let (pred_index, succ_index) = rule.index(&update);
            if pred_index == -1 || succ_index == -1 {
                continue;
            }
            if succ_index < pred_index {
                valid = false;
                break;
            }
        }
        if valid {
            count += update[(update.len() - 1) / 2];
        }
    }

    count
}

fn part_two(rules: &Vec<Rule>, updates: &Vec<Update>) -> i32 {
    let mut count: i32 = 0;

    for upd_idx in 0..updates.len() {
        let mut valid: bool = true;
        let mut update: Vec<i32> = updates[upd_idx].clone();

        let mut idx: usize = 0;
        while idx < rules.len() {
            let (pred_index, succ_index) = rules[idx].index(&update);
            if pred_index == -1 || succ_index == -1 {
                idx += 1;
                continue;
            }
            if succ_index < pred_index {
                valid = false;

                let temp: i32 = update[pred_index as usize];
                update[pred_index as usize] = update[succ_index as usize];
                update[succ_index as usize] = temp;

                idx = 0;
            }
            idx += 1;
        }
        if !valid {
            count += update[(update.len() - 1) / 2];
        }
    }

    count
}

fn main() {
    let (rules, updates) = load_data("input.txt");
    println!("Part One: {}", part_one(&rules, &updates));
    println!("Part Two: {}", part_two(&rules, &updates));
}
