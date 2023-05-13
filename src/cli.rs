use std::io::{Write, stdout, stdin};

use termion::input::TermRead;
use termion::event::Key;
use termion::raw::IntoRawMode;

use std::{fs, process};

use crate::player;
use crate::board;
use crate::levels;

const NEW_LINE: &str = "\r\n";

pub fn draw_ui_choice(launch_gui: bool) {
    let mut std_out = stdout().into_raw_mode().unwrap();
    write!(std_out, "{}", termion::cursor::Hide).unwrap();
    write!(std_out, "{}", termion::clear::All).unwrap();
    write!(std_out, "{}", termion::cursor::Goto(1, 1)).unwrap();

    let cli_char = match launch_gui {
        true => '-',
        false => '*'
    };

    let gui_char = match launch_gui {
        true => '*',
        false => '-'
    };

    write!(std_out, "Play in terminal or GUI? (press 'q' to quit){}", NEW_LINE).unwrap();
    write!(std_out, "{} CLI{}", cli_char, NEW_LINE).unwrap();
    write!(std_out, "{} GUI{}", gui_char, NEW_LINE).unwrap();
}

pub fn make_ui_choice() -> bool {
    let std_in = stdin();
    let mut std_out = stdout().into_raw_mode().unwrap();
    write!(std_out, "{}", termion::cursor::Hide).unwrap();

    let mut launch_gui = false;

    draw_ui_choice(launch_gui);

    for c in std_in.keys() {
        match c.unwrap() {
            Key::Char('q') => {
                write!(std_out, "Quitting!{}", NEW_LINE).unwrap();
                write!(std_out, "{}", termion::cursor::Show).unwrap();
                process::exit(0);
            },          
            Key::Up => {
                launch_gui = false;
            },
            Key::Down => {
                launch_gui = true;
            },
            Key::Char(' ') => {
                break;
            },
            _ => {
                // Nothing
            }
        }

        draw_ui_choice(launch_gui);
    }

    return launch_gui;
}

pub fn run_game_cli() {
    let mut std_out = stdout().into_raw_mode().unwrap();
    write!(std_out, "{}", termion::cursor::Hide).unwrap();

    let std_in = stdin();
    let level = levels::let_user_select_level(&mut std_out);

    // Read board data
    let path = match level {
        Ok(level) => format!("boards/{}", level),
        Err(message) => panic!("Invalid level selected: {}", message),
    };

    let raw_data = match fs::read_to_string(path) {
        Ok(data) => data,
        Err(error) => panic!("{}", error),
    };

    let board: Vec<Vec<char>> = match board::parse_board_string(&mut std_out, raw_data) {
        Ok(data) => data,
        Err(error) => panic!("{}", error),
    };

    let mut position = player::Position {
        x: 0,
        y: 0
    };    

    board::draw_board(&mut std_out, &board, &position);

    let mut direction: player::Direction;

    for c in std_in.keys() {
        write!(std_out, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();

        let mut move_text: &str = "";

        match c.unwrap() {
            Key::Char('q') => {
                write!(std_out, "Quitting!{}", NEW_LINE).unwrap();
                write!(std_out, "{}", termion::cursor::Show).unwrap();
                process::exit(0);
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
        board::draw_board(&mut std_out, &board, &position);
        write!(std_out, "{} {}", move_text, NEW_LINE).unwrap();

        if board[position.y][position.x] == board::FINISH_TILE
            || board[position.y][position.x] == board::ALTERNATE_FINISH_TILE {
            write!(std_out, "\nSuccess! Congratulations!\n").unwrap();
            break;
        }
    }

    write!(std_out, "{}", termion::cursor::Show).unwrap();
}