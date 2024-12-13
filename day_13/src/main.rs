use nalgebra::{Matrix, Matrix2};
use regex::Regex;

#[derive(Debug, Copy, Clone)]
struct Prize {
    btn_a: (i64, i64),
    btn_b: (i64, i64),
    pos: (i64, i64),
}

fn load_data(filename: &str) -> Vec<Prize> {
    let data: String = std::fs::read_to_string(filename).expect("Failed to read file");
    let lines: Vec<&str> = data.lines().collect();
    let mut prizes: Vec<Prize> = Vec::new();

    let btn_re: Regex = Regex::new(r"X\+(\d+), Y\+(\d+)").unwrap();
    let pos_re: Regex = Regex::new(r"X=(\d+), Y=(\d+)").unwrap();
    for idx in (0..lines.len()).step_by(4) {
        let mut prize: Prize = Prize {
            btn_a: (0, 0),
            btn_b: (0, 0),
            pos: (0, 0),
        };
        for i in 0..3 {
            let caps = match i {
                0 | 1 => btn_re.captures(lines[idx + i]).unwrap(),
                2 => pos_re.captures(lines[idx + i]).unwrap(),
                _ => panic!("Invalid index"),
            };
            let x: i64 = caps.get(1).unwrap().as_str().parse().unwrap();
            let y: i64 = caps.get(2).unwrap().as_str().parse().unwrap();
            match i {
                0 => prize.btn_a = (x, y),
                1 => prize.btn_b = (x, y),
                2 => prize.pos = (x, y),
                _ => (),
            }
        }
        prizes.push(prize);
    }

    prizes
}

fn part_one(prizes: &Vec<Prize>) -> i64 {
    let mut sum: i64 = 0;
    for (idx, prize) in prizes.iter().enumerate() {
        let mut min_cost = std::i64::MAX;
        for i in 0..100 {
            for j in 0..100 {
                let x = prize.btn_a.0 * i + prize.btn_b.0 * j;
                let y = prize.btn_a.1 * i + prize.btn_b.1 * j;
                if x == prize.pos.0 && y == prize.pos.1 {
                    let cost: i64 = i * 3 + j;
                    if cost < min_cost {
                        min_cost = cost;
                    }
                }
            }
        }
        if min_cost < std::i64::MAX {
            sum += min_cost;
        }
    }

    sum
}

fn part_two(prizes: &Vec<Prize>) -> i128 {
    let mut sum: i128 = 0;
    for (idx, prize) in prizes.iter().enumerate() {
        let prize: Prize = Prize {
            btn_a: (prize.btn_a.0, prize.btn_a.1),
            btn_b: (prize.btn_b.0, prize.btn_b.1),
            pos: (
                prize.pos.0 + 10_000_000_000_000,
                prize.pos.1 + 10_000_000_000_000,
            ),
        };

        let at_bt: nalgebra::Matrix<
            f64,
            nalgebra::Const<2>,
            nalgebra::Const<2>,
            nalgebra::ArrayStorage<f64, 2, 2>,
        > = Matrix2::new(
            prize.btn_a.0 as f64,
            prize.btn_b.0 as f64,
            prize.btn_a.1 as f64,
            prize.btn_b.1 as f64,
        );
        let at_bt_inv: Matrix<
            f64,
            nalgebra::Const<2>,
            nalgebra::Const<2>,
            nalgebra::ArrayStorage<f64, 2, 2>,
        > = at_bt.try_inverse().unwrap();

        let pos: Matrix<
            f64,
            nalgebra::Const<2>,
            nalgebra::Const<1>,
            nalgebra::ArrayStorage<f64, 2, 1>,
        > = nalgebra::Vector2::new(prize.pos.0 as f64, prize.pos.1 as f64);

        let at_bt_inv_pos: Matrix<
            f64,
            nalgebra::Const<2>,
            nalgebra::Const<1>,
            nalgebra::ArrayStorage<f64, 2, 1>,
        > = at_bt_inv * pos;

        let i: i128 = at_bt_inv_pos[0].round() as i128;
        let j: i128 = at_bt_inv_pos[1].round() as i128;

        // ? Check if the Inverse Matrix was an actual inverse or just a pseudo-inverse
        if (prize.btn_a.0 as i128 * i + prize.btn_b.0 as i128 * j != prize.pos.0 as i128)
            || (prize.btn_a.1 as i128 * i + prize.btn_b.1 as i128 * j != prize.pos.1 as i128)
        {
            continue;
        }
        let cost: i128 = i * 3 + j;
        sum += cost;
    }

    sum
}

fn main() {
    let prizes: Vec<Prize> = load_data("input.txt");
    println!("Part One: {}", part_one(&prizes));
    println!("Part Two: {}", part_two(&prizes));
}
