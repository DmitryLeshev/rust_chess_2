#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl Default for Color {
    fn default() -> Self {
        Color::White
    }
}
