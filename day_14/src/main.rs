use std::fs::read_to_string;
use std::thread::sleep;
use std::time::Duration;
use itertools::Itertools;
use regex::Regex;

const WIDTH: isize = 101;
const HEIGHT: isize = 103;

//const WIDTH: isize = 11;
//const HEIGHT: isize = 7;


#[derive(Debug)]
#[derive(Clone)]
struct Robot {
    x: isize,
    y: isize,
    velocity: (isize, isize)
}

impl Robot {
    fn update_pos(&mut self, steps: isize) {
        let x: isize = (self.x + (steps * self.velocity.0)).rem_euclid(WIDTH);
        let y: isize = (self.y + (steps * self.velocity.1)).rem_euclid(HEIGHT);

        self.x = x;
        self.y = y;
    }
}

fn main() {
    let lines: Vec<String> = read_to_string("./puzzle_input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut robots: Vec<Robot> = parse_input(lines.clone());

    for i in 0..robots.len() {
        robots[i].update_pos(100);
    }

    let safety_factor = calculate_safety_factor(&robots);

    println!("Safety factor for part 1: {}", safety_factor);

    let mut new_robots: Vec<Robot> = parse_input(lines);

    print_board(new_robots);
}

fn print_board(mut robots: Vec<Robot>) {
    const width: usize= WIDTH as usize;
    const height: usize = HEIGHT as usize;
    let mut board: [[char; width]; height] = [[' '; width]; height];

    let mut i = 0;
    'outer: while i < 10000 {
        board = [[' '; width]; height];
        for j in 0..robots.len() {
            robots[j].update_pos(1);
        }

        for z in 0..robots.len() {
            board[robots[z].y as usize][robots[z].x as usize] = 'X';
        }

        let mut vert_found = false;
        let mut hori_found = false;

        for y in 0..height {
            for x in 0..width {
                if board[y][x] == 'X' {
                    if !vert_found {
                        for j in 0..20 {
                            if y + j >= height {
                                vert_found = false;
                                break
                            }
                            if board[y + j][x] == 'X' {
                                vert_found = true;
                            } else {
                                vert_found = false;
                                break
                            }
                        }
                    }
                    if !hori_found {
                        for j in 0..20 {
                            if x + j >= width {
                                hori_found = false;
                                break
                            }
                            if board[y][x + j] == 'X' {
                                hori_found = true;
                            } else {
                                hori_found = false;
                                break
                            }
                        }
                    }
                }
                if vert_found && hori_found {
                    println!("{i}");
                    break 'outer
                }
            }
        }

        i += 1;
    }
    for y in 0..HEIGHT {
        let row = board[y as usize].into_iter().join("");
        println!("{row}");
    }

}

fn calculate_safety_factor(robots: &Vec<Robot>) -> u32 {
    let mid_y = HEIGHT / 2;
    let mid_x = WIDTH / 2;

    let h_half_1 = 0..mid_x;
    let h_half_2 = (WIDTH - mid_x)..WIDTH;
    let v_half_1 = 0..mid_y;
    let v_half_2 = (HEIGHT - mid_y)..HEIGHT;

    let mut scores: [u32; 4] = [0; 4];

    for robot in robots {
        for (index, (v, h)) in [&v_half_1, &v_half_2].iter().cartesian_product([&h_half_1, &h_half_2].iter()).enumerate() {
            if v.contains(&robot.y) && h.contains(&robot.x) {
                scores[index] += 1;
            }
        }
    }

    let safety_factor = scores[0] * scores[1] * scores[2] * scores[3];

    return safety_factor
}

fn parse_input(lines: Vec<String>) -> Vec<Robot> {
    let mut robots: Vec<Robot> = vec![];

    for line in lines.into_iter() {
        let re_p = Regex::new(r"[0-9]+,[0-9]+ ").unwrap();
        let match_p = re_p.find(&line)
            .unwrap()
            .as_str()
            .replace(" ", "");
        let x = match_p.split(",")
            .collect::<Vec<&str>>()[0]
            .parse::<isize>()
            .unwrap();
        let y = match_p.split(",")
            .collect::<Vec<&str>>()[1]
            .parse::<isize>()
            .unwrap();

        let re_v = Regex::new(r"\-?[0-9]+,\-?[0-9]+$").unwrap();
        let match_v = re_v.find(&line)
            .unwrap()
            .as_str();
        let v_x = match_v.split(",")
            .collect::<Vec<&str>>()[0]
            .parse::<isize>()
            .unwrap();
        let v_y = match_v.split(",")
            .collect::<Vec<&str>>()[1]
            .parse::<isize>()
            .unwrap();

        let robot = Robot {
            x,
            y,
            velocity: (v_x, v_y)
        };

        robots.push(robot)
    }

    robots
}
