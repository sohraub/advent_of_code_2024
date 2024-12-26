use std::fs::read_to_string;
use std::collections::HashMap;

fn main() {
    let lines: Vec<String> = read_to_string("./puzzle_input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut satellites: HashMap<char, Vec<(isize, isize)>> = HashMap::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, symbol) in line.chars().enumerate() {
            if symbol != '.' {
                if satellites.contains_key(&symbol) {
                    satellites.get_mut(&symbol).unwrap().push((x as isize, y as isize));
                } else {
                    satellites.insert(symbol, vec![(x as isize, y as isize)]);
                }
            }
        }
    }

    let city_length: isize = lines.len() as isize;
    let city_width: isize = lines[0].len() as isize;

    let unique_antinode_count: usize = count_antinodes(&satellites, city_length, city_width);

    println!("Number of unique antinode locations (Part 1): {unique_antinode_count}");

    let unique_antinode_count_part2: usize = count_antinodes_part2(&satellites, city_length, city_width);

    println!("Number of unique antinode locations (Part 1): {unique_antinode_count_part2}");
}


fn count_antinodes_part2(satellites: &HashMap<char, Vec<(isize, isize)>>, length: isize, width: isize) -> usize {
    
    let mut antinodes: Vec<(isize, isize)> = vec![];

    for (_satellite, locations) in satellites.into_iter() {
        for p1 in locations {
            for p2 in locations {
                if p2 == p1 {
                    continue
                }
                let dx = (p1.0 - p2.0) as f32;
                let dy = (p1.1 - p2.1) as f32;
                let x1 = p1.0 as f32;
                let y1 = p1.1 as f32;
                let slope = dy / dx;
                for x in 0..width {
                    let x = x as f32;
                    let y = slope * (x - x1) + y1;
                    if y == (y as i32) as f32 && (y as isize) >= 0 && (y as isize) < length {
                        let new_antinode = (x as isize, y as isize);
                        if !antinodes.contains(&new_antinode) {
                            antinodes.push(new_antinode);
                        }
                    }
                }
            }
        }
    }

    return antinodes.len()
}


fn count_antinodes(satellites: &HashMap<char, Vec<(isize, isize)>>, length: isize, width: isize) -> usize {
    let mut antinodes: Vec<(isize, isize)> = vec![];

    for (_satellite, locations) in satellites.into_iter() {
        for primary in locations {
            for location in locations {
                if location == primary {
                    continue
                }
                let dx = primary.0 - location.0;
                let dy = primary.1 - location.1;
                let a1 = (primary.0 + dx, primary.1 + dy);
                let a2 = (location.0 - dx, location.1 - dy);
                if a1.0 < width && a1.0 >= 0 && a1.1 < length && a1.1 >= 0 && !antinodes.contains(&a1) {
                    antinodes.push(a1);
                }
                if a2.0 < width && a2.0 >= 0 && a2.1 < length && a2.1 >= 0 && !antinodes.contains(&a2) {
                    antinodes.push(a2);
                }
            }
        }
    }

    return antinodes.len()
}
