use std::time::Duration;

use crate::frame::Frame;

pub trait Drawable {
    fn tick(&mut self, delta: Duration) -> bool;

    fn draw(&self, frame: &mut Frame);
}
