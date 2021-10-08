use super::color::Color;

#[derive(Debug, Clone)]
pub struct Player {
    color: Color,
}

impl Player {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}
