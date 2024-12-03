use regex::Regex;
use std::fs::read_to_string;


fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn parse_mul_string_into_product(mul_string: &str) -> u32 {
    let re = Regex::new(r"[0-9]+").unwrap();
    let numbers: Vec<&str> = re.find_iter(&mul_string).map(|m| m.as_str()).collect();
    let a: u32 = numbers[0].parse().unwrap();
    let b: u32 = numbers[1].parse().unwrap();

    return a * b
}

fn parse_lines(input_line: &str) -> u32{
    let re = Regex::new(r"mul\([0-9]+\,[0-9]+\)").unwrap();
    let caps: Vec<&str> = re.find_iter(&input_line).map(|m| m.as_str()).collect();

    let mut result: u32 = 0;

    for cap in caps.iter() {
        result += parse_mul_string_into_product(cap);
    }

    return result
}

fn parse_with_conditions(line: &str) {
    let re = Regex::new(r"(mul\([0-9]+\,[0-9]+\))|(don\'t\(\))|(do\(\))").unwrap();
    let caps: Vec<&str> = re.find_iter(&line).map(|m| m.as_str()).collect();

    let mut result: u32 = 0;
    let mut compute: bool = true;

    for cap in caps.iter() {
        if cap.contains("mul") {
            if compute {
                println!("computing due to {cap}");
                result += parse_mul_string_into_product(cap);
            } else {
                continue;
            }
        } else if cap.contains("don") {
            println!("deactivating due to {cap}");
            compute = false;
        } else if cap.contains("do") {
            println!("activating due to {cap}");
            compute = true;
        }

    }

    println!("Result with conditions: {result}")

}

fn main() {
    let input_file = "./puzzle_input.txt";
    let lines = read_lines(&input_file);
    let mut final_result: u32 = 0;
    for line in &lines {
        final_result += parse_lines(&line)
    }
    println!("Final result for part 1: {}", final_result);

    let mut single_line: String = "".to_owned();
    for line in &lines {
        single_line.push_str(&line)
    }

    parse_with_conditions(&single_line)

}
