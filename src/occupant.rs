use crate::drawable::*;

pub enum Class {
    Player,
    Enemy,
    Wall,
}

pub struct Occupant {
    class: Class,
}

impl Drawable for Occupant {
    fn draw(&self) -> &str {
        match self.class {
            Class::Enemy => "E",
            Class::Player => "P",
            Class::Wall => "W",
        }
    }
}
