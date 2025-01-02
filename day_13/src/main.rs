use std::fs::read_to_string;
use regex::Regex;

fn main() {
    let lines: Vec<String> = read_to_string("./puzzle_input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let gachas = parse_input(lines.clone(), false);
    let mut total_tokens: u64 = 0;
    for gacha in gachas {
        total_tokens += determine_minimum_spend(gacha);
    }
    println!("Total tokens for part 1: {total_tokens}");

    let gachas_part2 = parse_input(lines, true);
    total_tokens = 0;
    for gacha in gachas_part2 {
        total_tokens += determine_minimum_spend(gacha);
    }
    println!("Total tokens for part 2: {total_tokens}");
}

fn determine_minimum_spend(gacha: [f64; 6]) -> u64 {
    let mut tokens: u64 = 0;
    let x1 = gacha[0];
    let x2 = gacha[2];
    let xf = gacha[4];
    let y1 = gacha[1];
    let y2 = gacha[3];
    let yf = gacha[5];

    let b: f64 = (xf - x1 * (yf / y1)) / (x2 - x1 * (y2 / y1 ));
    let a: f64 = (xf - x2 * b) / x1;

    if is_almost_whole(b) && is_almost_whole(a) {
        tokens = ((a.round() as u64) * 3) + (b.round() as u64);
    }

    return tokens
}


fn is_almost_whole(x: f64) -> bool {
    let fract = x.fract();
    if fract < 0.001 || (1.0 - fract) < 0.001 {
        return true
    }
    return false
}

fn parse_input(lines: Vec<String>, part_2: bool) -> Vec<[f64; 6]> {
    let mut gachas: Vec<[f64; 6]> = vec![];

    let mut gacha: [f64; 6] = [0.0; 6];
    let num_lines = lines.len();
    for (i, line) in lines.into_iter().enumerate() {
        if line.contains("Button A") {
            let re_x = Regex::new(r"X\+[0-9]+").unwrap();
            let mat_x = re_x.find(&line).unwrap();
            gacha[0] = *(&(mat_x.as_str())[2..].parse::<f64>().unwrap());

            let re_y = Regex::new(r"Y\+[0-9]+").unwrap();
            let mat_y = re_y.find(&line).unwrap();
            gacha[1] = *(&(mat_y.as_str())[2..].parse::<f64>().unwrap());
        } else if line.contains("Button B") {
            let re_x = Regex::new(r"X\+[0-9]+").unwrap();
            let mat_x = re_x.find(&line).unwrap();
            gacha[2] = *(&(mat_x.as_str())[2..].parse::<f64>().unwrap());

            let re_y = Regex::new(r"Y\+[0-9]+").unwrap();
            let mat_y = re_y.find(&line).unwrap();
            gacha[3] = *(&(mat_y.as_str())[2..].parse::<f64>().unwrap());
        } else if line.contains("Prize") {
            let re_x = Regex::new(r"X\=[0-9]+").unwrap();
            let mat_x = re_x.find(&line).unwrap();
            gacha[4] = *(&(mat_x.as_str())[2..].parse::<f64>().unwrap());
            if part_2 {
                gacha[4] += 10000000000000.0;
            }

            let re_y = Regex::new(r"Y\=[0-9]+").unwrap();
            let mat_y = re_y.find(&line).unwrap();
            gacha[5] = *(&(mat_y.as_str())[2..].parse::<f64>().unwrap());
            if part_2 {
                gacha[5] += 10000000000000.0;
            }
        } else {
            gachas.push(gacha);
            gacha = [0.0; 6];
        }
        if i == num_lines - 1 {
            gachas.push(gacha);
        }
    }

    return gachas

}