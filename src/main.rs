use std::io::{Write, stdout, stdin};
use termion::input::TermRead;
use termion::event::Key;
use termion::raw::IntoRawMode;

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

        write!(stdout, "Use arrow keys to go through the maze. Type 'q' to quit\r\n").unwrap();
        write!(stdout, "w - wall, dot (.) - path, d - doors, ☑ - exit\r\n").unwrap();
        for (row_index, row) in board.iter().enumerate() {
            for (column_index, character) in row.iter().enumerate() {
                if position == [row_index, column_index] {
                    write!(stdout, "🨄").unwrap();
                } else {
                    write!(stdout, "{character}").unwrap();
                }
            }
            write!(stdout, "\r\n").unwrap();
        }

        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
