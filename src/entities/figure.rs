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
}

impl Figure {
    pub fn new(name: FigureName) -> Self {
        Self { name }
    }
}
