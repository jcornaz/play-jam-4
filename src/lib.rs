#![cfg_attr(not(test), no_std)]

extern crate alloc;

mod animation;
mod level;
mod player;

use crankit_game_loop::game_loop;

use crankit_graphics::{image::Image, Color};
use crankit_input::button_state;
use crankit_time::reset_elapsed_time;
use grid::Grid;
use level::{Cell, Level};
use player::Player;

type Vector = math2d::Vector<f32>;
type Point = math2d::Point<f32>;
type CellCoord = [usize; 2];

struct Game {
    level_image: Image,
    player_images: player::Images,
    player: Player,
    grid: Grid<Cell>,
}

impl crankit_game_loop::Game for Game {
    fn new() -> Self {
        let level = Level::load(0).unwrap();
        let player_images = player::Images::load().unwrap();
        let player = Player::new(level.player_start);
        Self {
            level_image: level.walls_image,
            player,
            player_images,
            grid: level.grid,
        }
    }

    fn update(&mut self) {
        let delta_time = reset_elapsed_time();
        let buttons = button_state();
        crankit_graphics::clear(Color::black());
        self.level_image.draw([0, 0]);
        self.player.update(delta_time, buttons, &self.grid);
        self.player.draw(&self.player_images);
    }
}

const TILE_SIZE: f32 = 16.0;

fn world_to_coord(point: Point) -> CellCoord {
    let v = Vector::from(point) / TILE_SIZE;
    [v.x as usize, v.y as usize]
}

fn coord_to_world([x, y]: CellCoord) -> Point {
    (Vector::new(x as f32, y as f32) * TILE_SIZE).into()
}

game_loop!(Game);
