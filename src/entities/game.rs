use super::{board::Board, color::Color, player::Player};

#[derive(Debug, Clone)]
pub struct Game {
    players: [Player; 2],
}

impl Game {
    pub fn new() -> Self {
        let player_1 = Player::new(Color::White);
        let player_2 = Player::new(Color::Black);
        let _ = Board::default();

        Self {
            players: [player_1, player_2],
        }
    }
}
