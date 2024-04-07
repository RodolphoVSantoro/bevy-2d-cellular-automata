use bevy::prelude::Color;

pub const MAX_WIDTH: usize = 100; // how many cells wide the board is
pub const MAX_HEIGHT: usize = 60; // how many cells tall the board is

pub const TICK_TIME: f32 = 0.1; // how many seconds until next tick is processed

pub const DECAY_TICKS: u32 = 10; // how many ticks a cell will decay for

pub const DEAD_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const ALIVE_COLOR: Color = Color::rgb(0.75, 0.75, 0.75);
