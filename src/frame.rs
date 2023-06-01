use rand::Rng;

use crate::{cell::Cell, position::Position, size::Size};

pub struct Frame {
    size: Size,
    pub cells: Vec<Cell>,
}

impl Frame {
    pub fn new(size: Size, cell: Cell) -> Frame {
        let mut cells = Vec::new();

        for y in 0..size.height {
            for x in 0..size.width {
                if cell == Cell::Unset {
                    cells.push(cell);
                } else if y == 0 || y == size.height - 1 || x == 0 || x == size.width - 1 {
                    cells.push(Cell::Wall);
                } else {
                    cells.push(cell);
                }
            }
        }

        Frame { size, cells }
    }

    pub fn index_to_position(&self, index: i32) -> Position {
        let x = index % self.size.width;
        let y = index / self.size.width;

        Position { x, y }
    }

    pub fn position_to_index(&self, position: Position) -> i32 {
        position.x + (position.y * self.size.width)
    }

    pub fn is_free_position(&self, position: Position) -> bool {
        let index = self.position_to_index(position);

        self.cells[index as usize] == Cell::Empty
    }

    pub fn random_position(&self) -> Position {
        let mut rng = rand::thread_rng();

        Position {
            x: rng.gen_range(2..(self.size.width - 2)),
            y: rng.gen_range(2..(self.size.height - 2)),
        }
    }
}
