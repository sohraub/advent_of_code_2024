use std::fs::read_to_string;
use std::collections::HashMap;
use itertools::Itertools;
use array2d::Array2D;

fn main() {
    let input: Vec<Vec<char>> = read_to_string("./puzzle_input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .map(|x| x.chars().collect())
        .collect();

    let board: Array2D<char> = Array2D::from_rows(&input).unwrap();

    let mut region_directory: HashMap<(usize, usize), String> = HashMap::new();
    let mut regions: HashMap<String, Vec<(usize, usize)>> = HashMap::new();


    for (y, row) in board.rows_iter().enumerate() {
        'search: for (x, elem) in row.enumerate() {
            for (dy, dx) in [(1, 0), (-1, 0), (0, 1), (0, -1)].map(|x| (x.0 as isize, x.1 as isize)) {
                let y1 = (y as isize) + dy;
                let x1 = (x as isize) + dx;
                if y1 < 0  || y1 >= board.column_len() as isize || x1 < 0 || x1 >= board.row_len() as isize {
                    continue
                }
                let y1 = y1 as usize;
                let x1 = x1 as usize;


                if board[(y1, x1)] == *elem && region_directory.contains_key(&(y1, x1)) {
                    let region: String = region_directory.get(&(y1, x1)).unwrap().to_string();
                    regions.get_mut(&region).unwrap().push((y, x));
                    region_directory.insert((y, x), region);
                    continue 'search;
                }
            }
            // Reaching here implies that (x, y) does not belong to an existing region of one of it's neighbours, so 
            // create a new region
            let mut region_name = format!("{elem}_0");
            if regions.contains_key(&region_name) {
                let mut i: i32 = 1;
                let mut new_region_name = format!("{elem}_{i}");
                while regions.contains_key(&new_region_name) {
                    i += 1;
                    new_region_name = format!("{elem}_{i}");
                }
                region_name = new_region_name;
            } 
            region_directory.insert((y, x), region_name.clone());
            regions.insert(region_name, vec![(y, x)]);
        }
    }

    for (y, row) in board.rows_iter().enumerate() {
        for (x, elem) in row.enumerate() {
            for (dy, dx) in [(1, 0), (-1, 0), (0, 1), (0, -1)].map(|x| (x.0 as isize, x.1 as isize)) {
                let y1 = (y as isize) + dy;
                let x1 = (x as isize) + dx;
                if y1 < 0  || y1 >= board.column_len() as isize || x1 < 0 || x1 >= board.row_len() as isize {
                    continue
                }
                let y1 = y1 as usize;
                let x1 = x1 as usize;

                if board[(y, x)] == board[(y1, x1)] && region_directory.get(&(y, x)).unwrap() != region_directory.get(&(y1, x1)).unwrap() {
                    let directory_clone = region_directory.clone();
                    let region_a = directory_clone.get(&(y, x)).unwrap();
                    let region_b = directory_clone.get(&(y1, x1)).unwrap();

                    let region_a_order: u64 = region_a.split("_").map(String::from).collect::<Vec<String>>()[1].parse().unwrap();
                    let region_b_order: u64 = region_b.split("_").map(String::from).collect::<Vec<String>>()[1].parse().unwrap();

                    if region_a_order > region_b_order {
                        for coord in regions.clone().get(region_a).unwrap() {
                            regions.get_mut(region_b).unwrap().push(*coord);
                            region_directory.insert(*coord, region_b.clone());
                        }
                        regions.remove(region_a);
                    } else {
                            for coord in regions.clone().get(region_b).unwrap() {
                                regions.get_mut(region_a).unwrap().push(*coord);
                                region_directory.insert(*coord, region_a.clone());
                            }
                            regions.remove(region_b);
                    }
                }
            
            }
        }
    }

    let mut total_price: u64 = 0;
    for (key, value) in regions.clone() {
        let mut perimeter: u64 = 0;
        let area: u64 = value.len() as u64;
        for (y, x) in value.into_iter() {
            if y == 0 || y == board.column_len() - 1 {
                perimeter += 1;
            }
            if x == 0 || x == board.row_len() - 1 {
                perimeter += 1;
            }

            for (dy, dx) in [(1, 0), (-1, 0), (0, 1), (0, -1)].map(|i| (i.0 as isize, i.1 as isize)) {
                let y1 = y as isize + dy;
                let x1 = x as isize + dx;
                if y1 < 0  || y1 >= board.column_len() as isize || x1 < 0 || x1 >= board.row_len() as isize {
                    continue
                }
                let y1 = (y as isize + dy) as usize;
                let x1 = (x as isize + dx) as usize;
                if *(region_directory.get(&(y1, x1)).unwrap()) != key {
                    perimeter += 1
                }
            }
        }
        total_price += area * perimeter;
    }

    println!("Part 1 solution: {total_price}");

    let mut total_price: u64 = 0;
    for (key, value) in regions.clone() {
        let mut corners: u64 = 0;
        let area: u64 = value.len() as u64;
        for (y, x) in value.into_iter() {
            let region = region_directory.get(&(y, x)).unwrap().clone();
            let y = y as isize;
            let x = x as isize;
            let up: (isize, isize) = (y - 1, x);
            let down: (isize, isize) = (y + 1, x);
            let left: (isize, isize) = (y, x - 1);
            let right: (isize, isize) = (y, x + 1);

            for (vert, hori) in [up, down].iter().cartesian_product([left, right].iter()) {
                if !is_in_region(*vert, &region, &region_directory, &board) && !is_in_region(*hori, &region, &region_directory, &board) {
                    corners += 1;
                }
                if !is_in_region((vert.0, hori.1), &region, &region_directory, &board) {
                    if is_in_region(*vert, &region, &region_directory, &board) && is_in_region(*hori, &region, &region_directory, &board) {
                        corners += 1;
                    }
                }
            }
        }
        total_price += area * corners;
    }

    println!("Part 2 solution: {total_price}");

}

fn is_in_region(coord: (isize, isize), region: &String, region_directory: &HashMap<(usize, usize), String>, board: &Array2D<char>) -> bool {
    if coord.0 < 0  || coord.0 as usize >= board.column_len() || coord.1 < 0 || coord.1 as usize >= board.row_len() {
        return false
    }

    if region_directory.get(&(coord.0 as usize, coord.1 as usize)).unwrap() != region {
        return false
    }
    return true
}