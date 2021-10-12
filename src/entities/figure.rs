use super::{color::Color, position::Position};

#[derive(Debug, Clone)]
pub enum FigureName {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone)]
pub struct Figure {
    pub name: FigureName,
    pub color: Color,
}

impl Figure {
    pub fn new(name: FigureName, color: Color) -> Self {
        Self { name, color }
    }
    pub fn get_default_name(position: &Position) -> Option<FigureName> {
        let name = match (position.x, position.y) {
            (1 | 8, 1 | 8) => Some(FigureName::Rook),
            (1 | 8, 2 | 7) => Some(FigureName::Knight),
            (1 | 8, 3 | 6) => Some(FigureName::Bishop),
            (1 | 8, 5) => Some(FigureName::King),
            (1 | 8, 4) => Some(FigureName::Queen),
            (2 | 7, _) => Some(FigureName::Pawn),
            _ => None,
        };
        name
    }
    pub fn get_default_color(position: &Position) -> Option<Color> {
        let color = match position.x {
            1 | 2 => Some(Color::White),
            7 | 8 => Some(Color::Black),
            _ => None,
        };
        color
    }
}
