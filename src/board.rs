use std::io::{Write, Stdout};
use termion::raw::RawTerminal;

pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub fn draw_board(stdout: &mut RawTerminal<Stdout>, board: &[[char; 5]; 6], position: &Position) {
    write!(stdout, "Use arrow keys to go through the maze. Type 'q' to quit\r\n").unwrap();
    write!(stdout, "w - wall, dot (.) - path, d - doors, ☑ - exit\r\n").unwrap();

    for (row_index, row) in board.iter().enumerate() {
        for (column_index, character) in row.iter().enumerate() {                
            if position.x == column_index && position.y == row_index {
                write!(stdout, "P").unwrap(); // 🨄
            } else {
                write!(stdout, "{character}").unwrap();
            }
        }
        write!(stdout, "\r\n").unwrap();
    }

    stdout.flush().unwrap();
}