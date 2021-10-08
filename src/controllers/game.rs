use crate::entities::figure::{Figure, FigureName};
use crate::entities::game::Game;

#[derive(Debug, Clone)]
pub enum PointDtoType {
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

    pub fn get_state(&self) -> [[PointDto; 8]; 8] {
        let cell = PointDto {
            state_type: PointDtoType::Cell,
        };
        let pawn = PointDto {
            state_type: PointDtoType::Figure(Figure::new(FigureName::Pawn)),
        };
        let rook = PointDto {
            state_type: PointDtoType::Figure(Figure::new(FigureName::Rook)),
        };
        let knight = PointDto {
            state_type: PointDtoType::Figure(Figure::new(FigureName::Knight)),
        };
        let bishop = PointDto {
            state_type: PointDtoType::Figure(Figure::new(FigureName::Bishop)),
        };
        let queen = PointDto {
            state_type: PointDtoType::Figure(Figure::new(FigureName::Queen)),
        };
        let king = PointDto {
            state_type: PointDtoType::Figure(Figure::new(FigureName::King)),
        };
        let state = [
            [
                rook.clone(),
                knight.clone(),
                bishop.clone(),
                queen.clone(),
                king.clone(),
                bishop.clone(),
                knight.clone(),
                rook.clone(),
            ],
            [
                pawn.clone(),
                pawn.clone(),
                pawn.clone(),
                pawn.clone(),
                pawn.clone(),
                pawn.clone(),
                pawn.clone(),
                pawn.clone(),
            ],
            [
                cell.clone(),
                cell.clone(),
                cell.clone(),
                cell.clone(),
                cell.clone(),
                cell.clone(),
                cell.clone(),
                cell.clone(),
            ],
            [
                cell.clone(),
                cell.clone(),
                cell.clone(),
                cell.clone(),
                cell.clone(),
                cell.clone(),
                cell.clone(),
                cell.clone(),
            ],
            [
                cell.clone(),
                cell.clone(),
                cell.clone(),
                cell.clone(),
                cell.clone(),
                cell.clone(),
                cell.clone(),
                cell.clone(),
            ],
            [
                cell.clone(),
                cell.clone(),
                cell.clone(),
                cell.clone(),
                cell.clone(),
                cell.clone(),
                cell.clone(),
                cell.clone(),
            ],
            [
                pawn.clone(),
                pawn.clone(),
                pawn.clone(),
                pawn.clone(),
                pawn.clone(),
                pawn.clone(),
                pawn.clone(),
                pawn.clone(),
            ],
            [
                rook.clone(),
                knight.clone(),
                bishop.clone(),
                queen.clone(),
                king.clone(),
                bishop.clone(),
                knight.clone(),
                rook.clone(),
            ],
        ];

        state
    }

    //Обновляем состояние игры
    pub fn update(mut self) -> Self {
        // let game = self.game.clone();
        // self.game  = match ХОД ИГРОКА ЗАКОНЧИЛСЯ {
        //     None => game,
        //     Some(d) => game.handle_input(d)
        // }
        self
    }
}
