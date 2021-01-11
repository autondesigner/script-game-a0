use crate::address::*;
use crate::camera::*;
use crate::direction::*;
use crate::size::*;
use crate::torus::*;

pub struct CenteredCamera {
    address: Address,
    size: Size,
}

impl Default for CenteredCamera {
    fn default() -> CenteredCamera {
        let address = Address::default();
        let size = Size::new(16, 64);
        CenteredCamera { address, size }
    }
}

impl Camera for CenteredCamera {
    fn position(&self, height: usize, width: usize) -> Address {
        let mut address = self.address;
        for _i in 0..self.size.height {
            address = address.find_neighbor(Direction::Up, height, width);
        }
        for _i in 0..self.size.width {
            address = address.find_neighbor(Direction::Left, height, width);
        }
        address
    }
    fn width(&self) -> usize {
        self.size.width * 2 + 1
    }
    fn height(&self) -> usize {
        self.size.height * 2 + 1
    }
}
