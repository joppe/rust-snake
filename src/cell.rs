use crossterm::style::{StyledContent, Stylize};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Cell {
    Unset,
    Empty,
    Snake,
    Segment,
    Wall,
    Candy,
}

impl Cell {
    pub fn to_char(&self) -> StyledContent<String> {
        match self {
            Cell::Unset => "\u{2588}".to_string().black(),
            Cell::Empty => "\u{2588}".to_string().black(),
            Cell::Snake => "\u{2588}".to_string().blue(),
            Cell::Segment => "\u{2588}".to_string().dark_blue(),
            Cell::Wall => "\u{2588}".to_string().white(),
            Cell::Candy => "\u{2588}".to_string().red(),
        }
    }
}
