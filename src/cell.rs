use crate::drawable::*;
use crate::occupant::*;

pub struct Cell {
    occupant: Option<Occupant>,
}

impl Default for Cell {
    fn default() -> Cell {
        let occupant = None;
        Cell { occupant }
    }
}

impl Drawable for Cell {
    fn draw(&self) -> &str {
        match &self.occupant {
            Some(occupant) => occupant.draw(),
            None => "X",
        }
    }
}
