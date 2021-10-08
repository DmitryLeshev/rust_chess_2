use crate::constants::messages::SIDE_BORDER;
use crate::constants::messages::SOLID_LINE;
use crate::controllers::game::{GameController, PointDto, PointDtoType};
use crate::entities::figure::FigureName;
use crate::WELCOME_MESSAGE;

use chrono::Local;
use core::time;
use std::thread;

#[derive(Debug, Clone)]
pub struct GameView {
    controller: GameController,
    pub counter: i8,
}

impl GameView {
    pub fn new() -> Self {
        let controller = GameController::new();
        Self {
            controller,
            counter: 0,
        }
    }

    fn get_meshes(&self) -> [[PointDto; 8]; 8] {
        let meshes = self.controller.clone().get_state();
        meshes
    }

    fn update(mut self) -> Self {
        let controller = self.controller.update();
        self.controller = controller;
        Self { ..self }
    }

    fn draw(self) -> Self {
        // let meshes = self.get_meshes();

        // std::process::Command::new("clear").status().unwrap();
        // let local_time = Local::now();
        // let dt = local_time.format("%Y-%m-%d %H:%M:%S").to_string();
        // println!("\n{}\n", SOLID_LINE);

        // println!(
        //     "{}  {}  {}  {}",
        //     SIDE_BORDER, WELCOME_MESSAGE, dt, SIDE_BORDER
        // );
        // println!("\n{}\n", SOLID_LINE);

        // println!(
        //     "{}                                      {}",
        //     SIDE_BORDER, SIDE_BORDER
        // );

        // for (idx_x, points) in meshes.iter().enumerate() {
        //     let mut hor_line = String::new();
        //     for p in points {
        //         let s = match p.state_type.clone() {
        //             PointDtoType::Figure(f) => {
        //                 let n = match f.name {
        //                     FigureName::Rook => "|Rk|",
        //                     FigureName::Knight => "|Kn|",
        //                     FigureName::Bishop => "|Bs|",
        //                     FigureName::Queen => "|Qn|",
        //                     FigureName::King => "|Kg|",
        //                     FigureName::Pawn => "|Pw|",
        //                 };
        //                 n
        //             }
        //             PointDtoType::Cell => "|  |",
        //         };
        //         hor_line.push_str(s);
        //     }
        //     println!(
        //         "{}  {} {}  {}",
        //         SIDE_BORDER,
        //         8 - idx_x,
        //         hor_line,
        //         SIDE_BORDER
        //     );
        // }

        // println!(
        //     "{}     a   b   c   d   e   f   g   h    {}",
        //     SIDE_BORDER, SIDE_BORDER
        // );
        // println!(
        //     "{}                                      {}",
        //     SIDE_BORDER, SIDE_BORDER
        // );
        // println!("\n{}\n", SOLID_LINE);
        Self { ..self }
    }

    // fn get_input(&self) -> Option<Direction> {
    //     match self.window.input.keys_hit().last() {
    //         None => None,
    //         Some(k) =>
    //             match *k {
    //                 three::Key::Left => Some(Direction::Left),
    //                 three::Key::Right => Some(Direction::Right),
    //                 three::Key::Down => Some(Direction::Top),
    //                 three::Key::Up => Some(Direction::Bottom),
    //                 _ => None,
    //             }
    //     }
    // }

    pub fn run(mut self) {
        loop {
            let ten_millis = time::Duration::from_millis(1000);
            thread::sleep(ten_millis);
            self = self.update().draw();
        }
    }
}
