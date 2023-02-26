use std::io::{Write, Stdout};
use termion::raw::RawTerminal;

use crate::player;

pub const FINISH_TILE: char = '☑';
pub const ALTERNATE_FINISH_TILE: char = 'f';
pub const WALL: char = 'w';
pub const DOORS: char = 'd';
pub const PATH: char = '.';

pub fn draw_board(stdout: &mut RawTerminal<Stdout>, board: &Vec<Vec<char>>, position: &player::Position) {
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

pub fn parse_board_string(stdout: &mut RawTerminal<Stdout>, raw_data: String) 
 -> Result<Vec<Vec<char>>, ()>
{
    write!(stdout, "Read board data: {}\r\n", raw_data).unwrap();

    return Result::Err(());
}