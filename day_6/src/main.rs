use std::fs::read_to_string;
use array2d::Array2D;
use std::collections::HashMap;

const DIRECTIONS: [&str; 4] = ["up", "right", "down", "left"];


fn main() {
    let lines: Vec<String> = read_to_string("./puzzle_input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut board_vec: Vec<Vec<char>> = vec![];

    for line in lines {
        board_vec.push(line.chars().collect());
    }

    let board: Array2D<char> = Array2D::from_rows(&board_vec).unwrap();

    let mut guard_pos: (usize, usize) = (0, 0);
    let direction: usize = 0;

    let steps_taken: Vec<(usize, usize)> = vec![];

    // Find security guard
    'outer: for (y, row) in board.rows_iter().enumerate() {
        for (x, elem) in row.enumerate() {
            if *elem == '^' {
                // Set the initial position
                guard_pos = (y, x);
                break 'outer;
            }
        }
    }

    let guard_origin = guard_pos.clone();

    let results = process_guard_movement(&board, guard_pos, direction, steps_taken);

    let total_steps = &results.0;

    let all_steps_taken = &results.1;

    println!("Number of steps taken: {}", total_steps);

    println!("Number of additional obstructions: {}", determine_new_obstructions(all_steps_taken, &board, guard_origin));


}


fn determine_new_obstructions(steps_taken: &Vec<(usize, usize)>, board: &Array2D<char>, mut guard_pos: (usize, usize)) -> u32 {
    let mut new_obstruction_count: u32 = 0;

    for step in steps_taken {
        let mut new_board = board.clone();
        new_board[*step] = '#';
        let obstructions    = HashMap::new();
        let is_looping = simulate_guard_movement(&new_board, guard_pos, 0, &obstructions);
        if is_looping {
            //println!("{:?}", step);
            new_obstruction_count += 1;
        }
    }
    return new_obstruction_count
}


fn simulate_guard_movement(board: &Array2D<char>, mut guard_pos: (usize, usize), direction: usize,
                           obstructions: &HashMap<(usize, usize), Vec<usize>>) -> bool {

    let mut is_obstructed: bool = false;

    let mut y_diff: isize = 0;
    let mut x_diff: isize = 0;

    match DIRECTIONS[direction] {
        "up" => {
            y_diff = -1;
            x_diff = 0;
        }
        "down" => {
            y_diff = 1;
            x_diff = 0;
        }
        "left" => {
            y_diff = 0;
            x_diff = -1;
        }
        "right" => {
            y_diff = 0;
            x_diff = 1;
        }
        _ => println!("Uncrecognized direction {}", direction)
    }

    let mut stuck: bool = false;

    let mut new_obstructions = obstructions.clone();

    while !stuck {
        let new_guard_pos = ((guard_pos.0 as isize + y_diff) as usize, (guard_pos.1 as isize + x_diff) as usize);
        if guard_pos.0 == board.num_rows() - 1 || guard_pos.1 == board.num_columns() - 1 || guard_pos.0 == 0 || guard_pos.1 == 0 {
            break;
        }
        else if board[new_guard_pos] == '#' {
            stuck = true;
            if new_obstructions.contains_key(&new_guard_pos) {
                let previous_approach_directions = new_obstructions.get(&new_guard_pos).unwrap();
                if previous_approach_directions.contains(&direction) {
                    is_obstructed = true;
                    break;
                } else {
                    new_obstructions.get_mut(&new_guard_pos).unwrap().push(direction);
                }
            } else {
                new_obstructions.insert(new_guard_pos, vec![direction]);
            }
            is_obstructed = simulate_guard_movement(board, guard_pos, (direction + 1) % 4, &new_obstructions)
            
        } else {
            guard_pos = new_guard_pos;
        }
    }

    return is_obstructed

}



fn process_guard_movement(board: &Array2D<char>, mut guard_pos: (usize, usize), direction: usize, 
                          mut steps_taken: Vec<(usize, usize)>) -> (u32, Vec<(usize, usize)>) {

    let mut y_diff: isize = 0;
    let mut x_diff: isize = 0;

    match DIRECTIONS[direction] {
        "up" => {
            y_diff = -1;
            x_diff = 0;
        }
        "down" => {
            y_diff = 1;
            x_diff = 0;
        }
        "left" => {
            y_diff = 0;
            x_diff = -1;
        }
        "right" => {
            y_diff = 0;
            x_diff = 1;
        }
        _ => println!("Uncrecognized direction {}", direction)
    }

    let mut steps: u32 = 0;

    let mut stuck: bool = false;

    while !stuck {
        //println!("{:?}", guard_pos);
        let new_guard_pos = ((guard_pos.0 as isize + y_diff) as usize, (guard_pos.1 as isize + x_diff) as usize);
        if guard_pos.0 == board.num_rows() - 1 || guard_pos.1 == board.num_columns() - 1 || guard_pos.0 == 0 || guard_pos.1 == 0 {
            steps += 1;
            break;
        }
        else if board[new_guard_pos] == '#' {
            stuck = true;
            let movement_results = process_guard_movement(board, guard_pos, (direction + 1) % 4, steps_taken[..].to_vec());
            steps += movement_results.0;
            steps_taken = movement_results.1;
        } else {
            if !steps_taken.contains(&new_guard_pos) {
                steps += 1;
                steps_taken.push(new_guard_pos);
            }
            guard_pos = new_guard_pos;
        }
    }

    return (steps, steps_taken)

}
