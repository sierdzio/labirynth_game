use std::fs;

use std::io::{Write, Stdout, stdin};
use termion::raw::RawTerminal;
use termion::input::TermRead;
use termion::event::Key;

pub fn get_available_levels() -> Result<Vec<String>, &'static str> {
    let mut result: Vec<String> = vec![];

    if let Ok(entries) = fs::read_dir("boards/") {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();

                let file_name = match path.file_name() {
                    None => continue,
                    Some(name) => name,
                };

                result.push(match file_name.to_str() {
                    None => return Err("Invalid file name! {file_name}"),
                    Some(name) => String::from(name),
                });
            }
        }
    } else {
        return Err("Could not read from'boards' directory!");
    }

    return Ok(result);
}

pub fn draw_available_levels(stdout: &mut RawTerminal<Stdout>, levels: &Vec<String>, current_index: i32) {
    if levels.len() > 0 {
        write!(stdout, "Select a level with SPACE or quit with 'q'\r\n").unwrap();
        write!(stdout, "Available levels are:\r\n").unwrap();

        let mut index = 0;
        for level in levels {
            if index == current_index {
                write!(stdout, " * {}\r\n", level).unwrap();
            } else {
                write!(stdout, " - {}\r\n", level).unwrap();
            }

            index = index + 1;
        }
    }
}

pub fn let_user_select_level(stdout: &mut RawTerminal<Stdout>) -> Result<String, &'static str> {
    let stdin = stdin();
    let levels = get_available_levels().unwrap();
    let level_count: usize = get_available_levels().unwrap().len();
    let mut current_index: usize = 0;

    write!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
    draw_available_levels(stdout, &levels, current_index as i32);

    for c in stdin.keys() {
        write!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();

        match c.unwrap() {
            Key::Char('q') => {
                write!(stdout, "Quitting!\r\n").unwrap();
                return Err("No selection!");
            },
            Key::Up => {
                if current_index > 0 {
                    current_index = current_index - 1;
                }
            },
            Key::Down => {
                if current_index < level_count - 1 {
                    current_index = current_index + 1;
                }
            },
            Key::Char(' ') => {
                return Ok(levels.get(current_index).unwrap().clone());
            },
            Key::Char('\r') => {
                return Ok(levels.get(current_index).unwrap().clone());
            },
            _ => {
                continue;
            }
        };

        draw_available_levels(stdout, &levels, current_index as i32);
    }

    return Err("No selection");
}