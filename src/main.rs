use std::io::{Write, Stdout, stdout, stdin};

use termion::input::TermRead;
use termion::event::Key;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;

struct Position {
    x: usize,
    y: usize,
}

const FINISH_TILE: char = '☑';

enum Direction {
    Unknown,
    Left,
    Right,
    Up,
    Down
}

fn draw_board(stdout: &mut RawTerminal<Stdout>, board: &[[char; 5]; 6], position: &Position) {
    write!(stdout, "Use arrow keys to go through the maze. Type 'q' to quit\r\n").unwrap();
    write!(stdout, "w - wall, dot (.) - path, d - doors, ☑ - exit\r\n").unwrap();

    for (row_index, row) in board.iter().enumerate() {
        for (column_index, character) in row.iter().enumerate() {                
            if position.x == row_index && position.y == column_index {
                write!(stdout, "P").unwrap(); // 🨄
            } else {
                write!(stdout, "{character}").unwrap();
            }
        }
        write!(stdout, "\r\n").unwrap();
    }

    stdout.flush().unwrap();
}

fn try_to_move(board: &[[char; 5]; 6], position: &mut Position, direction: Direction) {
    match direction {
        Direction::Unknown => {
        },
        Direction::Left => {
            position.y = position.y - 1;
        },        
        Direction::Right => {
            position.y = position.y + 1;
        },
        Direction::Up => {
            position.x = position.x - 1;
        },
        Direction::Down => {
            position.x = position.x + 1;
        }
    }
}

fn main() {
    let board = [
        ['.', '.', 'w', '.', '.'], 
        ['w', '.', 'w', 'w', 'w'], 
        ['w', '.', 'd', '.', 'w'], 
        ['.', '.', 'w', '.', 'w'], 
        ['w', 'w', 'w', '.', FINISH_TILE], 
        ['w', 'w', 'w', '.', 'w'], 
        ];

    let mut position = Position {
        x: 0,
        y: 0
    };

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    
    write!(stdout, "{}", termion::cursor::Hide).unwrap();

    draw_board(&mut stdout, &board, &position);

    let mut direction: Direction = Direction::Unknown;

    for c in stdin.keys() {
        write!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();

        match c.unwrap() {
            Key::Char('q') => {
                write!(stdout, "Quitting!\r\n").unwrap();
                break;
            },
            Key::Left => {
                write!(stdout, "Moving left\r\n").unwrap();
                direction = Direction::Left;
            },
            Key::Right => {
                write!(stdout, "Moving right\r\n").unwrap();
                direction = Direction::Right;
            },            
            Key::Up => {
                write!(stdout, "Moving up\r\n").unwrap();
                direction = Direction::Up;
            },
            Key::Down => {
                write!(stdout, "Moving down\r\n").unwrap();
                direction = Direction::Down;
            },
            _ => continue
        }

        try_to_move(&board, &mut position, direction);
        draw_board(&mut stdout, &board, &position);
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
