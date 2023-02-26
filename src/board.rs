use std::io::{Write, Stdout};
use termion::raw::RawTerminal;

use crate::player;

pub const FINISH_TILE: char = '☑';
pub const ALTERNATE_FINISH_TILE: char = 'f';
pub const WALL: char = 'w';
pub const DOORS: char = 'd';
pub const PATH: char = '.';
pub const PLAYER: char = 'P'; // 🨄
pub const COMMENT: char = '#';

pub fn draw_board(stdout: &mut RawTerminal<Stdout>, board: &Vec<Vec<char>>, position: &player::Position) {
    write!(stdout, "Use arrow keys to go through the maze. Type 'q' to quit\r\n").unwrap();
    write!(stdout, "w - wall, dot (.) - path, d - doors, ☑ - exit\r\n").unwrap();

    for (row_index, row) in board.iter().enumerate() {
        for (column_index, character) in row.iter().enumerate() {                
            if position.x == column_index && position.y == row_index {
                write!(stdout, "{PLAYER}").unwrap(); 
            } else {
                write!(stdout, "{character}").unwrap();
            }
        }
        write!(stdout, "\r\n").unwrap();
    }

    stdout.flush().unwrap();
}

pub fn parse_board_string(stdout: &mut RawTerminal<Stdout>, raw_data: String) 
 -> Result<Vec<Vec<char>>, &'static str>
{
    write!(stdout, "Read board data: {}\r\n", raw_data).unwrap();

    let mut result: Vec<Vec<char>> = vec![];
    for line in raw_data.lines() {
        if line.starts_with(COMMENT) || line.is_empty() {
            continue;
        }

        let mut chars: Vec<char> = vec![];

        for character in line.chars() {
            if character == FINISH_TILE || character == ALTERNATE_FINISH_TILE {
                chars.push(FINISH_TILE);
            } else if character == WALL || character == DOORS || character == PATH {
                chars.push(character);
            }
        }

        if chars.is_empty() == false {
            result.push(chars);
        }
    }

    if result.is_empty() {
        return Err("Board data is empty");
    }

    return Ok(result);
}