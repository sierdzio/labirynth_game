use std::io::{Write, stdout, stdin};

use termion::input::TermRead;
use termion::event::Key;
use termion::raw::IntoRawMode;

pub mod player;
pub mod board;

fn main() {
    let board: Vec<Vec<char>> = vec![
        vec![board::WALL, board::PATH, board::WALL, board::WALL, board::WALL], 
        vec![board::PATH, board::PATH, board::WALL, board::PATH, board::PATH], 
        vec![board::WALL, board::PATH, board::DOORS, board::PATH, board::WALL], 
        vec![board::PATH, board::PATH, board::WALL, board::PATH, board::WALL], 
        vec![board::WALL, board::WALL, board::WALL, board::PATH, board::FINISH_TILE], 
        vec![board::WALL, board::WALL, board::WALL, board::PATH, board::WALL], 
        ];

    let mut position = player::Position {
        x: 0,
        y: 0
    };

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    
    write!(stdout, "{}", termion::cursor::Hide).unwrap();

    board::draw_board(&mut stdout, &board, &position);

    let mut direction: player::Direction;

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
                direction = player::Direction::Left;
            },
            Key::Right => {
                move_text = "Moving right\r\n";
                direction = player::Direction::Right;
            },            
            Key::Up => {
                move_text = "Moving up\r\n";
                direction = player::Direction::Up;
            },
            Key::Down => {
                move_text = "Moving down\r\n";
                direction = player::Direction::Down;
            },
            _ => {
                direction = player::Direction::Unknown;
            }
        }

        player::try_to_move(&board, &mut position, direction);
        board::draw_board(&mut stdout, &board, &position);
        write!(stdout, "{} \r\n", move_text).unwrap();

        if board[position.y][position.x] == board::FINISH_TILE {
            write!(stdout, "\nSuccess! Congratulations!\n").unwrap();
            break;
        }
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
