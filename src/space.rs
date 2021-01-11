use crate::camera::*;
use crate::cell::*;
use crate::drawable::*;
use crate::torus::*;
use std::io::*;

pub struct Space {
    height: usize,
    width: usize,
    torus: Torus<Cell>,
}

impl Default for Space {
    fn default() -> Space {
        let height = 32;
        let width = 64;
        let torus = Torus::new(width, height);
        Space {
            height,
            width,
            torus,
        }
    }
}

impl Space {
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn draw(&self, row: usize, column: usize) -> &str {
        self.torus.cells[self.torus.index(row, column)].draw()
    }
}
