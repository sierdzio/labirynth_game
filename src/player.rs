use crate::board;

pub struct Position {
    pub x: usize,
    pub y: usize,
} 

pub enum Direction {
    Unknown,
    Left,
    Right,
    Up,
    Down
}

pub fn try_to_move(board: &[[char; 5]; 6], position: &mut Position, direction: Direction) {
    match direction {
        Direction::Unknown => {
        },
        Direction::Left => {
            let new_pos: usize;
            
            if position.x > 0 {
                new_pos = position.x - 1;
            } else {
                new_pos = position.x;
            }

            if board[position.y][new_pos] != board::WALL {
                position.x = new_pos;
            }
        },        
        Direction::Right => {
            let new_pos: usize;
            
            if position.x < 4 {
                new_pos = position.x + 1;
            } else {
                new_pos = position.x;
            }

            if board[position.y][new_pos] != board::WALL {
                position.x = new_pos;
            }
        },
        Direction::Up => {
            let new_pos: usize;
            
            if position.y > 0 {
                new_pos = position.y - 1;
            } else {
                new_pos = position.y;
            }

            if board[new_pos][position.x] != board::WALL {
                position.y = new_pos;
            }
        },
        Direction::Down => {
            let new_pos: usize;
            
            if position.y < 5 {
                new_pos = position.y + 1;
            } else {
                new_pos = position.y;
            }

            if board[new_pos][position.x] != board::WALL {
                position.y = new_pos;
            }
        }
    }
}