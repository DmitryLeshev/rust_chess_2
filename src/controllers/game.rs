use crate::constants::board::{HORIZONTAL, VERTICAL};
use crate::entities::cell::Cell;
use crate::entities::color::Color;
use crate::entities::figure::{Figure, FigureName};
use crate::entities::game::Game;

#[derive(Debug, Clone)]
pub enum PointDtoType {
    Frame,
    Player,
    Timer,
    Figure(Figure),
    Cell,
}

impl Default for PointDtoType {
    fn default() -> PointDtoType {
        PointDtoType::Cell
    }
}

#[derive(Debug, Clone)]
pub struct PointDto {
    pub state_type: PointDtoType,
}

#[derive(Debug, Clone)]
pub struct GameController {
    pub game: Game,
}

impl GameController {
    pub fn new() -> Self {
        let game = Game::new();
        Self { game }
    }

    pub fn get_state(&self) -> Vec<Vec<&Cell>> {
        let mut state = Vec::new();

        for idx in VERTICAL.iter() {
            let mut v = Vec::new();
            for char in HORIZONTAL.iter() {
                let key = format!("{}{}", char, 9 - idx);
                if let Some(cell) = self.game.board.grid.get(&key) {
                    v.push(cell)
                }
            }
            state.push(v);
        }
        state
    }

    pub fn move_figure(mut self, selected_figure: String, selected_cell: String) -> Self {
        if Cell::is_correct_name(&selected_figure) && Cell::is_correct_name(&selected_cell) {
            self.game.move_figure(selected_figure, selected_cell);
            self
        } else {
            self
        }
    }

    pub fn update(mut self) -> Self {
        let game = self.game.update();
        self.game = game;
        self
    }
}
