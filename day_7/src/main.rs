use std::fs::read_to_string;
use std::collections::HashMap;


fn main() {
    let lines: Vec<String> = read_to_string("./puzzle_input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut inputs: HashMap<u64, Vec<u64>> = HashMap::new();

    for line in lines {
        let x: Vec<String> = line.split(": ").map(String::from).collect();
        let key: u64 = x[0].parse().unwrap();
        let value: Vec<u64> = x[1].split(" ").map(|i| -> u64{i.parse().unwrap()}).collect();
        inputs.insert(key, value);
    }

    println!("Sum of valid inputs for part 1: {}", determine_sum_of_valid_results(&inputs, false));
    println!("Sum of valid inputs for part 2: {}", determine_sum_of_valid_results(&inputs, true));

}

fn get_operations(n: u32) -> Vec<String> {
    if n == 1 {
        let operations = ["m", "p"].to_vec();
        return operations.into_iter().map(String::from).collect()
    } else {
        let prev_operations = get_operations(n - 1);
        let mut new_operations: Vec<String> = vec![];
        for x in prev_operations {
            for y in get_operations(1) {
                new_operations.push(x.clone() + &y);
            }
        }
        return new_operations
    }
}

fn get_operations_with_concat(n: u32) -> Vec<String> {
    if n == 1 {
        let operations = ["m", "p", "c"].to_vec();
        return operations.into_iter().map(String::from).collect()
    } else {
        let prev_operations = get_operations_with_concat(n - 1);
        let mut new_operations: Vec<String> = vec![];
        for x in prev_operations {
            for y in get_operations_with_concat(1) {
                new_operations.push(x.clone() + &y);
            }
        }
        return new_operations
    }
}

fn determine_sum_of_valid_results(inputs: &HashMap<u64, Vec<u64>>, concat: bool) -> u64 {
    let mut result: u64 = 0;

    for (key, value) in inputs.into_iter() {

        let mut all_operations: Vec<String> = vec![];
        if concat {
            all_operations = get_operations_with_concat(value.len() as u32 - 1)
        } else {
            all_operations = get_operations(value.len() as u32 - 1);
        }
        'check_valid: for operation in all_operations {
            let operations: Vec<char> = operation.chars().collect();
            let mut sum: u64 = 0;
            let mut concat_checkpoint: String = "".to_string();
            for (i, x) in value.iter().enumerate() {
                if i == 0 {
                    sum = *x;
                } else if i < value.len() {
                    if operations[i - 1] == 'p' {
                        sum = sum + x;
                    } else if operations[i - 1] == 'm' {
                        sum = sum * x;
                    } else if operations[i - 1] == 'c' {
                        let concatenation: String = sum.to_string() + &x.to_string();
                        sum = concatenation.parse().unwrap();
                    }
                }
            }
            if sum == *key {
                result += key;
                break 'check_valid;
            }

        }
    }
    return result
}
    
    