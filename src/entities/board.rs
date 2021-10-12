use std::collections::HashMap;

use crate::{
    constants::board::{HORIZONTAL, VERTICAL},
    entities::cell::Cell,
};

use super::{
    figure::{Figure, FigureName},
    position::Position,
};

#[derive(Debug, Clone)]
pub struct Board {
    pub grid: HashMap<String, Cell>,
}

impl Board {
    pub fn load() {}
    pub fn new() -> Self {
        let mut grid = HashMap::new();
        for (coll, v) in VERTICAL.iter().enumerate() {
            for (row, char) in HORIZONTAL.iter().enumerate() {
                let position = Position::new(coll as u8 + 1, row as u8 + 1);
                if let Some(grid_name) = Cell::get_name(&position) {
                    let name = Figure::get_default_name(&position);
                    let figure = match name {
                        Some(name) => {
                            if let Some(color) = Figure::get_default_color(&position) {
                                Some(Figure::new(name, color))
                            } else {
                                None
                            }
                        }
                        None => None,
                    };
                    let cell = Cell { position, figure };
                    grid.insert(grid_name, cell);
                }
            }
        }
        Self { grid }
    }
}
