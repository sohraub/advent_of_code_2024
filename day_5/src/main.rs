use std::fs::read_to_string;
use std::str::FromStr;
use std::collections::HashMap;


fn main() {
    let lines: Vec<String> = read_to_string("./puzzle_input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();

    let mut updates: Vec<String> = vec![];

    for line in lines {
        if line.contains(',') {
            updates.push(line);
        } else if line.contains('|') {
            let rule: Vec<String> = line.split('|').map(String::from).collect();
            let key: u32 = rule[0].parse().unwrap();
            let value: u32 = rule[1].parse().unwrap();
                if rules.contains_key(&key) {
                    rules.get_mut(&key).unwrap().push(value)
                } else {
                    rules.insert(key, vec![value]);

                }
        }
    }

    let mut middle_number_sum: u32 = 0;
    let mut incorrect_middle_sum: u32 = 0;

    for update_string in updates {

        let update: Vec<u32> = update_string.split(',')
            .filter_map(|x| u32::from_str(x).ok())
            .collect();

        if update_is_correct(&rules, &update) {
            let mid_index: usize = update.len() / 2;
            middle_number_sum += update[mid_index]
        } else {
            incorrect_middle_sum += handle_incorrect_update(&rules, &update)
        }
    }

    println!("Part 1 answer is {}", middle_number_sum);
    println!("Part 2 answer is {}", incorrect_middle_sum);

}


fn update_is_correct(rules: &HashMap<u32, Vec<u32>>, update: &Vec<u32>) -> bool {
    let mut is_correct: bool = true;

    'outer: for (index, x) in update.iter().enumerate() {
        if rules.contains_key(x) {
            if index < update.len() - 1 {
                for y in &update[index+1..] {
                    if rules.get(x).unwrap().contains(y) {
                        continue
                    } else { 
                        is_correct = false;
                        break 'outer
                    }
                }
            }
            if index > 0 {
                for y in &update[0..index] {
                    if rules.get(x).unwrap().contains(y) {
                        is_correct = false;
                        break 'outer
                    }
                }
            }
        }
    }

    return is_correct
}

fn handle_incorrect_update(rules: &HashMap<u32, Vec<u32>>, update: &Vec<u32>) -> u32 {

    let mut middle_value: u32 = 0;

    if update_is_correct(rules, update) {
        let mid_index: usize = update.len() / 2;
        middle_value = update[mid_index];
    }
    else {
        'outer: for (index, x) in update.iter().enumerate() {
            if rules.contains_key(x) {
                if index < update.len() - 1 {
                    for y in &update[index+1..] {
                        if rules.get(x).unwrap().contains(y) {
                            continue
                        } else { 
                            if rules.contains_key(y) && rules.get(y).unwrap().contains(x) {
                                let mut new_update: Vec<u32> = vec![];
                                for z in update.iter() {
                                    if z == x {
                                        new_update.push(*y)
                                    } else if z == y {
                                        new_update.push(*x)
                                    } else {
                                        new_update.push(*z)
                                    }
                                }
                                middle_value = handle_incorrect_update(rules, &new_update);
                            }
                            break 'outer
                        }
                    }
                }
                if index > 0 {
                    for y in &update[0..index] {
                        if rules.get(x).unwrap().contains(y) {
                            let mut new_update: Vec<u32> = vec![];
                            for z in update.iter() {
                                if z == x {
                                    new_update.push(*y)
                                } else if z == y {
                                    new_update.push(*x)
                                } else {
                                    new_update.push(*z)
                                }
                            }
                            middle_value = handle_incorrect_update(rules, &new_update);
                            break 'outer
                        }
                    }
                }
            }
        }
    }

    return middle_value
}