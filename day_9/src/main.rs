use std::fs::read_to_string;

fn main() {
    let line: String = read_to_string("./puzzle_input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();


    let input: Vec<u32> = line.chars().map(|i| -> u32{i.to_digit(10).unwrap()}).collect();

    let mut disk_map: Vec<i32> = vec![];
    let mut current_id: i32 = 0;
    let mut empty_space_count: usize = 0;

    for (i, digit) in input.into_iter().enumerate() {
        for _j in 0..digit {
            if i % 2 == 0 {
                disk_map.push(current_id);
            } else {
                disk_map.push(-1);
                empty_space_count += 1;
            }
        }
        if i % 2 == 0 {
            current_id += 1
        }
    }

    let part_1_checksum = calculate_checksum_part1(disk_map.clone(), empty_space_count);
    println!("Part one checksum: {part_1_checksum}");

    let part_2_checksum = calculate_checksum_part2(disk_map.clone());
    println!("Part two checksum: {part_2_checksum}")

}


fn calculate_checksum_part2(mut disk_map: Vec<i32>) -> i64 {
    let mut empty_spaces: Vec<(i64, usize)> = vec![];
    let mut empty_sum: i64 = 0;
    let mut start_index: usize = 0;

    for (index, value) in disk_map.clone().into_iter().enumerate() {
        if value == -1 && empty_sum >=0 {
            if empty_sum == 0 {
                start_index = index;
            }
            empty_sum += 1;
        }

        if value != -1 {
            if empty_sum != 0 {
                empty_spaces.push((empty_sum, start_index));
            }
            empty_sum = 0
        }
    }

    let mut current_data: i32 = -1;
    let mut source_index_end: usize = 0;
    let mut source_index_start: usize = 0;
    //let mut dest_index_start: usize = 0;
    let mut data_found: bool = false;
    let mut completed_values: Vec<i32> = vec![];
    let mut all_space_sizes: Vec<i64> = vec![];

    while current_data != 0 {
        
        all_space_sizes = vec![];
        for value in empty_spaces.clone().iter() {
            all_space_sizes.push(value.0);
        }
        let max_size: i64 = *all_space_sizes.iter().max().unwrap();
        
        let mut range_end = source_index_start;
        if source_index_start == 0 {
            range_end = disk_map.len();
        }        
        for index in (0..range_end).rev() {
            let value = disk_map[index];
            if value != -1 && !data_found && !completed_values.contains(&value) {
                current_data = value;
                source_index_end = index;
                data_found = true;
            }
            if value != current_data && data_found {
                source_index_start = index + 1;
                data_found = false;
                break
            }
        }

        if current_data == 0 {
            break
        }

        let data_size: usize = source_index_end - source_index_start + 1;
       // for (index, value) in disk_map.clone().into_iter().enumerate() {
       //     if index >= source_index_start {
       //         break
       //     }
       //     if value == -1 && index + data_size < disk_map.len() {
       //         let slice_sum: i32 = disk_map[index..(index + data_size)].iter().sum();
       //         if slice_sum == -1 * (data_size as i32) {
       //             dest_index_start = index;
       //             break
       //         }
       //     }
       // }
       // if dest_index_start > 0 {
       //     for i in 0..data_size {
       //         disk_map[dest_index_start + i] = current_data;
       //         disk_map[source_index_start + i] = -1;
       //     }
       // }
       // dest_index_start = 0;

        let data_size_i: i64 = data_size as i64;
        if data_size_i <= max_size {

            for (index, empty_space) in empty_spaces.clone().iter().enumerate() {
                if empty_space.1 >= source_index_start {
                    break
                }
                if empty_space.0 >= data_size_i  {
                    for i in 0..data_size {
                        disk_map[empty_space.1 + i] = current_data;
                        disk_map[source_index_start + i] = -1;
                    }
                    if empty_space.0 > data_size_i {
                        empty_spaces[index] = (empty_space.0 - data_size_i, empty_space.1 + data_size);
                    } else {
                        empty_spaces.remove(index);
                    }
                }
                    break
            }

        }
        
        completed_values.push(current_data);
    }

    let mut checksum: i64 = 0;

    for (index, value) in disk_map.into_iter().enumerate() {
        if value != -1 {
            checksum += (index as i64) * (value as i64);
        }
    }

    return checksum
}


fn calculate_checksum_part1(mut disk_map: Vec<i32>, empty_space_count: usize) -> i64 {

    let mut tail_sum: i32 = disk_map[disk_map.len()-empty_space_count..].iter().sum();

    while tail_sum != -1 * (empty_space_count as i32) {
        let mut source_index: usize = 0;
        let mut dest_index: usize = 0;
        let mut source_value: i32 = 0;
        for (index, value) in disk_map.clone().into_iter().enumerate().rev() {
            if value != -1 {
                source_index = index;
                source_value = value;
                break
            }
        }
        for (index, value) in disk_map.clone().into_iter().enumerate() {
            if value == -1 {
                dest_index = index;
                break
            }
        }

        disk_map[dest_index] = source_value;
        disk_map[source_index] = -1;

        tail_sum = disk_map[disk_map.len()-empty_space_count..].iter().sum();

    }

    let mut checksum: i64 = 0;
    for (index, value) in disk_map.into_iter().enumerate() {
        if value == -1 {
            break
        }
        checksum = checksum + (index as i64) * (value as i64);
    }

    return checksum;
}


