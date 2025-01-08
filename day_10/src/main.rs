use std::fs::read_to_string;
use array2d::Array2D;

fn main() {
    let lines: Vec<String> = read_to_string("./puzzle_input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut board_vec: Vec<Vec<u32>> = vec![];

    for line in lines {
        board_vec.push(line.chars().map(|i| -> u32{i.to_digit(10).unwrap()}).collect());
    }

    let board: Array2D<u32> = Array2D::from_rows(&board_vec).unwrap();

    let part_1_answer = get_trailhead_score_sum(&board);
    println!("Part 1 answer: {part_1_answer}");

    let part_2_answer = get_trailhead_rating_sum(&board);
    println!("Part 2 answer: {part_2_answer}");

}

fn get_possible_directions(board_length: &usize, board_width: &usize, current_position: (&usize, &usize)) -> Vec<String> {
    let mut possible_directions: Vec<String> = vec![];

    if *current_position.0 > 0 {
        possible_directions.push("up".to_string());
    }
    if *current_position.1 > 0 {
        possible_directions.push("left".to_string());
    }
    if *current_position.0 < board_length - 1 {
        possible_directions.push("down".to_string());
    }
    if *current_position.1 < board_width - 1 {
        possible_directions.push("right".to_string())
    }

    return possible_directions
}

fn get_return_direction(direction: &str) -> String {
    if direction == "up" {
        return "down".to_string()
    } else if direction == "down" {
        return "up".to_string()
    } else if direction == "right" {
        return "left".to_string()
    } else {
        return "right".to_string()
    }
}

fn get_trailhead_score_sum(board: &Array2D<u32>) -> u32 {
    let mut trailhead_score_sum: u32 = 0;

    let board_length: usize = board.column_len();
    let board_width: usize = board.row_len();

    for (y, row) in board.rows_iter().enumerate() {
        for (x, value) in row.enumerate() {
            if *value == 0 {
                let mut completed_summits: Vec<(usize, usize)> = vec![];
                for direction in get_possible_directions(&board_length, &board_width, (&y, &x)) {
                    let new_summits = check_direction(board, (y, x), &direction, &board_length, &board_width);
                    for summit in new_summits {
                        if !completed_summits.contains(&summit) {
                            completed_summits.push(summit);
                        }
                    }
                }
                trailhead_score_sum += completed_summits.len() as u32;
            }
          }
    }

    return trailhead_score_sum
}


fn check_direction(board: &Array2D<u32>, current_position: (usize, usize), direction: &str, length: &usize, width: &usize) -> Vec<(usize, usize)> {

    let new_position: (usize, usize)= match direction {
        "up" => (current_position.0 - 1, current_position.1),
        "down" => (current_position.0 + 1, current_position.1),
        "left" => (current_position.0, current_position.1 - 1),
        "right" => (current_position.0, current_position.1 + 1),
        _ => (0, 0)
    };

    let new_value: u32 = board[new_position];
    let current_value: u32 = board[current_position];

    let mut all_summits: Vec<(usize, usize)> = vec![];

    if current_value == 8 && new_value == 9 {
            return vec![new_position]
    } else if new_value == current_value + 1 {
        let possible_next_directions = get_possible_directions(length, width, (&new_position.0, &new_position.1));
        let return_direction = get_return_direction(direction);
        for new_direction in possible_next_directions {
            if new_direction == return_direction {
                continue
            }
            let new_summits = check_direction(board, new_position, &new_direction, length, width);
            for summit in new_summits {
                if !all_summits.contains(&summit) {
                    all_summits.push(summit)
                }
            }
        }
        return all_summits
    } else {
        return vec![]
    }

}

fn get_trailhead_rating_sum(board: &Array2D<u32>) -> u32 {
    let mut trailhead_score_sum: u32 = 0;

    let board_length: usize = board.column_len();
    let board_width: usize = board.row_len();

    for (y, row) in board.rows_iter().enumerate() {
        for (x, value) in row.enumerate() {
            if *value == 0 {
                for direction in get_possible_directions(&board_length, &board_width, (&y, &x)) {
                    trailhead_score_sum += check_route_rating(board, (y, x), &direction, &board_length, &board_width);
                }
            }
          }
    }

    return trailhead_score_sum
}


fn check_route_rating(board: &Array2D<u32>, current_position: (usize, usize), direction: &str, length: &usize, width: &usize) -> u32 {

    let new_position: (usize, usize)= match direction {
        "up" => (current_position.0 - 1, current_position.1),
        "down" => (current_position.0 + 1, current_position.1),
        "left" => (current_position.0, current_position.1 - 1),
        "right" => (current_position.0, current_position.1 + 1),
        _ => (0, 0)
    };

    let new_value: u32 = board[new_position];
    let current_value: u32 = board[current_position];

    let mut total_route_rating: u32 = 0;

    if current_value == 8 && new_value == 9 {
            return 1
    } else if new_value == current_value + 1 {
        let possible_next_directions = get_possible_directions(length, width, (&new_position.0, &new_position.1));
        let return_direction = get_return_direction(direction);
        for new_direction in possible_next_directions {
            if new_direction == return_direction {
                continue
            }
            total_route_rating += check_route_rating(board, new_position, &new_direction, length, width);
        }
        return total_route_rating
    } else {
        return 0
    }

}
