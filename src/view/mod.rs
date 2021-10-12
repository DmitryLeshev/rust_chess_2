use crate::constants::figure::BISHOP_BLACK;
use crate::constants::figure::BISHOP_WHITE;
use crate::constants::figure::KING_BLACK;
use crate::constants::figure::KING_WHITE;
use crate::constants::figure::KNIGHT_BLACK;
use crate::constants::figure::KNIGHT_WHITE;
use crate::constants::figure::PAWN_BLACK;
use crate::constants::figure::PAWN_WHITE;
use crate::constants::figure::QUEEN_BLACK;
use crate::constants::figure::QUEEN_WHITE;
use crate::constants::figure::ROOK_BLACK;
use crate::constants::figure::ROOK_WHITE;
use crate::constants::messages::SIDE_BORDER;
use crate::constants::messages::SOLID_LINE;
use crate::controllers::game::GameController;
use crate::entities::cell::Cell;
use crate::entities::color::Color;
use crate::entities::figure::FigureName;
use crate::WELCOME_MESSAGE;

use chrono::Local;
use core::time;
use std::io;
use std::thread;

#[derive(Debug, Clone)]
pub struct GameView {
    controller: GameController,
}

impl GameView {
    pub fn new() -> Self {
        let controller = GameController::new();
        Self { controller }
    }

    fn get_meshes(&self) -> Vec<Vec<&Cell>> {
        let state = self.controller.get_state();
        state
    }

    fn update(mut self) -> Self {
        let controller = self.controller.update();
        self.controller = controller;
        Self { ..self }
    }

    fn draw(self) -> Self {
        let meshes = self.get_meshes();

        std::process::Command::new("clear").status().unwrap();
        let local_time = Local::now();
        let dt = local_time.format("%Y-%m-%d %H:%M:%S").to_string();
        println!("\n{}\n", SOLID_LINE);

        println!(
            "{}  {}  {}  {}",
            SIDE_BORDER, WELCOME_MESSAGE, dt, SIDE_BORDER
        );
        println!("\n{}\n", SOLID_LINE);

        println!(
            "{}                                      {}",
            SIDE_BORDER, SIDE_BORDER
        );

        for (idx_x, cells) in meshes.iter().enumerate() {
            let mut hor_line = String::new();
            for cell in cells {
                let s = match &cell.figure {
                    Some(f) => match f.name {
                        FigureName::Rook => {
                            let figure = if f.color == Color::White {
                                ROOK_WHITE
                            } else {
                                ROOK_BLACK
                            };
                            format!("{}", figure)
                        }
                        FigureName::Knight => {
                            let figure = if f.color == Color::White {
                                KNIGHT_WHITE
                            } else {
                                KNIGHT_BLACK
                            };
                            format!("{}", figure)
                        }
                        FigureName::Bishop => {
                            let figure = if f.color == Color::White {
                                BISHOP_WHITE
                            } else {
                                BISHOP_BLACK
                            };
                            format!("{}", figure)
                        }
                        FigureName::Queen => {
                            let figure = if f.color == Color::White {
                                QUEEN_WHITE
                            } else {
                                QUEEN_BLACK
                            };
                            format!("{}", figure)
                        }
                        FigureName::King => {
                            let figure = if f.color == Color::White {
                                KING_WHITE
                            } else {
                                KING_BLACK
                            };
                            format!("{}", figure)
                        }
                        FigureName::Pawn => {
                            let figure = if f.color == Color::White {
                                PAWN_WHITE
                            } else {
                                PAWN_BLACK
                            };
                            format!("{}", figure)
                        }
                    },
                    None => " ".to_string(),
                };
                let s = format!("|{}|", s);
                hor_line.push_str(&s);
            }
            println!(
                "{}      {} {}      {}",
                SIDE_BORDER,
                8 - idx_x,
                hor_line,
                SIDE_BORDER
            );
        }

        println!(
            "{}         a  b  c  d  e  f  g  h       {}",
            SIDE_BORDER, SIDE_BORDER
        );
        println!(
            "{}                                      {}",
            SIDE_BORDER, SIDE_BORDER
        );
        println!("\n{}\n", SOLID_LINE);
        Self { ..self }
    }

    fn get_input(&mut self) -> io::Result<(String, String)> {
        let mut selected_figure = String::new();
        let mut selected_cell = String::new();
        let mut result = String::new();
        println!("Выбери фигуру (e2)");

        io::stdin().read_line(&mut selected_figure)?;
        println!("Выбери клетку куда пойти (e4)");
        io::stdin().read_line(&mut selected_cell)?;

        let selected_figure = selected_figure.trim().to_string();
        let selected_cell = selected_cell.trim().to_string();

        println!("Ваш ход: {} -> {}", selected_figure, selected_cell);
        println!("Вы уверены? [y/n]");
        io::stdin().read_line(&mut result)?;

        Ok((selected_figure, selected_cell))
    }

    pub fn run(mut self) {
        loop {
            let ten_millis = time::Duration::from_millis(1000);
            thread::sleep(ten_millis);
            self = self.update().draw();
            let res = self.get_input().unwrap();
            self.controller = self.controller.move_figure(res.0, res.1);
        }
    }
}
