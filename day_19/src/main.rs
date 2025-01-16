use std::fs::read_to_string;
use std::collections::HashMap;

fn read_input(filename: &str) -> (Vec<String>, Vec<String>) {
    let input: Vec<String> = read_to_string(filename)
        .unwrap()
        .split("\r\n\r\n")
        .map(String::from)
        .collect();

    let patterns: Vec<String> = input[0]
        .split(", ")
        .map(String::from)
        .collect();

    let towels: Vec<String> = input[1]
        .split("\r\n")
        .map(String::from)
        .collect();

    (patterns, towels)
}

fn check_towel(pattern_map: &HashMap<String, bool>, towel: &String) -> bool {
    if pattern_map.contains_key(towel) {
        return true
    }
    if *towel == "" || towel.len() == 1 {
        return false
    }
    for i in (0..=towel.len()-1).rev() {
        let substring_a = towel[0..i].to_string();
        if pattern_map.contains_key(&substring_a) {
            let substring_b = towel[i..].to_string();
            let result = check_towel(pattern_map, &substring_b);
            if result {
                return true
            }
        }
    }
    return false
}

fn get_all_combos(pattern_map: &HashMap<String, bool>, towel: &String, mut count_map: HashMap<String, u64>)  -> HashMap<String, u64> {
    if count_map.keys().len() == 0 {
        for (key, _) in pattern_map {
            if key.len() == 1 {
                count_map.insert(key.clone(), 1);
            }
        }
    }
    for i in 1..=towel.len() {
        let mut count: u64 = 0;
        let substring = towel[towel.len()-i..].to_string();
        if count_map.contains_key(&substring) {
            continue
        }
        if pattern_map.contains_key(&substring) {
            count += 1;
        }
        if substring.len() == 1 {
            // Reaching here implies a single character substring that isn't a matched pattern
            count_map.insert(substring, 0);
            continue
        }
        for j in 1..substring.len() {
            let substring_start = substring[0..j].to_string();
            if pattern_map.contains_key(&substring_start) {
                let substring_end = substring[j..].to_string();
                if (&count_map).contains_key(&substring_end) {
                    count += (&count_map).get(&substring_end).unwrap();
                } 
            }
        }
        count_map.insert(substring, count);
    }
    count_map
}

fn part_two(patterns: &Vec<String>, towels: &Vec<String>) -> u64 {
    let mut count: u64 = 0;
    let pattern_map: HashMap<String, bool> = patterns
        .into_iter()
        .map(|x| (x.clone(), true))
        .collect::<HashMap<String, bool>>();

    let mut count_map = HashMap::new();
    for (key, _) in &pattern_map {
        if key.len() == 1 {
            count_map.insert(key.clone(), 1);
        }
    }
    for towel in towels {
        if !check_towel(&pattern_map, towel) {
            continue
        }
        let count_map = get_all_combos(&pattern_map, towel, count_map.clone());
        count += count_map.get(towel).unwrap()
    }
    return count
}

fn part_one(patterns: &Vec<String>, towels: &Vec<String>) -> u32 {
    let mut count =  0;
    let pattern_map: HashMap<String, bool> = patterns
        .into_iter()
        .map(|x| (x.clone(), true))
        .collect::<HashMap<String, bool>>();

    for towel in towels {
        if check_towel(&pattern_map, &towel) {
            count += 1
        }
    }
    count
}

fn main() {
    let (patterns, towels) = read_input("./puzzle_input.txt");
    let part_one_answer = part_one(&patterns, &towels);
    println!("Answer for part one: {part_one_answer}");
    let part_two_answer = part_two(&patterns, &towels);
    println!("Answer for part two: {part_two_answer}");
}

#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn recursion_base_case() {
        let pattern_map = HashMap::from([
            ("brwrr".to_string(), true)
        ]);
        assert_eq!(check_towel(&pattern_map, &"brwrr".to_string()), true);
    }

    #[test]
    fn recursion_2nd_case() {
        let pattern_map = HashMap::from([
            ("br".to_string(), true),
            ("w".to_string(), true),
            ("r".to_string(), true)
        ]);
        assert_eq!(check_towel(&pattern_map, &"brwrr".to_string()), true);
    }

    #[test]
    fn recursion_fail() {
        let pattern_map = HashMap::from([
            ("br".to_string(), true),
            ("wr".to_string(), true)
        ]);
        assert_eq!(check_towel(&pattern_map, &"brwrr".to_string()), false);
    }

    #[test]
    fn full_test_input_part_1() {
        let (patterns, towels) = read_input("test_puzzle_input.txt");
        let count = part_one(&patterns, &towels);
        assert_eq!(count, 6);
    }

    #[test]
    fn every_combo_test() {
        let pattern_map = HashMap::from([
            ("br".to_string(), true),
            ("gb".to_string(), true),
            ("rb".to_string(), true),
            ("g".to_string(), true),
            ("b".to_string(), true),
            ("r".to_string(), true)
        ]);
        let combos = get_all_combos(&pattern_map, &"rrbgbr".to_string(), HashMap::new());
        println!("{:?}", combos);
        assert_eq!(*combos.get(&"rrbgbr".to_string()).unwrap(), 6);
    }

    #[test]
    fn full_test_input_part_2() {
        let (patterns, towels) = read_input("test_puzzle_input.txt");
        let count = part_two(&patterns, &towels);
        assert_eq!(count, 16);
    }
}