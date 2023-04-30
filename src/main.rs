use std::io::{Write, stdout, stdin};

use termion::input::TermRead;
use termion::event::Key;
use termion::raw::IntoRawMode;

use std::fs;

pub mod player;
pub mod board;
pub mod levels;

fn main() {
    MainWindow::new().unwrap().run().unwrap();

    // Set up terminal controls
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "{}", termion::cursor::Hide).unwrap();

    let level = levels::let_user_select_level(&mut stdout);

    // Read board data
    let path = match level {
        Ok(level) => format!("boards/{}", level),
        Err(message) => panic!("Invalid level selected: {}", message),
    };

    let raw_data = match fs::read_to_string(path) {
        Ok(data) => data,
        Err(error) => panic!("{}", error),
    };

    let board: Vec<Vec<char>> = match board::parse_board_string(&mut stdout, raw_data) {
        Ok(data) => data,
        Err(error) => panic!("{}", error),
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

slint::slint! {
    component Tile inherits Rectangle {
        in property <string> character;

        width: 24px;
        height: 24px;
        background: #12aa12;
    
        Text {
            text: parent.character;
            color: #000000;
        }
    }

    export component MainWindow inherits Window {
        Tile {
            character: "w";
        }
    }
}