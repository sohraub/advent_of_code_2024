use std::collections::HashMap;
use std::fs::read_to_string;


fn parse_input(filename: &str, fallen_bytes: usize, size: usize) -> Vec<Vec<char>> {
    let lines: Vec<String> = read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut board: Vec<Vec<char>> = vec![vec!['.'; size + 1]; size + 1];
    for line in &lines[0..fallen_bytes] {
        let coords = line.split(',')
            .map(|i| i.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        board[coords[1]][coords[0]] = '#';
    }

    board
}

fn parse_input_part_2(filename: &str, size: usize) -> (Vec<Vec<char>>, Vec<(usize, usize)>) {
    let lines: Vec<String> = read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let board: Vec<Vec<char>> = vec![vec!['.'; size + 1]; size + 1];
    let mut steps: Vec<(usize, usize)> = vec![];

    for line in &lines {
        let coords = line.split(',')
            .map(|i| i.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        steps.push((coords[1], coords[0]));
    }

    (board, steps)
}

fn check_directions(board: &Vec<Vec<char>>, (y, x): &(usize, usize)) -> Vec<(usize, usize)> {
    let size = board.len();
    let mut valid_moves: Vec<(usize, usize)> = vec![];
    let y = *y;
    let x = *x;
    if y > 0 && board[y - 1][x] == '.' {
        valid_moves.push((y - 1, x));
    }
    if y >= 0 && y + 1 < size && board[y + 1][x] == '.' {
        valid_moves.push((y + 1, x));
    }
    if x > 0 && board[y][x - 1] == '.' {
        valid_moves.push((y, x - 1));
    }
    if x >= 0 && x + 1 < size && board[y][x + 1] == '.' {
        valid_moves.push((y, x + 1));
    }
    valid_moves
}

fn djikstra(board: &Vec<Vec<char>>) -> u32 {
    let mut unvisited: HashMap<(usize, usize), u32> = HashMap::new();
    let mut visited: HashMap<(usize, usize), u32> = HashMap::new();
    let mut previous: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let size = board.len();

    for y in 0..size {
        for x in 0..size {
            if board[y][x] == '.' {
                if y == 0 && x == 0 {
                    unvisited.insert((y, x), 0);
                } else {
                    unvisited.insert((y, x), u32::MAX);
                }
            }
        }
    }

    while unvisited.keys().len() > 0 {
        let mut coord_to_check: (usize, usize) = (size, size);
        let mut smallest: u32 = u32::MAX - 1;
        for (coord, value ) in &unvisited {
            if *value < smallest {
                coord_to_check = *coord;
                smallest = *value;
            }
        }
        if coord_to_check.0 == size {
            break
        }
        let tiles_to_check = check_directions(board, &coord_to_check);
        for tile in tiles_to_check {
            if !unvisited.contains_key(&tile) {
                continue
            }
            let new_score = smallest + 1;
            if visited.contains_key(&tile) {
                let current_score = visited.get_mut(&tile).unwrap();
                if new_score < *current_score {
                    *current_score = new_score;
                    *(previous.get_mut(&tile).unwrap()) = coord_to_check;
                }
            }
            else {
                visited.insert(tile, new_score);
                previous.insert(tile, coord_to_check);
            }
            *(unvisited.get_mut(&tile).unwrap()) = new_score;
        }

        unvisited.remove(&coord_to_check);
    }
    
    if visited.get(&(size - 1, size - 1)).is_some() {
        return *(visited.get(&(size - 1, size -1)).unwrap())
    } else {
        return u32::MAX - 1
    }

}

fn part_one() {
    let board = parse_input("./puzzle_input.txt", 1024, 70);
    let score = djikstra(&board);

    println!("Part one answer: {score}");
}

fn find_blockage(mut board: Vec<Vec<char>>, steps: &Vec<(usize, usize)>, start: usize) -> usize {

    for (i, step) in steps.into_iter().enumerate() {
        let (y, x) = *step;
        println!("{i}");
        board[y][x] = '#';
        if i < start {
            continue
        }
        let score = djikstra(&board);
        if score == u32::MAX - 1 {
            return i
        }
    }

    return 0
}

fn part_two() {
    let (board, steps) = parse_input_part_2("./puzzle_input.txt", 70);
    let answer = find_blockage(board, &steps, 1024);
    println!("Part two answer: {answer}" );
    
}

fn main() {
    part_one();
    part_two();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_test_input() {
        let board = parse_input("./test_puzzle_input.txt", 12, 6);
        let coords: Vec<Vec<usize>> = read_to_string("./test_puzzle_input.txt")
            .unwrap()
            .lines()
            .map(|x| x.split(',').map(|i| i.parse::<usize>().unwrap())
                 .collect::<Vec<usize>>())
            .collect();

        for y in 0..=6 {
            for x in 0..=6 {
                if coords[..12].contains(&vec![x, y]) {
                    assert_eq!(board[y][x], '#');
                } else {
                    assert_eq!(board[y][x], '.');
                }
            }
        }
        assert_eq!(board.len(), 7);
    }

    #[test]
    fn test_djikstra() {
        let board = parse_input("./test_puzzle_input.txt", 12, 6);
        let result = djikstra(&board);
        assert_eq!(result, 22)
    }

    #[test]
    fn test_part_two() {
        let (board, steps) = parse_input_part_2("./test_puzzle_input.txt", 6);
        let result = find_blockage(board, &steps, 12);
        assert_eq!(result, 20)
    }
}