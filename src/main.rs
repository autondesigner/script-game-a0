#![allow(warnings)]
mod address;
mod address_book;
mod camera;
mod cause;
mod cell;
mod centered_camera;
mod direction;
mod drawable;
mod flow;
mod game;
mod occupant;
mod size;
mod space;
mod torus;
use crate::game::*;
use crossterm::Result;
fn main() -> Result<()> {
    Game::start()
}
