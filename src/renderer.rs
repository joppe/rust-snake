use std::io::{self, Write};

use crossterm::{cursor, style, QueueableCommand};

use crate::{cell::Cell, frame::Frame, size::Size};

pub struct Renderer {
    last_frame: Frame,
}

impl Renderer {
    pub fn new(size: Size) -> Renderer {
        Renderer {
            last_frame: Frame::new(size, Cell::Unset),
        }
    }

    pub fn render(&mut self, stdout: &mut io::Stdout, frame: Frame) {
        for (i, cell) in frame.cells.iter().enumerate() {
            if *cell != self.last_frame.cells[i] {
                let position = frame.index_to_position(i as i32);

                stdout
                    .queue(cursor::MoveTo(position.x as u16 * 2, position.y as u16))
                    .unwrap();
                stdout
                    .queue(style::PrintStyledContent(cell.to_char()))
                    .unwrap();
                stdout
                    .queue(cursor::MoveTo(
                        (position.x as u16 * 2) + 1,
                        position.y as u16,
                    ))
                    .unwrap();
                stdout
                    .queue(style::PrintStyledContent(cell.to_char()))
                    .unwrap();
            }
        }

        stdout.flush().unwrap();
        self.last_frame = frame;
    }
}
