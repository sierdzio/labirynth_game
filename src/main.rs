use std::io::{Write, stdout, stdin};

use termion::input::TermRead;
use termion::event::Key;
use termion::raw::IntoRawMode;

use std::env;
use std::fs;

pub mod player;
pub mod board;
pub mod levels;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);

    // Set up terminal controls
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "{}", termion::cursor::Hide).unwrap();

    levels::draw_available_levels(&mut stdout, 1);

    // Read board data
    let path = "boards/default.board";
    let raw_data = match fs::read_to_string(path) {
        Ok(data) => data,
        Err(error) => panic!("{}: {}", error, path),
    };

    let board: Vec<Vec<char>> = match board::parse_board_string(&mut stdout, raw_data) {
        Ok(data) => data,
        Err(error) => panic!("{} Board path: {}", error, path),
    };

    let mut position = player::Position {
        x: 0,
        y: 0
    };    

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

        if board[position.y][position.x] == board::FINISH_TILE
            || board[position.y][position.x] == board::ALTERNATE_FINISH_TILE {
            write!(stdout, "\nSuccess! Congratulations!\n").unwrap();
            break;
        }
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
