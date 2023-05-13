pub mod player;
pub mod board;
pub mod levels;
pub mod cli;
pub mod gui;

fn main() {
    let mut launch_gui = true;
    let mut launcher = false;
    for arg in std::env::args() {
        if arg == "--cli" {
            launch_gui = false;
        } else if arg == "--choose" {
            launcher = true;
        }
    }

    if launcher {
        launch_gui = cli::make_ui_choice();
    }

    if launch_gui {
        gui::run_game_gui();
    } else {
        cli::run_game_cli();
    }
}