use std::fs::read_to_string;

struct Robot {
    x: usize,
    y: usize
}

impl Robot {
    fn make_move(&mut self, direction: &char, mut board: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let (mut dy, mut dx): (isize, isize) = (0, 0);
        match direction {
            '^' => (dy, dx) = (-1, 0),
            '<' => (dy, dx) = (0, -1),
            '>' => (dy, dx) = (0, 1),
            'v' => (dy, dx) = (1, 0),
            _ => println!("{direction} is an invalid direction input")
        }
        let x: isize = self.x as isize;
        let y: isize = self.y as isize;

        let (y1, x1): (usize, usize) = ((dy + y) as usize, (dx + x) as usize);
        if board[y1][x1] == '#' {
            return board
        }
        if board[y1][x1] == 'O' {
            let mut is_space: bool = false;
            let mut mult: isize = 2;
            while !is_space {
                let (yf, xf): (usize, usize) = ((y + mult * dy) as usize, (x + mult * dx) as usize);
                if board[yf][xf] == '#' {
                    return board
                }
                if board[yf][xf] == 'O' {
                    mult += 1;
                    continue
                }
                is_space = true;
            }
            self.x = x1;
            self.y = y1;
            board[y as usize][x as usize] = '.';
            board[self.y][self.x] = '@';

            for m in 2..(mult+1) {
                board[(y + m * dy) as usize][(x + m * dx) as usize] = 'O';
            }
        }
        if board[y1][x1] == '.' {
            self.x = x1;
            self.y = y1;
            board[y as usize][x as usize] = '.';
            board[self.y][self.x] = '@';
        }

        return board

    }

    fn make_move_v2(&mut self, direction: char, mut board: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let (mut dy, mut dx): (isize, isize) = (0, 0);
        match direction {
            '^' => (dy, dx) = (-1, 0),
            '<' => (dy, dx) = (0, -1),
            '>' => (dy, dx) = (0, 1),
            'v' => (dy, dx) = (1, 0),
            _ => println!("{direction} is an invalid direction input")
        }
        let x: isize = self.x as isize;
        let y: isize = self.y as isize;

        let (y1, x1): (usize, usize) = ((dy + y) as usize, (dx + x) as usize);
        if board[y1][x1] == '#' {
            return board
        }
        if board[y1][x1] == '[' || board[y1][x1] == ']' {
            let c = new_crate(x1, y1, board[y1][x1]);
            let mut mult: isize = 1;
            let mut is_space: bool = false;

            let mut crates: Vec<Crate> = vec![];
            crates.push(new_crate(x1, y1, board[y1][x1]));

            let new_crates = c.check_direction(direction, mult, &board);
            if new_crates.len() == 0 {
                return board
            }
            crates.extend(new_crates);

            println!("debug");

            
            for new_c in &crates {
                board[new_c.left.0][new_c.left.1] = '.';
                board[new_c.right.0][new_c.right.1] = '.';
            }
            

            for new_c in crates.into_iter().rev() {
                board = new_c.move_crate(direction, board);
            }

            self.x = x1;
            self.y = y1;
            board[y as usize][x as usize] = '.';
            board[self.y][self.x] = '@';
        }

        if board[y1][x1] == '.' {
            self.x = x1;
            self.y = y1;
            board[y as usize][x as usize] = '.';
            board[self.y][self.x] = '@';
        }

        return board
    }
}

struct Crate {
    left: (usize, usize),
    right: (usize, usize)
}

impl Crate {
    fn check_direction(&self, direction: char, mult: isize, board: &Vec<Vec<char>>) -> Vec<Crate> {
        let (mut dy, mut dx) = (0, 0);
        match direction {
            '^' => (dy, dx) = (-1, 0),
            '<' => (dy, dx) = (0, -1),
            '>' => (dy, dx) = (0, 1),
            'v' => (dy, dx) = (1, 0),
            _ => println!("{direction} is an invalid direction input")
        }

        let me = new_crate(self.left.1, self.left.0, '[');
        let c = new_crate((self.left.1 as isize + mult * dx) as usize, (self.left.0 as isize + mult * dy) as usize,  '[');
        if board[c.left.0][c.left.1] == '#' || board[c.right.0][c.right.1] == '#' {
            return vec![]
        }
        let mut crates = vec![me];
        if direction == '<' && board[c.left.0][c.left.1] == '.' {
            return crates
        }
        if direction == '>' && board[c.right.0][c.right.1] == '.' {
            return crates
        }
        if ['v', '^'].contains(&direction) && board[c.left.0][c.left.1] == '.' && board[c.right.0][c.right.1] == '.' {
            return crates
        }

        if board[c.left.0][c.left.1] == '[' && board[c.right.0][c.right.1] == ']' {
            let new_crates = c.check_direction(direction, mult, board);
            if new_crates.len() == 0 {
                return vec![]
            }
            crates.extend(new_crates);
            return crates
        }

        if direction == '<' && board[c.left.0][c.left.1] == ']' {
            let new_c = new_crate(c.left.1, c.left.0, ']');
            let new_crates = new_c.check_direction(direction, mult, board);
            if new_crates.len() == 0 {
                return vec![]
            }
            crates.extend(new_crates)
        }

        if direction == '>' && board[c.right.0][c.right.1] == '[' {
            let new_c = new_crate(c.right.1, c.right.0, board[c.right.0][c.right.1]);
            let new_crates = new_c.check_direction(direction, mult, board);
            if new_crates.len() == 0 {
                return vec![]
            }
            crates.extend(new_crates)
        }

        if direction == '^' || direction == 'v' {
            if board[c.left.0][c.left.1] == ']' {
                let new_c = new_crate(c.left.1, c.left.0, ']');
                let new_crates_left = new_c.check_direction(direction, mult, board);
                if new_crates_left.len() == 0 {
                    return vec![]
                }
                crates.extend(new_crates_left);
            }

            if board[c.right.0][c.right.1] == '[' {
                let new_c = new_crate(c.right.1, c.right.0, board[c.right.0][c.right.1]);
                let new_crates_right = new_c.check_direction(direction, mult, board);
                if new_crates_right.len() == 0 {
                    return vec![]
                }
                crates.extend(new_crates_right);
            }

        }

        return crates
    }

    fn move_crate(&self, direction: char, mut board: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let (mut dy, mut dx) = (0, 0);
        match direction {
            '^' => (dy, dx) = (-1, 0),
            '<' => (dy, dx) = (0, -1),
            '>' => (dy, dx) = (0, 1),
            'v' => (dy, dx) = (1, 0),
            _ => println!("{direction} is an invalid direction input")
        }

        board[(self.right.0 as isize + dy) as usize][(self.right.1 as isize + dx) as usize] = ']';
        board[(self.left.0 as isize + dy) as usize][(self.left.1 as isize + dx) as usize] = '[';

        /*
        match direction {
            '^' | 'v' => {
                board[self.right.0][self.right.1] = '.';
                board[self.left.0][self.left.1] = '.';
            },
            '<' => board[self.right.0][self.right.1] = '.',
            '>' => board[self.left.0][self.left.1] = '.',
            _ => println!("{direction} is an invalid direction input")
        }
        */

        return board

    }
}

fn new_crate(x: usize, y: usize, half: char) -> Crate {
    if half == '[' {
        return Crate {
            left: (y, x),
            right: (y, x + 1)
        }
    } else {
        return Crate {
            left: (y, x - 1),
            right: (y, x)
        }
    }
}

fn main() {
    let lines: Vec<String> = read_to_string("./puzzle_input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut board: Vec<Vec<char>> = vec![];
    let mut moves: Vec<char> = vec![];

    for line in &lines {
        if line.contains("#") {
            board.push(line.chars().collect());
        } else if line.len() > 0 {
            let mut x: Vec<char> = line.chars().collect();
            if x[x.len() - 1] == '\n' {
                x.pop();
            }
            moves.extend(x);
        }
    }

    let mut robot: Robot = Robot{x:0, y:0};
    for (y, row) in (&board).into_iter().enumerate() {
        for (x, elem) in row.into_iter().enumerate() {
            if *elem == '@' {
                robot = Robot{
                    x,
                    y
                };
            }
        }
    }

    for direction in &moves {
        board = robot.make_move(&direction, board);
    }

    let mut box_score_sum: u64 = 0;

    for (y, row) in (&board).into_iter().enumerate() {
        for (x, elem) in row.into_iter().enumerate() {
            if *elem == 'O' {
                box_score_sum += ((y * 100) + x) as u64;
            }
        }
    }

    println!("Part 1 answer: {box_score_sum}");

    let mut board_v2: Vec<Vec<char>> = vec![];
    for line in &lines { 
        if !line.contains('#'){
            continue;
        }
        let mut board_line: Vec<char> = vec![];
        for elem in line.chars() {
            if elem == 'O' {
                board_line.extend(['[', ']']);
            } else if elem == '@' {
                board_line.extend(['@', '.']);
            } else {
                board_line.extend([elem, elem]);
            }
        }
        board_v2.push(board_line);
    }

    let mut robot: Robot = Robot{x:0, y:0};
    for (y, row) in (&board_v2).into_iter().enumerate() {
        for (x, elem) in row.into_iter().enumerate() {
            if *elem == '@' {
                robot = Robot{
                    x,
                    y
                };
            }
        }
    }

    for line in &board_v2 {
        let s :String = line.iter().collect();
        println!("{}", s);
    }

    let debug: bool = false;
    for direction in &moves {
        if debug {
            println!("~~~~~~~~~~~~~~ {direction} ~~~~~~~~~~~~~~");
        }
        board_v2 = robot.make_move_v2(*direction, board_v2);
        if debug {
            for line in &board_v2 {
                let s :String = line.iter().collect();
                println!("{}", s);
            }
        }
        for line in &board_v2 {
            for (i, elem) in line.into_iter().enumerate() {
                if *elem == '[' && line[i + 1] != ']' {
                    println!("here");
                }
                if *elem == ']' && line[i - 1] != '[' {
                    println!("here");
                }
            }
        }
    }
    for line in &board_v2 {
        let s :String = line.iter().collect();
        println!("{}", s);
    }

    let mut box_score_sum: u64 = 0;

    for (y, row) in (&board_v2).into_iter().enumerate() {
        for (x, elem) in row.into_iter().enumerate() {
            if *elem == '[' {
                box_score_sum += ((y * 100) + x) as u64;
            }
        }
    }

    println!("Part 2 answer: {box_score_sum}");
}
