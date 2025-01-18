use std::fs::read_to_string;
use std::collections::HashMap;
use std::u32;


fn parse_input(filename: &str) -> Vec<Vec<char>> {
    let board = read_to_string(filename)
        .unwrap()
        .split("\r\n")
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    board
}

fn get_possible_moves((y, x): (usize, usize), board: &Vec<Vec<char>>, get_walls: bool, reverse: bool) -> Vec<(usize, usize)> {
    let size = board.len();
    let mut valid_moves: Vec<(usize, usize)> = vec![];
    let mut valid_tiles: Vec<char> = vec!['.', 'E'];
    if reverse {
        valid_tiles = vec!['.', 'S'];
    }
    if y > 0 {
        if get_walls || valid_tiles.contains(&board[y - 1][x]) {
            valid_moves.push((y - 1, x));
        }
    }
    if y + 1 < size {
		if get_walls || valid_tiles.contains(&board[y + 1][x]) {
            valid_moves.push((y + 1, x));
        }
    }
    if x > 0 {
		if get_walls || valid_tiles.contains(&board[y][x - 1]) {
            valid_moves.push((y, x - 1));
        }
    }
    if x + 1 < size {
		if get_walls || valid_tiles.contains(&board[y][x + 1]) {
            valid_moves.push((y, x + 1));
        }
    }
    valid_moves

}

fn get_all_points_within_distance((y, x): &(usize, usize), distance: i32, board: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let x = *x as i32;
    let y = *y as i32;
    let mut valid_tiles = vec![];
    for d in -1*distance..=distance {
        let new_x = x + d;
        if new_x < 0 || new_x > (board[0].len() - 1) as i32 {
            continue
        }
        for f in -1*distance..=distance {
            if f == 0 && d == 0 {
                continue
            }
            if f.abs() + d.abs() > distance {
                continue
            }
            let new_y = y + f;
            if new_y < 0 || new_y > (board.len() - 1) as i32 {
                continue
            }
            let new_tile = (new_y as usize, new_x as usize);
            if board[new_tile.0][new_tile.1] != '#' {
                valid_tiles.push(new_tile);
            } 
        }
    }
    return valid_tiles
}

fn djikstra(board: &Vec<Vec<char>>, reverse: bool) -> (u32, HashMap<(usize, usize), u32>) {
    let mut end: (usize, usize) = (0, 0);
    let mut visited: HashMap<(usize, usize), u32> = HashMap::new();
    let mut unvisited: HashMap<(usize, usize), u32> = HashMap::new();
    let mut previous: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    for (y, row) in board.into_iter().enumerate() {
        for (x, elem) in row.into_iter().enumerate() {
            if *elem == 'S' {
                if reverse {
                    unvisited.insert((y, x), u32::MAX);
                    end = (y, x);
                } else {
                    unvisited.insert((y, x), 0);
                }
            } else {
                unvisited.insert((y, x), u32::MAX);
            }
            if *elem == 'E' {
                if reverse {
                    unvisited.insert((y, x), 0);
                } else {
                    end = (y, x);
                }
            }
        }
    }
    while unvisited.keys().len() > 0 {
        let mut current: (usize, usize) = (0, 0);
        let mut min_score = u32::MAX - 1;
        for (coord, score) in &unvisited {
            if *score < min_score {
                min_score = *score;
                current = *coord;
            }
        }
        if current == (0, 0) {
            break
        }
        let new_score = unvisited.get(&current).unwrap() + 1;
        for next in get_possible_moves(current, board, false, reverse) {
            if visited.contains_key(&next) {
                continue
            }
            if *unvisited.get(&next).unwrap() > new_score {
                unvisited.insert(next, new_score);
                previous.insert(next, current);
            }
        }
        visited.insert(current, min_score);
        unvisited.remove(&current);
    }

    let final_score = visited.get(&end).unwrap_or(&u32::MAX);

    return (final_score.clone(), visited)
}

fn get_mh_distance(p1: &(usize, usize), p2: &(usize, usize)) -> u32 {
    let (y1, x1) = (p1.0 as isize, p1.1 as isize);
    let (y2, x2) = (p2.0 as isize, p2.1 as isize);
    let distance = (x1 - x2).abs() + (y1-y2).abs();
    return distance as u32
}

fn part_one_mk2(filename: &str, target_diff: u32, cheat_length: i32) -> u32 {
    let board = parse_input(filename);
    let (score, visisted) = djikstra(&board, false);
    let (_, r_visisted) = djikstra(&board, true);

    let mut result = 0;
    for (coord, value) in &visisted {
        let tiles = get_all_points_within_distance(&coord, cheat_length, &board);
        for tile in tiles {
            let r_score = *r_visisted.get(&tile).unwrap();
            let distance = get_mh_distance(coord, &tile);
            let cheat_score = value + r_score + distance;
            if cheat_score < score && score - cheat_score >= target_diff {
                result += 1;
            }
        }
    }
    result
}

fn main() {
    let part_one_answer = part_one_mk2("puzzle_input.txt", 100, 2);
    println!("Part one answer is {part_one_answer}");

    let part_two_answer = part_one_mk2("puzzle_input.txt", 100, 20);
    println!("Part one answer is {part_two_answer}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input() {
        let board = parse_input(&"test_puzzle_input.txt");
        assert_eq!(board.len(), 15);
        assert_eq!(board[0].len(), 15);
    }

    #[test]
    fn test_get_possible_moves() {
        let board = parse_input(&"test_puzzle_input.txt");
        let moves = get_possible_moves((1, 2), &board, false, false);
        assert_eq!(moves, vec![(1, 1), (1, 3)]);
    }

    #[test]
    fn test_get_possible_moves_with_walls() {
        let board = parse_input(&"test_puzzle_input.txt");
        let moves = get_possible_moves((1, 2), &board, true, false);
        assert_eq!(moves, vec![(0, 2), (2, 2), (1, 1), (1, 3)]);
    }

    #[test]
    fn test_part_one() {
        let answer = part_one_mk2(&"test_puzzle_input.txt", 0, 2);

        assert_eq!(answer, 44);
    }

    #[test]
    fn test_get_tiles_within_distance() {
        let board = parse_input(&"test_puzzle_input.txt");
        let new_tiles = get_all_points_within_distance(&(2, 3), 2, &board);
        println!("{:?}", new_tiles);
        assert_eq!(new_tiles.len(), 6);
    }
}