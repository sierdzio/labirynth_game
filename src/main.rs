pub mod player;
pub mod board;
pub mod levels;
pub mod cli;

fn main() {
    let launch_gui = cli::make_ui_choice();

    if launch_gui {
        MainWindow::new().unwrap().run().unwrap();
    } else {
        cli::run_game_cli();
    }
}

slint::slint! {
    struct TileData {
        character: string,
        textColor: color
    }

    component Tile inherits Rectangle {
        in property <string> character;
        in property <color> textColor: #000000;
        in property <image> icon;

        width: 24px;
        height: 24px;
        background: #12aa12;
    
        Text {
            text: parent.character;
            color: parent.textColor;
        }

        Image {
            source: parent.icon;
        }
    }

    export component MainWindow inherits Window {
        width: 300px;
        height: 300px;

        in property <[TileData]> tiles: [
            { character: "0" },
            { character: "1" },
            { character: "2" },
            { character: "3" },
            { character: "4" },
            { character: "5" },
            { character: "6" },
            { character: "7" },
            { character: "8" },
            { character: "9" },
            { character: "10" },
        ];

        in property <int> columnCount: 4;
        in property <int> rowCount: 3;

        VerticalLayout {
            spacing: 1px;

            for row in rowCount: HorizontalLayout {
                spacing: 1px;
                
                for column in columnCount: Tile {
                    preferred-height: 15px;
                    preferred-width: 15px;

                    character: tiles[(row * columnCount) + column].character;
                }
            }
        }
    }
}