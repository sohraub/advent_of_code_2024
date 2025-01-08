use std::fs::read_to_string;
use std::collections::{HashMap, VecDeque};
use std::process::exit;


fn parse_input(filename: &str) -> Vec<Vec<char>> {
    let board: Vec<Vec<char>> = read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .map(|line| line.chars().collect())
        .collect();

    board
}

fn get_next_tiles(current_tile: (usize, usize), visited: &Vec<(usize, usize)>, board: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut candidates  = [(current_tile.0 + 1, current_tile.1), 
                                                (current_tile.0, current_tile.1 + 1)].to_vec();
    if current_tile.0 > 0 {
        candidates.push((current_tile.0 - 1, current_tile.1));
    }
    if current_tile.1 > 0 {
        candidates.push((current_tile.0, current_tile.1 - 1));
    }
    let mut results: Vec<(usize, usize)> = vec![];
    for coord in candidates {
        if coord.0 >= board.len() {
            continue
        }
        if coord.1 >= board[0].len() {
            continue
        }
        if board[coord.0][coord.1] == '#' {
            continue
        }
        if visited.contains(&coord) {
            continue
        }
        results.push(coord)
    }

    results
}

fn calculate_turning_cost(current: (usize, usize), direction: char, next: (usize, usize)) -> (i32, char) {
    let resulting_direction:char;
    if current.0 > next.0 {
        resulting_direction = '^';
    } else if current.0 < next.0 {
        resulting_direction = 'v';
    } else if current.1 < next.1 {
        resulting_direction = '>';
    } else {
        resulting_direction = '<';
    }

    if resulting_direction == direction {
        return (1, direction)
    } else if ['^', 'v'].contains(&resulting_direction) && ['<', '>'].contains(&direction) {
        return (1001, resulting_direction)
    } else if ['<', '>'].contains(&resulting_direction) && ['v', '^'].contains(&direction) {
        return (1001, resulting_direction)
    } else {
        return (2001, resulting_direction)
    }
}

fn djikstra(board: &Vec<Vec<char>>, start_direction: char) -> (i32, Vec<(usize, usize)>, HashMap<(usize, usize), (i32, char)>) {
    let mut exit_coord: (usize, usize) = (0, 0);
    let mut start_coord: (usize, usize) = (0, 0);
    let mut unvisited = HashMap::new();
    for (y, row) in board.into_iter().enumerate() {
        for (x, elem) in row.into_iter().enumerate() {
            if *elem == 'E' {
                exit_coord = (y, x);
            }
            if *elem == '#' {
                continue
            } else if *elem == 'S' {
                start_coord = (y, x);
                unvisited.insert((y, x), 0);
            } else {
                unvisited.insert((y, x), i32::MAX);
            }
        }
    }

    let mut visited: HashMap<(usize, usize), (i32, char)> = HashMap::new();
    let mut previous: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    visited.insert(start_coord, (0, start_direction));

    while unvisited.keys().len() > 0{
        let mut lowest = i32::MAX - 1;
        let mut current_node = (0, 0);
        for (key, value) in &unvisited {
            if *value < lowest {
                lowest = *value;
                current_node = *key;
            }
        }

        if current_node == (0, 0) {
            break
        }

        let next_tiles = get_next_tiles(current_node, &vec![],board);
        for next_node in next_tiles {
            let (current_cost, current_direction) = *(visited.get(&current_node).unwrap());
            let (additional_cost, new_direction) = calculate_turning_cost(current_node, current_direction, next_node);
            let new_cost = current_cost + additional_cost;

            if visited.contains_key(&next_node) {
                let (existing_value, _) = visited.get(&next_node).unwrap();
                if new_cost == *existing_value || (new_cost - existing_value).abs() == 1000 {
                    previous.get_mut(&next_node).unwrap().push(current_node);
                }
                if new_cost < *existing_value {
                    if let Some(x) = visited.get_mut(&next_node) {
                        *x = (new_cost, new_direction);
                    }
                    if let Some(x) = previous.get_mut(&next_node) {
                        *x = vec![current_node];
                    }
                }
            } else {
                visited.insert(next_node, (new_cost, new_direction));
                previous.insert(next_node, vec![current_node]);
            }
            if let Some(x) = unvisited.get_mut(&next_node) {
                *x = new_cost;
            }
        }
        unvisited.remove(&current_node);
    }

    let tiles_in_path = count_tiles_in_best_path(&previous, &exit_coord, &start_coord);
    if tiles_in_path.len() == 0 {
        return (-1, tiles_in_path, visited);
    }
    return (visited.get(&exit_coord).unwrap().0, tiles_in_path, visited);
}


fn count_tiles_in_best_path(previous: &HashMap<(usize, usize), Vec<(usize, usize)>>,
                            exit_coord: &(usize, usize),
                            start_coord: &(usize, usize),) -> Vec<(usize, usize)> {

    let mut history: Vec<(usize, usize)> = vec![*exit_coord];
    let mut extras: VecDeque<(usize, usize)> = VecDeque::from([]);

    if !previous.contains_key(exit_coord) {
        return vec![];
    }

    let mut pointed = previous.get(exit_coord).unwrap();


    while pointed[0] != *start_coord {
        history.push(pointed[0]);
        pointed = previous.get(&pointed[0]).unwrap();
        if pointed.len() > 1 {
            for tile in pointed {
                extras.push_front(*tile);
            }
            
        }
    }

    history
}

fn main() {
    let mut board = parse_input("puzzle_input.txt");

    let (lowest_score, best_path, visited) = djikstra(&board, '>');

    println!("Lowest score: {lowest_score}");

    let mut start_coord: (usize, usize) = (0, 0);
    let mut exit_coord: (usize, usize) = (0, 0);

    for (y, row) in (&board).into_iter().enumerate() {
        for (x, elem) in row.into_iter().enumerate() {
            if *elem == 'E' {
                exit_coord = (y, x);
            } else if *elem == 'S' {
                start_coord = (y, x);
            }
        }
    }

    board[exit_coord.0][exit_coord.1] = 'S';
    board[start_coord.0][start_coord.1] = 'E';
    let mut reverse_start_direction = visited.get(&exit_coord).unwrap().1;

    if reverse_start_direction == '^' {
        reverse_start_direction = 'v';
    } else if reverse_start_direction == '>' {
        reverse_start_direction = '<';
    }

    let (_, _, visited_reverse) = djikstra(&board, reverse_start_direction);

    let mut full_best_path = best_path.clone();

    for tile in visited_reverse.keys() {
        let (reverse_score, reverse_direction) = visited_reverse.get(tile).unwrap();
        let (score, direction) = visited.get(tile).unwrap();
        let new_score = reverse_score + score;
        if are_opposite(*reverse_direction, *direction) {
            if new_score == lowest_score && !full_best_path.contains(tile) {
                full_best_path.push(*tile);
            }
        } else {
            if lowest_score - (score + reverse_score) == 1000 && !full_best_path.contains(tile) {
                full_best_path.push(*tile);
            }
        }
    }
    println!("Number of tiles in best possible paths: {}", full_best_path.len());


}

fn are_opposite(a: char, b: char) -> bool {
    if (a == 'v' && b == '^') || (a == '<' && b == '>') || (a == '^' && b == 'v') || (a == '>' && b == '<') {
        return true
    }
    return false
}