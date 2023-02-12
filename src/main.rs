use std::io::{Write, stdout, stdin};

use termion::input::TermRead;
use termion::event::Key;
use termion::raw::IntoRawMode;

pub mod board;

const FINISH_TILE: char = '☑';
const WALL: char = 'w';
const DOORS: char = 'd';
const PATH: char = '.';

enum Direction {
    Unknown,
    Left,
    Right,
    Up,
    Down
}

fn try_to_move(board: &[[char; 5]; 6], position: &mut board::Position, direction: Direction) {
    match direction {
        Direction::Unknown => {
        },
        Direction::Left => {
            let new_pos: usize;
            
            if position.x > 0 {
                new_pos = position.x - 1;
            } else {
                new_pos = position.x;
            }

            if board[position.y][new_pos] != WALL {
                position.x = new_pos;
            }
        },        
        Direction::Right => {
            let new_pos: usize;
            
            if position.x < 4 {
                new_pos = position.x + 1;
            } else {
                new_pos = position.x;
            }

            if board[position.y][new_pos] != WALL {
                position.x = new_pos;
            }
        },
        Direction::Up => {
            let new_pos: usize;
            
            if position.y > 0 {
                new_pos = position.y - 1;
            } else {
                new_pos = position.y;
            }

            if board[new_pos][position.x] != WALL {
                position.y = new_pos;
            }
        },
        Direction::Down => {
            let new_pos: usize;
            
            if position.y < 5 {
                new_pos = position.y + 1;
            } else {
                new_pos = position.y;
            }

            if board[new_pos][position.x] != WALL {
                position.y = new_pos;
            }
        }
    }
}

fn main() {
    let board = [
        [PATH, PATH, WALL, PATH, PATH], 
        [WALL, PATH, WALL, WALL, WALL], 
        [WALL, PATH, DOORS, PATH, WALL], 
        [PATH, PATH, WALL, PATH, WALL], 
        [WALL, WALL, WALL, PATH, FINISH_TILE], 
        [WALL, WALL, WALL, PATH, WALL], 
        ];

    let mut position = board::Position {
        x: 0,
        y: 0
    };

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    
    write!(stdout, "{}", termion::cursor::Hide).unwrap();

    board::draw_board(&mut stdout, &board, &position);

    let mut direction: Direction;

    for c in stdin.keys() {
        write!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();

        let mut move_text: &str = "";

        match c.unwrap() {
            Key::Char('q') => {
                write!(stdout, "Quitting!\r\n").unwrap();
                break;
            },
            Key::Left => {
                move_text = "Moving left\r\n";
                direction = Direction::Left;
            },
            Key::Right => {
                move_text = "Moving right\r\n";
                direction = Direction::Right;
            },            
            Key::Up => {
                move_text = "Moving up\r\n";
                direction = Direction::Up;
            },
            Key::Down => {
                move_text = "Moving down\r\n";
                direction = Direction::Down;
            },
            _ => {
                direction = Direction::Unknown;
            }
        }

        try_to_move(&board, &mut position, direction);
        board::draw_board(&mut stdout, &board, &position);
        write!(stdout, "{} \r\n", move_text).unwrap();

        if board[position.y][position.x] == FINISH_TILE {
            write!(stdout, "\nSuccess! Congratulations!\n").unwrap();
            break;
        }
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
