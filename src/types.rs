use bevy::prelude::{Component, Resource, Timer};

pub struct Cell {
    pub decaying: bool,
    pub decaying_ticks: u32,
    pub dead: bool,
    pub neighbors: u32,
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            decaying: true,
            dead: true,
            decaying_ticks: 0,
            neighbors: 0,
        }
    }
}

#[derive(Resource)]
pub struct Board(pub Vec<Vec<Cell>>);

#[derive(Component, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Resource)]
pub struct FrameTimer(pub Timer);
