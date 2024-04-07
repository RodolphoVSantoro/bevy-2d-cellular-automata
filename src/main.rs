use bevy::prelude::{App, DefaultPlugins, Startup, Timer, TimerMode, Update};

mod config;
use config::TICK_TIME;

mod types;
use types::FrameTimer;

mod setup;
use setup::{create_board, setup};

mod ticks_processor;
use ticks_processor::process_tick;

mod rules;

fn main() {
    let board = create_board();
    App::new()
        .insert_resource(board)
        .insert_resource(FrameTimer(Timer::from_seconds(
            TICK_TIME,
            TimerMode::Repeating,
        )))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, process_tick)
        .run();
}
