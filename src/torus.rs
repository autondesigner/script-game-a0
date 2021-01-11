use crate::address::*;
use crate::address_book::*;
use crate::direction::*;

pub struct Torus<T: Default> {
    pub height: usize,
    pub width: usize,
    pub cells: Vec<T>,
}

impl<T: Default> Torus<T> {
    pub fn new(height: usize, width: usize) -> Torus<T> {
        let cells = Torus::<T>::build_cells(width, height);
        Torus {
            width,
            height,
            cells,
        }
    }
    pub fn build_cells(height: usize, width: usize) -> Vec<T> {
        let mut cells = Vec::with_capacity(width * height);
        for _i in 0..width * height {
            cells.push(Default::default());
        }
        cells
    }
    pub fn index(&self, row: usize, column: usize) -> usize {
        row * self.width + column
    }
    pub fn address_index(&self, address: Address) -> usize {
        address.row * self.width + address.column
    }
}
