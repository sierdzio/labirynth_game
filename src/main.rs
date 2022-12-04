use std::io::{Write, stdout, stdin};
use std::convert::TryFrom;

use termion::input::TermRead;
use termion::event::Key;
use termion::raw::IntoRawMode;

fn draw_board(board: &[[char; 5]; 6], position: &[i32]) {
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "Use arrow keys to go through the maze. Type 'q' to quit\r\n").unwrap();
        write!(stdout, "w - wall, dot (.) - path, d - doors, ☑ - exit\r\n").unwrap();
        for (row_index, row) in board.iter().enumerate() {
            for (column_index, character) in row.iter().enumerate() {
                let row_i32 = match i32::try_from(row_index) {
                    Ok(v) => v,
                    Err(_) => panic!("Could not fit row index into i32!"),
                };
                
                let column_i32 = match i32::try_from(column_index) {
                    Ok(v) => v,
                    Err(_) => panic!("Could not fit column index into i32!"),
                };
                
                if position == [row_i32, column_i32] {
                    write!(stdout, "🨄").unwrap();
                } else {
                    write!(stdout, "{character}").unwrap();
                }
            }
            write!(stdout, "\r\n").unwrap();
        }

        stdout.flush().unwrap();
}

fn main() {
    let board = [
        ['.', '.', 'w', '.', '.'], 
        ['w', '.', 'w', 'w', 'w'], 
        ['w', '.', 'd', '.', 'w'], 
        ['.', '.', 'w', '.', 'w'], 
        ['w', 'w', 'w', '.', '☑'], 
        ['w', 'w', 'w', '.', 'w'], 
        ];

    // TODO: change this into a struct
    let mut position = [0, 0];

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    
    write!(stdout, "{}", termion::cursor::Hide).unwrap();

    draw_board(&board, &position);

    for c in stdin.keys() {
        write!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();

        match c.unwrap() {
            Key::Char('q') => {
                write!(stdout, "Quitting!\r\n").unwrap();
                break;
            },
            Key::Left => {
                write!(stdout, "Moving left\r\n").unwrap();
                position = [position[0], position[1] - 1];
            },
            Key::Right => {
                write!(stdout, "Moving right\r\n").unwrap();
                position = [position[0], position[1] + 1];
            },            
            Key::Up => {
                write!(stdout, "Moving up\r\n").unwrap();
                position = [position[0] - 1, position[1]];
            },
            Key::Down => {
                write!(stdout, "Moving down\r\n").unwrap();
                position = [position[0] + 1, position[1]];
            },
            _ => continue
        }

        draw_board(&board, &position);
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
