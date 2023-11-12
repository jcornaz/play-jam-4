#![cfg_attr(not(test), no_std)]

extern crate alloc;

use alloc::vec::Vec;

use playdate_sys::println;

use collision::Aabb;
use crankit_game_loop::game_loop;
use crankit_graphics::{image::Image, Color};
use crankit_input::{button_state, crank_change};
use crankit_time::reset_elapsed_time;
use grid::Grid;
use level::{Cell, Level};
use player::Player;

use crate::lift::Lift;

mod animation;
mod level;
mod lift;
mod player;

type Vector = math2d::Vector<f32>;
type IVector = math2d::Vector<i32>;

const TILE_SIZE: f32 = 16.0;

const PENETRATION_RESOLUTION_MAX_ITER: u32 = 10;

struct Game {
    level_image: Image,
    lift_image: Image,
    player_images: player::Images,
    player: Player,
    grid: Grid<Cell>,
    lifts: Vec<Lift>,
    active_lift: Option<usize>,
}

impl crankit_game_loop::Game for Game {
    fn new() -> Self {
        let level = Level::load(0).unwrap();
        let player_images = player::Images::load().unwrap();
        let player = Player::new(level.player_start);
        let lifts = level
            .lifts
            .into_iter()
            .map(|(base, height)| Lift::new(base, height))
            .collect();
        let lift_image = Image::load("img/lift").unwrap();
        Self {
            level_image: level.walls_image,
            player,
            player_images,
            grid: level.grid,
            lifts,
            lift_image,
            active_lift: None,
        }
    }

    fn update(&mut self) {
        self.update();
        self.draw();
    }
}

impl Game {
    fn update(&mut self) {
        let delta_time = reset_elapsed_time();
        let buttons = button_state();
        self.player.handle_input(buttons);
        self.player.update(delta_time);
        let player_collision_box = self.player.collision_box();
        self.resolve_collisions();
        if let Some(lift) = self.active_lift.map(|i| &mut self.lifts[i]) {
            if player_collision_box.collides(lift.interaction_box()) {
                lift.update(crank_change(), &mut self.player);
            } else {
                self.active_lift = None;
            }
        } else if let Some(index) = (0..self.lifts.len()).find(|i| {
            self.lifts[*i]
                .interaction_box()
                .collides(player_collision_box)
        }) {
            self.active_lift = Some(index);
        }
    }

    fn draw(&mut self) {
        crankit_graphics::clear(Color::black());
        self.player.draw(&self.player_images);
        self.lifts.iter().for_each(|l| l.draw(&self.lift_image));
        self.level_image.draw([0, 0]);
    }

    fn resolve_collisions(&mut self) {
        let mut iter = 0;
        while let Some(penetration) = self.collides_against_terrain(self.player.collision_box()) {
            iter += 1;
            if iter > PENETRATION_RESOLUTION_MAX_ITER {
                println!(
                    "Exhausted number of iteration for inter-penetration resolution ({})",
                    PENETRATION_RESOLUTION_MAX_ITER
                );
                return;
            }
            self.player.move_by(penetration);
            if penetration.y < 0. {
                self.player.on_floor_hit();
            } else if penetration.y > 0. {
                self.player.on_roof_hit();
            }
        }
    }

    fn collides_against_terrain(&self, bounding_box: Aabb) -> Option<Vector> {
        let terrain = coords(bounding_box)
            .filter(|c| matches!(self.grid.get(*c), Some(Cell::Terrain)))
            .map(|[x, y]| {
                Aabb::from_min_max([x as f32, y as f32], [(x + 1) as f32, (y + 1) as f32])
            });
        let lifts = self.lifts.iter().map(|l| l.collision_box());
        bounding_box
            .max_penetration(terrain.chain(lifts))
            .map(Into::into)
    }
}

fn coords(bounding_box: Aabb) -> impl Iterator<Item = [usize; 2]> {
    let [min_x, max_x] = bounding_box.x.into();
    let [min_y, max_y] = bounding_box.y.into();
    ((libm::floorf(min_x) as usize)..=(libm::ceilf(max_x) as usize)).flat_map(move |x| {
        ((libm::floorf(min_y) as usize)..=(libm::ceilf(max_y) as usize)).map(move |y| [x, y])
    })
}

game_loop!(Game);
