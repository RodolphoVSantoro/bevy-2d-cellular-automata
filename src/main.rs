use std::cmp;

use bevy::prelude::*;

const MAX_WIDTH: i32 = 100;
const MAX_HEIGHT: i32 = 60;

const FRAME_TIME: f32 = 0.2;

const GROWTH_SPEED: f32 = 10.0;

const DECAY_TIME: i32 = (GROWTH_SPEED / (1.0 / FRAME_TIME)) as i32;

const DEAD_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
const ALIVE_COLOR: Color = Color::rgb(0.75, 0.75, 0.75);

struct Cell {
    decaying: bool,
    decaying_time: i32,
    dead: bool,
    neighbors: i32,
}

#[derive(Component)]
struct Person;

impl Default for Cell {
    fn default() -> Self {
        Cell {
            decaying: true,
            dead: true,
            decaying_time: 0,
            neighbors: 0,
        }
    }
}

#[derive(Resource)]
struct Board(Vec<Vec<Cell>>);

fn is_cell_adjacent(cell_position: (i32, i32), other_position: (i32, i32)) -> bool {
    let x_dist = (cell_position.0 - other_position.0).abs();
    let y_dist = (cell_position.1 - other_position.1).abs();
    return x_dist == 1 && y_dist < 2 || y_dist == 1 && x_dist < 2;
}

fn limited_positive_range(start: i32, end: i32, max: i32) -> std::ops::Range<i32> {
    return cmp::max(start, 0)..cmp::min(end, max);
}

fn count_neighbors(mut board: ResMut<Board>) {
    for x in 0..MAX_WIDTH {
        for y in 0..MAX_HEIGHT {
            let mut neighbors = 0;
            for other_x in limited_positive_range(x - 1, x + 1, MAX_WIDTH) {
                for other_y in limited_positive_range(y - 1, y + 1, MAX_HEIGHT) {
                    if board.0[other_x as usize][other_y as usize].dead {
                        continue;
                    }
                    if is_cell_adjacent((x, y), (other_x, other_y)) {
                        neighbors += 1;
                    }
                }
            }
            board.0[x as usize][y as usize].neighbors = neighbors;
        }
    }
}

#[derive(Component, Debug)]
struct Position {
    x: i32,
    y: i32,
}

fn should_start_alive(x: i32, y: i32) -> bool {
    return (x - 20) * (x - 20) + (y - 20) * (y - 20) < 5 * 5 && x % 3 != 0;
}

fn should_stay_alive(neighbors: i32) -> bool {
    return neighbors == 2 || neighbors == 3;
}

fn should_spawn(neighbors: i32) -> bool {
    return neighbors == 1;
}

fn create_board() -> Board {
    let mut board: Board = Board(vec![]);
    for x in 0..MAX_WIDTH {
        let mut column: Vec<Cell> = vec![];
        for y in 0..MAX_HEIGHT {
            let mut cell = Cell {
                decaying: true,
                dead: true,
                decaying_time: 0,
                neighbors: 0,
            };
            if should_start_alive(x, y) {
                cell.decaying = false;
                cell.dead = false;
                cell.decaying_time = DECAY_TIME;
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

// fn print_cells(time: Res<Time>, mut timer: ResMut<PrintTimer>, board: Res<Board>) {
//     if !timer.0.tick(time.delta()).just_finished() {
//         return;
//     }

//     let mut everybody_dies = true;

//     for x in 0..MAX_WIDTH {
//         for y in 0..MAX_HEIGHT {
//             let is_dead = board.0[x as usize][y as usize].decaying
//                 && board.0[x as usize][y as usize].decaying_time == 0;
//             if is_dead {
//                 continue;
//             }
//             everybody_dies = false;
//             print!("Cell at ({}, {})", x, y);
//             let cell = &board.0[x as usize][y as usize];
//             if !cell.decaying {
//                 print!(" is not decaying");
//             } else {
//                 print!(" is decaying in {} frames", cell.decaying_time);
//             }
//             println!(", and has {} neighbors", cell.neighbors);
//         }
//     }

//     if everybody_dies {
//         println!("Everybody dies!");
//     }
// }

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

fn spawn_cells(
    time: Res<Time>,
    mut timer: ResMut<FrameTimer>,
    // mut commands: Commands,
    mut sprite_query: Query<(&mut Sprite, &Position)>,
    mut board: ResMut<Board>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    for x in 0..MAX_WIDTH {
        for y in 0..MAX_HEIGHT {
            let cell = &mut board.0[x as usize][y as usize];
            let is_dead = cell.decaying && cell.decaying_time == 0;
            if !is_dead {
                continue;
            }

            if should_spawn(cell.neighbors) {
                cell.decaying = false;
                cell.dead = false;
                cell.decaying_time = DECAY_TIME;
                let (mut sprite, _) = sprite_query
                    .iter_mut()
                    .find(|(_, position)| position.x == x && position.y == y)
                    .expect("Cell sprite not found");
                sprite.color = ALIVE_COLOR;
                // cell_sprite.sprite_bundle.sprite.color = Color::rgb(0.75, 0.75, 0.75);
            }
        }
    }
}

fn decay_cells(time: Res<Time>, mut timer: ResMut<FrameTimer>, mut board: ResMut<Board>) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    for x in 0..MAX_WIDTH {
        for y in 0..MAX_HEIGHT {
            let cell = &mut board.0[x as usize][y as usize];
            if (!should_stay_alive(cell.neighbors)) && !cell.decaying {
                cell.decaying = true;
            }
            if cell.decaying && cell.decaying_time > 0 {
                cell.decaying_time -= 1;
            }
        }
    }
}

fn kill_cells(
    time: Res<Time>,
    mut timer: ResMut<FrameTimer>,
    mut board: ResMut<Board>,
    mut sprite_query: Query<(&mut Sprite, &Position)>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    for x in 0..MAX_WIDTH {
        for y in 0..MAX_HEIGHT {
            let cell = &mut board.0[x as usize][y as usize];
            if cell.dead {
                continue;
            }
            if cell.decaying && cell.decaying_time == 0 {
                cell.dead = true;
                let (mut sprite, _) = sprite_query
                    .iter_mut()
                    .find(|(_, position)| position.x == x && position.y == y)
                    .expect("Cell sprite not found");
                sprite.color = DEAD_COLOR;
                // cell_sprite.sprite_bundle.sprite.color = Color::rgb(0.25, 0.25, 0.25);
            }
        }
    }
}

fn main() {
    let board = create_board();
    App::new()
        .insert_resource(board)
        .insert_resource(PrintTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .insert_resource(FrameTimer(Timer::from_seconds(
            FRAME_TIME,
            TimerMode::Repeating,
        )))
        .add_plugins(DefaultPlugins)
        // .add_systems(Startup, (setup_graphics, add_cells, init_cells).chain())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                count_neighbors,
                spawn_cells,
                decay_cells,
                kill_cells,
                // print_cells,
            )
                .chain(),
        )
        .run();
}
