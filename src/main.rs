use std::cmp;

use bevy::prelude::*;

const MAX_WIDTH: usize = 100; // how many cells wide the board is
const MAX_HEIGHT: usize = 60; // how many cells tall the board is

const TICK_TIME: f32 = 0.1; // how many seconds until next tick is processed

const DECAY_TICKS: u32 = 10; // how many ticks a cell will decay for

const DEAD_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
const ALIVE_COLOR: Color = Color::rgb(0.75, 0.75, 0.75);

struct Cell {
    decaying: bool,
    decaying_ticks: u32,
    dead: bool,
    neighbors: u32,
}

#[derive(Component)]
struct Person;

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
struct Board(Vec<Vec<Cell>>);

/**
 * Returns a range that is limited to the positive range of \[0, max\]
 */
fn limited_positive_range(start: usize, end: usize, max: usize) -> std::ops::Range<usize> {
    return cmp::max(start, 0)..cmp::min(end, max);
}

fn count_neighbors(board: &mut ResMut<Board>) {
    for x in 0..MAX_WIDTH {
        for y in 0..MAX_HEIGHT {
            count_cell_neighbors(board, x, y);
        }
    }
}

fn count_cell_neighbors(board: &mut ResMut<Board>, x: usize, y: usize) {
    for adjacent_x in limited_positive_range(x - 1, x + 1, MAX_WIDTH) {
        for adjacent_y in limited_positive_range(y - 1, y + 1, MAX_HEIGHT) {
            if !board.0[adjacent_x][adjacent_y].dead {
                board.0[x][y].neighbors += 1;
            }
        }
    }
}

#[derive(Component, Debug)]
struct Position {
    x: usize,
    y: usize,
}

fn should_start_alive(x: usize, y: usize) -> bool {
    return (x - 20) * (x - 20) + (y - 20) * (y - 20) < 5 * 5 && x % 3 != 0;
}

fn should_stay_alive(neighbors: u32) -> bool {
    return neighbors == 2 || neighbors == 3;
}

fn should_spawn(neighbors: u32) -> bool {
    return neighbors > 1;
}

fn create_board() -> Board {
    let mut board: Board = Board(vec![]);
    for x in 0..MAX_WIDTH {
        let mut column: Vec<Cell> = vec![];
        for y in 0..MAX_HEIGHT {
            let mut cell = Cell {
                decaying: true,
                dead: true,
                decaying_ticks: 0,
                neighbors: 0,
            };
            if should_start_alive(x, y) {
                cell.decaying = false;
                cell.dead = false;
                cell.decaying_ticks = DECAY_TICKS;
            }
            column.push(cell);
        }
        board.0.push(column);
    }

    return board;
}

#[derive(Resource)]
struct PrintTimer(Timer);

#[derive(Resource)]
struct FrameTimer(Timer);

fn setup(mut commands: Commands) {
    //init camera
    commands.spawn(Camera2dBundle::default());
    // init board sprites
    for x in 0..MAX_WIDTH {
        for y in 0..MAX_HEIGHT {
            let color = if should_start_alive(x, y) {
                ALIVE_COLOR
            } else {
                DEAD_COLOR
            };
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::new(10.0, 10.0)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(
                        (x * 10) as f32 - 500.0,
                        (y * 10) as f32 - 300.0,
                        0.,
                    )),
                    ..default()
                },
                Position { x, y },
            ));
        }
    }
}

fn spawn_cells(board: &mut ResMut<Board>, sprite_query: &mut Query<(&mut Sprite, &Position)>) {
    for x in 0..MAX_WIDTH {
        for y in 0..MAX_HEIGHT {
            let cell = &mut board.0[x as usize][y as usize];
            if !cell.dead {
                continue;
            }

            if should_spawn(cell.neighbors) {
                cell.decaying = false;
                cell.dead = false;
                cell.decaying_ticks = DECAY_TICKS;
                let (mut sprite, _) = sprite_query
                    .iter_mut()
                    .find(|(_, position)| position.x == x && position.y == y)
                    .expect("Cell sprite not found");
                sprite.color = ALIVE_COLOR;
            }
        }
    }
}

fn decay_cells(board: &mut ResMut<Board>) {
    for x in 0..MAX_WIDTH {
        for y in 0..MAX_HEIGHT {
            let cell = &mut board.0[x as usize][y as usize];
            if (!should_stay_alive(cell.neighbors)) && !cell.decaying {
                cell.decaying = true;
            }
            if cell.decaying && cell.decaying_ticks > 0 {
                cell.decaying_ticks -= 1;
            }
        }
    }
}

fn kill_cells(board: &mut ResMut<Board>, sprite_query: &mut Query<(&mut Sprite, &Position)>) {
    for x in 0..MAX_WIDTH {
        for y in 0..MAX_HEIGHT {
            let cell = &mut board.0[x as usize][y as usize];
            if cell.dead {
                continue;
            }
            if cell.decaying && cell.decaying_ticks == 0 {
                cell.dead = true;
                let (mut sprite, _) = sprite_query
                    .iter_mut()
                    .find(|(_, position)| position.x == x && position.y == y)
                    .expect("Cell sprite not found");
                sprite.color = DEAD_COLOR;
            }
        }
    }
}

fn process_tick(
    time: Res<Time>,
    mut timer: ResMut<FrameTimer>,
    mut board: ResMut<Board>,
    mut sprite_query: Query<(&mut Sprite, &Position)>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }
    count_neighbors(&mut board);
    spawn_cells(&mut board, &mut sprite_query);
    decay_cells(&mut board);
    kill_cells(&mut board, &mut sprite_query);
    // count_neighbors,
    // spawn_cells,
    // decay_cells,
    // kill_cells,
}

fn main() {
    let board = create_board();
    App::new()
        .insert_resource(board)
        .insert_resource(PrintTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .insert_resource(FrameTimer(Timer::from_seconds(
            TICK_TIME,
            TimerMode::Repeating,
        )))
        .add_plugins(DefaultPlugins)
        // .add_systems(Startup, (setup_graphics, add_cells, init_cells).chain())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                process_tick,
                // count_neighbors,
                // spawn_cells,
                // decay_cells,
                // kill_cells,
                // print_cells,
            )
                .chain(),
        )
        .run();
}
