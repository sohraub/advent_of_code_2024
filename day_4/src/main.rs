use std::fs::read_to_string;
use array2d::Array2D;

fn main() {
    let input_file: &str = "./puzzle_input.txt";
    let lines: Vec<String> = read_to_string(input_file)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    let mut board_vec: Vec<Vec<char>> = vec![];
    for line in lines {
        board_vec.push(line.chars().collect());
    }


    let board = Array2D::from_rows(&board_vec).unwrap();

    let mut found_count_x: u32 = 0;
    let mut found_count_a: u32 = 0;

    for (y, row) in board.rows_iter().enumerate() {
        for (x, elem) in row.enumerate() {
            if *elem == 'X' {
                found_count_x += handle_x(&board, &x, &y)
            } else if *elem == 'A' {
                found_count_a += handle_a(&board, &x, &y)
            }
        }
    }

    println!("Result of part 1: {}", found_count_x);
    println!("Result of part 2: {}", found_count_a);

}


fn handle_a(board: &Array2D<char>, x: &usize, y:&usize) -> u32 {
    let mut count: u32 = 0;

    let height: usize = board.num_rows();
    let width: usize = board.num_columns();

    if 1 <= *x && *x <= width - 2  && 1 <= *y && *y <= height - 2 {
        // Check if S-A-M
        if board[(*y - 1, *x - 1)] == 'S' && board[(*y + 1, *x + 1)] == 'M' {
            if board[(*y + 1, *x - 1)] == 'S' && board[(*y - 1, *x + 1)] == 'M' {
                count += 1
            } else if board[(*y + 1, *x - 1)] == 'M' && board[(*y - 1, *x + 1)] == 'S' {
                count += 1
            }
        }
        // Check if M-A-S
        else if board[(*y - 1, *x - 1)] == 'M' && board[(*y + 1, *x + 1)] == 'S' {
            if board[(*y + 1, *x - 1)] == 'S' && board[(*y - 1, *x + 1)] == 'M' {
                count += 1
            } else if board[(*y + 1, *x - 1)] == 'M' && board[(*y - 1, *x + 1)] == 'S' {
                count += 1
            }
        }
    }

    return count;
}


fn handle_x(board: &Array2D<char>, x: &usize , y: &usize) -> u32 {
    let mut count: u32 = 0;

    let height: usize = board.num_rows();
    let width: usize = board.num_columns();
    // Check horizontal forward
    if *x <= width - 4 {
        if board[(*y, *x + 1)] == 'M' && board[(*y, *x + 2)] == 'A' && board[(*y, *x + 3)] == 'S' {
            count += 1;
        }
    }
    // Check diagonal forward and up
    if *x <= width - 4  && *y >= 3 {
        if board[(*y - 1, *x + 1)] == 'M' && board[(*y - 2, *x + 2)] == 'A' && board[(*y - 3, *x + 3)] == 'S' {
            count += 1;
        }
    }
    // Check diagonal forward and down
    if *x <= width - 4  && *y <= height - 4 {
        if board[(*y + 1, *x + 1)] == 'M' && board[(*y + 2, *x + 2)] == 'A' && board[(*y + 3, *x + 3)] == 'S' {
            count += 1;
        }
    }
    // Check horizontal backwards
    if *x >= 3 {
        if board[(*y, *x - 1)] == 'M' && board[(*y, *x - 2)] == 'A' && board[(*y, *x - 3)] == 'S' {
            count += 1;
        }
    }
    // Check diagonal backwards and up
    if *x >= 3 && *y >= 3{
        if board[(*y - 1, *x - 1)] == 'M' && board[(*y - 2, *x - 2)] == 'A' && board[(*y - 3, *x - 3)] == 'S' {
            count += 1;
        }
    }
    // Check diagonal backwards and down
    if *x >= 3 && *y <= height - 4{
        if board[(*y + 1, *x - 1)] == 'M' && board[(*y + 2, *x - 2)] == 'A' && board[(*y + 3, *x - 3)] == 'S' {
            count += 1;
        }
    }
    // Check vertical up
    if *y >= 3 {
        if board[(*y - 1, *x)] == 'M' && board[(*y - 2, *x)] == 'A' && board[(*y - 3, *x)] == 'S' {
            count += 1;
        }
    }
    // Check vertical down
    if *y <= height - 4 {
        if board[(*y + 1, *x)] == 'M' && board[(*y + 2, *x)] == 'A' && board[(*y + 3, *x)] == 'S' {
            count += 1;
        }
    }

    return count
}
