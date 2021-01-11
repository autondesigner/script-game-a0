#[derive(Default)]
pub struct Size {
    pub height: usize,
    pub width: usize,
}

impl Size {
    pub fn new(height: usize, width: usize) -> Size {
        Size { height, width }
    }
}
