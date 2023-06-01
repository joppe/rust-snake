use std::time::Duration;

use crate::{cell::Cell, drawable::Drawable, frame::Frame, position::Position};

pub struct Candy {
    position: Option<Position>,
}

impl Candy {
    pub fn new() -> Candy {
        Candy {
            position: Option::None,
        }
    }

    pub fn reset(&mut self) {
        self.position = None;
    }

    pub fn set_position(&mut self, position: Position) {
        self.position = Some(position);
    }

    pub fn eat(&self, head: Position) -> bool {
        match self.position {
            Some(position) => position == head,
            None => false,
        }
    }
}

impl Drawable for Candy {
    fn draw(&self, frame: &mut Frame) {
        match self.position {
            Some(position) => {
                let index = frame.position_to_index(position);

                frame.cells[index as usize] = Cell::Candy;
            }
            None => {}
        }
    }

    fn tick(&mut self, _delta: Duration) -> bool {
        match self.position {
            Some(_position) => true,
            None => false,
        }
    }
}
