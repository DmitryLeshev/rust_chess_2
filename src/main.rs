mod constants;
mod controllers;
mod entities;
mod view;

use crate::{constants::messages::WELCOME_MESSAGE, view::GameView};

fn main() {
    let view = GameView::new();
    view.run();
}
