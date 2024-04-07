use crate::{
    config::{ALIVE_COLOR, DEAD_COLOR, DECAY_TICKS, MAX_HEIGHT, MAX_WIDTH},
    types::{Board, Cell, Position},
};

use bevy::prelude::{Camera2dBundle, Commands, Res, Sprite, SpriteBundle, Transform, Vec2, Vec3};
use std::fs;

fn should_start_alive(x: usize, y: usize, alive_board: &Vec<Vec<bool>>) -> bool {
    return alive_board[x][y];
}

fn read_alive_board() -> Vec<Vec<bool>> {
    let alive_file_string = fs::read_to_string("alive.csv").expect("Unable to read file");

    let mut alive_board: Vec<Vec<bool>> = vec![vec![false; MAX_HEIGHT]; MAX_WIDTH];

    let mut line_number = 0;
    for line in alive_file_string.lines() {
        line_number += 1;
        let xy = line.split(";").collect::<Vec<&str>>();
        if xy.len() != 2 {
            println!(
                "Warning: Invalid alive cell position: {}, on line {} of alive.csv",
                line, line_number
            );
            continue;
        }
        let x = xy[0].parse::<usize>().unwrap();
        let y = xy[1].parse::<usize>().unwrap();
        if x >= MAX_WIDTH || y >= MAX_HEIGHT {
            panic!("Invalid alive cell position: ({}, {})", x, y);
        }
        alive_board[x][y] = true;
    }
    return alive_board;
}

pub fn create_board() -> Board {
    let alive_board = read_alive_board();
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
            if should_start_alive(x, y, &alive_board) {
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

pub fn setup(mut commands: Commands, board: Res<Board>) {
    //init camera
    commands.spawn(Camera2dBundle::default());
    // init board sprites
    for x in 0..MAX_WIDTH {
        for y in 0..MAX_HEIGHT {
            let color = if board.0[x][y].dead {
                DEAD_COLOR
            } else {
                ALIVE_COLOR
            };
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::new(10.0, 10.0)),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(Vec3::new(
                        (x * 10) as f32 - 500.0,
                        (y * 10) as f32 - 300.0,
                        0.,
                    )),
                    ..Default::default()
                },
                Position { x, y },
            ));
        }
    }
}
