use chrono::Duration;

use super::{board::Board, cell::Cell, color::Color, figure::Figure, player::Player};

#[derive(Debug, Clone)]
pub struct Game {
    pub players: [Player; 2],
    pub board: Board,
    pub timers: [Duration; 2],
    pub selected_player: Player,
    pub selected_cell: Option<Cell>,
    pub moves: Vec<String>,
}

impl Game {
    pub fn new() -> Self {
        let player_1 = Player::new(Color::White);
        let player_2 = Player::new(Color::Black);
        let board = Board::new();
        let timer = Duration::minutes(3);
        Self {
            players: [player_1.clone(), player_2],
            board,
            timers: [timer, timer],
            selected_player: player_1,
            selected_cell: None,
            moves: Vec::new(),
        }
    }

    pub fn select_player(&mut self, player: &Player) {
        self.selected_player = player.clone();
    }

    pub fn move_figure(&mut self, selected_figure: String, selected_cell: String) {
        let selected_f = self.board.grid.get_mut(&selected_figure);
        if let Some(selected_f) = selected_f {
            let f = selected_f.figure.clone();
            selected_f.figure = None;
            let selected_cell = self.board.grid.get_mut(&selected_cell);
            if let Some(selected_cell) = selected_cell {
                selected_cell.figure = f;
            }
        } else {
            println!("Выбранная фигура не найдена")
        }
    }

    pub fn update(self) -> Self {
        self
    }
}
