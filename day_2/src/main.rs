use std::fs::read_to_string;
use std::cmp::Ordering;


fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}


fn check_if_safe(mut values: Vec<u32>, dampener: bool, remove_value: usize) -> (u16, usize) {
    let mut result: (u16, usize);
    println!("{:?}", values);
    if remove_value < 100 {
        values.remove(remove_value);
    }

    match values[0].cmp(&values[1]) {
        Ordering::Less => {
            result = check_if_increasing(&values);
            if dampener && result.0 == 0 {
                let new_values = values;
                for i in result.1 - 1..=result.1 + 1{
                    result = check_if_safe(new_values.clone(), false, i);
                    if result.0 == 1 {
                        break
                    }
                }
            }
        },
        Ordering::Greater => {
            result = check_if_decreasing(&values);
            if dampener && result.0 == 0 {
                let new_values = values;
                for i in result.1 - 1..=result.1 + 1{
                    result = check_if_safe(new_values.clone(), false, i);
                    if result.0 == 1 {
                        break
                    }
                }
            }
        },
        Ordering::Equal => {
            result = (0, 0);
            if dampener {
                values.remove(0);
                result = check_if_safe(values, false, 101)
            }
        }
    }
   
    return result
}


fn check_if_decreasing(values: &Vec<u32>) -> (u16, usize) {
    let mut is_safe: bool = true;
    let mut bad_index: usize = 0;
    for (index, value) in values.iter().enumerate() {
        if index == 0 {
            continue;
        }
        if values[index - 1] <= *value || (values[index - 1] - value) > 3 {
            is_safe = false;
            bad_index = index;
            break;
        }
    }

    if is_safe {
        return (1, bad_index)
    } else {
        return (0, bad_index)
    }
}

fn check_if_increasing(values: &Vec<u32>) -> (u16, usize) {
    let mut is_safe: bool = true;
    let mut bad_index: usize = 0;
    for (index, value) in values.iter().enumerate() {
        if index == 0 {
            continue;
        }
        if values[index - 1] >= *value || (value - values[index - 1]) > 3 {
            is_safe = false;
            bad_index = index;
            break;
        }
    }

    if is_safe {
        return (1, bad_index)
    } else {
        return (0, bad_index)
    }
}


fn main() {
    let input_file = "./puzzle_input.txt";
    let  lines = read_lines(&input_file);

    let mut final_result: u16 = 0;
    let mut values: Vec<u32>;

    for line in &lines {
        values = line.split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        final_result += check_if_safe(values,  false, 101).0
    }

    println!("Safe Reports without problem dampener: {final_result}");

    final_result = 0;

    for line in &lines {
        values = line.split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        final_result += check_if_safe(values,  true, 101).0
    }

    println!("Safe Reports with problem dampener: {final_result}");
}
