use super::{figure::Figure, position::Position};

#[derive(Debug, Clone)]
pub struct Cell {
    pub position: Position,
    pub figure: Option<Figure>,
}

impl Cell {
    pub fn new(x: u8, y: u8, figure: Option<Figure>) -> Self {
        let position = Position::new(x, y);
        Self { position, figure }
    }
    pub fn get_name(position: &Position) -> Option<String> {
        let Position { x, y } = position;
        if x > &0 && x < &9 && y > &0 && y < &9 {
            let mut name = String::new();
            let letter = match y {
                1 => 'a',
                2 => 'b',
                3 => 'c',
                4 => 'd',
                5 => 'e',
                6 => 'f',
                7 => 'g',
                _ => 'h',
            };
            name.push_str(&letter.to_string());
            name.push_str(&x.to_string());
            Some(name)
        } else {
            None
        }
    }
    pub fn is_correct_name(name: &String) -> bool {
        let name = name.clone();
        // let first_char = name.chars().next().unwrap();
        // let second_char = name.chars().next().unwrap();

        // println!("{} {}", first_char, second_char);

        true
    }
}
