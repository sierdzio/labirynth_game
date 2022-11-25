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

    loop {
        println!("Use arrow keys to go through the maze. Type 'q' to quit");
        println!("w - wall, dot (.) - path, d - doors, ☑ - exit");
        println!("");

        for (row_index, row) in board.iter().enumerate() {
            for (column_index, character) in row.iter().enumerate() {
                if position == [row_index, column_index] {
                    print!("🨄");
                } else {
                    print!("{character}");
                }
            }
            println!();
        }

        let mut move_str = String::new();

        std::io::stdin()
            .read_line(&mut move_str)
            .expect("Failed to read command");

        println!("Command is: {move_str}");

        if move_str == "q" {
            break;
        } else if  move_str == "\033[D" {
            println!("Moving left");
            position = [position[0] - 1, position[1]];
        } else if  move_str == "\033[C" {
            println!("Moving right");
            position = [position[0] + 1, position[1]];
        } else if  move_str == "\033[A" {
            println!("Moving up");
            position = [position[0], position[1] - 1];
        } else if  move_str == "\033[B" {
            println!("Moving down");
            position = [position[0], position[1] + 1];
        }
    }
}
