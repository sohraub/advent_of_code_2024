use std::fs::read_to_string;
use std::collections::HashMap;

fn parse_input(filename: &str) -> (u64, u64, u64, Vec<u64>) {
    let lines: Vec<String> = read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    let a: u64 = lines[0].split(' ').collect::<Vec<&str>>()[2].parse::<u64>().unwrap();
    let b: u64 = lines[1].split(' ').collect::<Vec<&str>>()[2].parse::<u64>().unwrap();
    let c: u64 = lines[2].split(' ').collect::<Vec<&str>>()[2].parse::<u64>().unwrap();
    let instructions: Vec<u64> = lines[4].split(' ')
        .collect::<Vec<&str>>()[1]
        .split(',')
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    (a, b, c, instructions)
}

fn get_combo_operand(operand: u64, register: &HashMap<char, u64>) -> u64 {
    if operand <= 3 {
        return operand
    }
    if operand == 4 {
        return *register.get(&'a').unwrap()
    }
    if operand == 5 {
        return *register.get(&'b').unwrap()
    }
    *register.get(&'c').unwrap()
}

fn adv(operand: u64, register: &HashMap<char, u64> ) -> u64 {
    let operand = get_combo_operand(operand, register) as f32;
    let a = *register.get(&'a').unwrap() as f32;
    let result = a / 2_f32.powf(operand);
    return result as u64
}

fn bxl(operand: u64, register: &HashMap<char, u64>) -> u64 {
    let result: u64 = *register.get(&'b').unwrap() ^ operand;
    result
}

fn bst(operand: u64, register: &HashMap<char, u64>) -> u64 {
    get_combo_operand(operand, register).rem_euclid(8)
}

fn jnz(operand: u64, register: &HashMap<char, u64>) -> i32 {
    if *register.get(&'a').unwrap() == 0 {
        return -1
    }
    operand as i32
}

fn bxc(register: &HashMap<char, u64>) -> u64 {
    *register.get(&'b').unwrap() ^ *register.get(&'c').unwrap()
}

fn run_program(a: u64, b: u64, c: u64, instructions: &Vec<u64>) -> (Vec<u64>, HashMap<char, u64>) {
    let mut register: HashMap<char, u64> = HashMap::new();
    register.insert('a', a);
    register.insert('b', b);
    register.insert('c', c);

    let mut outputs: Vec<u64> = vec![];

    let mut i: usize = 0;
    let mut iteration: usize;

    while i < instructions.len() - 1 {
        iteration = 2;
        let instruction = instructions[i];
        let operand = instructions[i + 1];
        match instruction {
            0 => {
                let new_a = adv(operand, &register);
                if let Some(x) = register.get_mut(&'a') {
                    *x = new_a;
                }
            },
            1 => {
                let new_b = bxl(operand, &register);
                if let Some(x) = register.get_mut(&'b') {
                    *x = new_b;
                }
            },
            2 => {
                let new_b = bst(operand, &register);
                if let Some(x) = register.get_mut(&'b') {
                    *x = new_b;
                }
            },
            3 => {
                let jump = jnz(operand, &register);
                if jump >= 0 {
                    i = jump as usize;
                    iteration = 0;
                }
            },
            4 => {
                let new_b = bxc(&register);
                if let Some(x) = register.get_mut(&'b') {
                    *x = new_b;
                }
            },
            5 => {
                let result = bst(operand, &register);
                outputs.push(result);
            },
            6 => {
                let new_b = adv(operand, &register);
                if let Some(x) = register.get_mut(&'b') {
                    *x = new_b;
                }
            },
            7 => {
                let new_c = adv(operand, &register);
                if let Some(x) = register.get_mut(&'c') {
                    *x = new_c;
                }
            }
            _ => println!("Invalid instruction given: {instruction}")
        }
        i += iteration;
    }
    (outputs, register)
}

fn main() {
    let (a, b, c, instructions) = parse_input("./puzzle_input.txt");
    let (output, _) = run_program(a, b, c, &instructions);

    println!("Part 1 solution: {}", output.into_iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","));

    let mut candidates: Vec<u64> = (0..8).collect();
    let mut new_candidates: Vec<u64> = vec![];
    for target in (&instructions).into_iter().rev() {
        new_candidates = vec![];
        for x in &candidates {
            let exp: u32  = ((x % 8) ^ 5) as u32;
            let denom = 2_u64.pow(exp);
            let result = (x % 8) ^ 3 ^ (x / denom);
            if result % 8 == *target {
                new_candidates.push(*x);
            }
        }
        if new_candidates.len() == 0 {
            println!("Nothing found");
            break
        }
        candidates = vec![];
        for new in &new_candidates {
            candidates.extend((8*new..(8*(new + 1))).collect::<Vec<u64>>())
        }
    }
    println!("Part 2 answer {:?}", new_candidates[0]);

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_test() {
        let (a, b, c, instructions) = parse_input("./test_puzzle_input.txt");
        let (output, _) = run_program(a, b, c, &instructions);
        assert_eq!(output, vec![4,6,3,5,6,3,5,2,1,0]);
    }

    #[test]
    fn validate_part_2() {
        let (output, _) = run_program(117440, 0, 0, &vec![0,3,5,4,3,0]);
        assert_eq!(output, vec![0,3,5,4,3,0]);
    }

    #[test]
    fn read_test_input() {
        let (a, b, c, instructions) = parse_input("./test_puzzle_input.txt");
        assert_eq!(a, 729);
        assert_eq!(b, 0);
        assert_eq!(c, 0);
        assert_eq!(instructions, [0, 1, 5, 4, 3, 0].to_vec());
    }

    #[test]
    fn case_1() {
        let (_, register) = run_program(0, 0, 9, &[2,6].to_vec());
        assert_eq!(*register.get(&'b').unwrap(), 1);
    }

    #[test]
    fn case_2() {
        let (output, _) = run_program(10, 0, 0, &vec![5,0,5,1,5,4]);
        assert_eq!(output, vec![0,1,2]);
    }

    #[test]
    fn case_3() {
        let (output, register) = run_program(2024, 0, 0, &vec![0,1,5,4,3,0]);
        assert_eq!(output, vec![4,2,5,6,7,7,7,7,3,1,0]);
        assert_eq!(*register.get(&'a').unwrap(), 0);
    }

    #[test]
    fn case_4() {
        let (_, register) = run_program(0, 29, 0, &vec![1,7]);
        assert_eq!(*register.get(&'b').unwrap(), 26);
    }

    #[test]
    fn case_5() {
        let (_, register) = run_program(0, 2024, 43690, &vec![4,0]);
        assert_eq!(*register.get(&'b').unwrap(), 44354)
    }
}