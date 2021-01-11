use crate::direction::*;
#[derive(Copy, Clone, Default)]
pub struct Address {
    pub row: usize,
    pub column: usize,
}

impl Address {
    pub fn new(row: usize, column: usize) -> Address {
        Address { row, column }
    }
    pub fn find_neighbor(&self, direction: Direction, height: usize, width: usize) -> Address {
        match direction {
            Direction::Down => {
                if self.row == height - 1 {
                    return Address::new(0, self.column);
                }
                return Address::new(self.row + 1, self.column);
            }
            Direction::Up => {
                if self.row == 0 {
                    return Address::new(height - 1, self.column);
                }
                return Address::new(self.row - 1, self.column);
            }
            Direction::Right => {
                if self.column == width - 1 {
                    return Address::new(self.row, 0);
                }
                return Address::new(self.row, self.column + 1);
            }
            Direction::Left => {
                if self.column == 0 {
                    return Address::new(self.row, width - 1);
                }
                return Address::new(self.row, self.column - 1);
            }
        }
    }
    pub fn get_usize_radius(&self, address: Address) -> usize {
        let x;
        if self.column > address.column {
            x = self.column - address.column;
        } else {
            x = address.column - self.column;
        }
        let y;
        if self.row > address.row {
            y = self.row - address.row;
        } else {
            y = address.row - self.row;
        }
        x + y
    }
    pub fn get_radius(&self, address: Address) -> f64 {
        let x;
        if self.column > address.column {
            x = self.column - address.column;
        } else {
            x = address.column - self.column;
        }
        let y;
        if self.row > address.row {
            y = self.row - address.row;
        } else {
            y = address.row - self.row;
        }
        ((x.pow(2) + y.pow(2)) as f64).sqrt()
    }
}
