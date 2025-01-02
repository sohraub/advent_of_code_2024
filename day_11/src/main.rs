use std::fs::read_to_string;
use std::collections::HashMap;

fn main() {
    let input: Vec<u64> = read_to_string("./puzzle_input.txt")
        .unwrap()
        .split(" ")
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    println!("Input: {:?}", input);

    let mut stones: HashMap<u64, u64> = HashMap::new();
    for stone in input {
        *stones.entry(stone).or_default() += 1;
    }

    for _ in 0..75 {
        stones = blink(stones)
    }

    let total: u64 = stones.values().sum();

    println!("{total}")

}

fn blink(stones: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut new_stones = HashMap::with_capacity(stones.len());
    for (key, value) in stones {
        match key {
            0 => *new_stones.entry(1).or_default() += value,
            _ => {
                let digits = key.checked_ilog10().unwrap_or(0) + 1;
                if digits % 2 == 1 {
                    *new_stones.entry(2024 * key).or_default() += value;
                } else {
                    let exp: u32 = digits / 2;
                    let value_a = key / 10_u64.pow(exp);
                    let value_b = key - (value_a * 10_u64.pow(exp));
                    *new_stones.entry(value_a).or_default() += value;
                    *new_stones.entry(value_b).or_default() += value;
                }
            }
        }
    }
    new_stones
}