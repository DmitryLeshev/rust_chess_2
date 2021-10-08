use crate::{
    constants::board::{HORIZONTAL, VERTICAL},
    entities::cell::Cell,
};

use super::position::Position;

#[derive(Debug, Clone)]
pub struct Board {
    cells: [[Cell; 8]; 8],
}

impl Board {
    pub fn default() {
        // let mut cells;
        let mut h: [Cell; 8];
        for (idx_x, v) in VERTICAL.iter().enumerate() {
            println!("{:?}", h[idx_x]);
            // for (idx_y, char) in HORIZONTAL.iter().enumerate() {
            //     let cell = Cell {
            //         position: Position::new(idx_x as u8, idx_y as u8),
            //         figure: None,
            //     };
            //     println!("{:x?}", cell);
            //     h[1]
            // }
            // cells.push(hor);
        }
        // println!("{}", cells)
        // Self { cells }
    }
}
