fn main() {
    let board = [
        ['.', '.', 'w', '.', '.'], 
        ['w', '.', 'w', 'w', 'w'], 
        ['w', '.', 'v', '.', 'w'], 
        ['.', '.', 'w', '.', 'w'], 
        ['w', 'w', 'w', '.', 'e'], 
        ];

    loop {
        println!("Use arrow keys to go through the maze.");

        for row in board.iter() {
            for character in row.iter() {
                print!("{character}");
            }
            println!();
        }

        break;
    }
}
