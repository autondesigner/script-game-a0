use crate::address::*;
use crate::torus::*;

pub trait Camera {
    fn position(&self, space_height: usize, space_width: usize) -> Address;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}
