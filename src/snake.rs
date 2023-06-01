use std::time::Duration;

use crate::{
    cell::Cell, drawable::Drawable, frame::Frame, position::Position, timer::Timer, vector::Vector,
};

pub struct Snake {
    timer: Timer,
    pub position: Position,
    pub direction: Vector,
    segments: Vec<Position>,
}

impl Snake {
    pub fn new(position: Position, direction: Vector, length: usize) -> Snake {
        let mut segments = Vec::new();

        for i in 1..(length + 1) {
            let position = Position {
                x: position.x + (i as i32 * -direction.x),
                y: position.y + (i as i32 * -direction.y),
            };

            segments.push(position);
        }

        Snake {
            timer: Timer::new(100),
            position,
            direction,
            segments,
        }
    }

    pub fn collision(&self, position: Position) -> bool {
        if self.segments.iter().any(|&segment| segment == position) {
            return true;
        }

        false
    }

    pub fn grow(&mut self, length: usize) {
        let position = *self.segments.last().unwrap();

        for _i in 1..(length + 1) {
            self.segments.push(position);
        }
    }
}

impl Drawable for Snake {
    fn draw(&self, frame: &mut Frame) {
        let index = frame.position_to_index(self.position) as usize;

        frame.cells[index] = Cell::Snake;

        for segment in self.segments.iter() {
            let index = frame.position_to_index(*segment) as usize;

            frame.cells[index] = Cell::Segment;
        }
    }

    fn tick(&mut self, delta: Duration) -> bool {
        self.timer.update(delta);

        if self.timer.ready {
            let mut position = self.position;
            let new_position = Position {
                x: self.position.x + self.direction.x,
                y: self.position.y + self.direction.y,
            };

            if self.collision(new_position) {
                return false;
            }

            self.position.x = new_position.x;
            self.position.y = new_position.y;

            for segment in self.segments.iter_mut() {
                let old_position = *segment;

                segment.x = position.x;
                segment.y = position.y;

                position = old_position;
            }

            self.timer.reset();
        }

        true
    }
}
