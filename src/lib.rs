#![cfg_attr(not(test), no_std)]

extern crate alloc;

use alloc::vec::Vec;
use core::time::Duration;

use anyhow::anyhow;
use libm::{ceilf, floorf};
use playdate_sys::println;

use collision::Aabb;
use crankit_game_loop::game_loop;
use crankit_graphics::{image::Image, Color};
use crankit_input::{button_state, crank_change};
use crankit_time::reset_elapsed_time;
use level::{Cell, LevelDef};
use player::Player;

use crate::lift::Lift;
use crate::water::Water;

mod animation;
mod level;
mod lift;
mod player;
mod water;

type Vector = math2d::Vector<f32>;
type IVector = math2d::Vector<i32>;

const TILE_SIZE: f32 = 16.0;
const SCREEN_WIDTH: i32 = 400;
const SCREEN_HEIGHT: i32 = 240;

const PENETRATION_RESOLUTION_MAX_ITER: u32 = 10;

struct Images {
    player_images: player::Images,
    water_images: water::Images,
    lift_image: Image,
}

impl Images {
    fn load() -> anyhow::Result<Self> {
        let player_images =
            player::Images::load().map_err(|err| anyhow!("cannot load player image: {err}"))?;
        let water_images =
            water::Images::load().map_err(|err| anyhow!("cannot load water images: {err}"))?;
        let lift_image =
            Image::load("img/lift").map_err(|err| anyhow!("cannot load lift image: {err}"))?;
        Ok(Self {
            player_images,
            water_images,
            lift_image,
        })
    }
}

struct Game {
    images: Images,
    level: LevelDef,
    player: Player,
    water: Water,
    lifts: Vec<Lift>,
    active_lift: Option<usize>,
    #[cfg(feature = "draw-fps")]
    frame_durations: Vec<Duration>,
}

#[cfg(feature = "draw-fps")]
const FRAME_WINDOW: usize = 30;

impl crankit_game_loop::Game for Game {
    fn new() -> Self {
        let level = LevelDef::load(0).unwrap();
        let images = Images::load().unwrap();
        let player = Player::new(level.player_start);
        let lifts = level
            .lifts
            .iter()
            .copied()
            .map(|(base, height)| Lift::new(base, height))
            .collect();
        Self {
            images,
            level,
            player,
            lifts,
            active_lift: None,
            water: Water::new(),
            #[cfg(feature = "draw-fps")]
            frame_durations: Vec::with_capacity(FRAME_WINDOW),
        }
    }

    fn update(&mut self) {
        let delta_time = reset_elapsed_time();
        self.update(delta_time);
        self.draw();
        #[cfg(feature = "draw-fps")]
        {
            self.frame_durations.push(crankit_time::elapsed_time());
            if self.frame_durations.len() >= FRAME_WINDOW {
                let max_duration = self.frame_durations.drain(0..).max().unwrap_or_default();
                println!("(max) frame duration: {max_duration:?}")
            }
        }
    }
}

impl Game {
    fn update(&mut self, delta_time: Duration) {
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
        self.water.update(delta_time);
    }

    fn draw(&mut self) {
        crankit_graphics::clear(Color::black());
        self.level.background.iter().for_each(|i| i.draw([0, 0]));
        self.player.draw(&self.images.player_images);
        self.lifts
            .iter()
            .for_each(|l| l.draw(&self.images.lift_image));
        self.level.foreground.iter().for_each(|i| i.draw([0, 0]));
        self.water.draw(&self.images.water_images);
        #[cfg(feature = "draw-fps")]
        crankit_graphics::draw_fps([0, 0]);
    }

    fn resolve_collisions(&mut self) {
        let mut iter = 0;
        while let Some(penetration) = self.collides_against_terrain() {
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

    fn collides_against_terrain(&self) -> Option<Vector> {
        let player_collision_box = self.player.collision_box();
        let terrain = coords(player_collision_box)
            .filter(|c| matches!(self.level.grid.get(*c), Some(Cell::Terrain)))
            .map(|[x, y]| {
                Aabb::from_min_max([x as f32, y as f32], [(x + 1) as f32, (y + 1) as f32])
            });
        let lifts = self.lifts.iter().map(|l| l.collision_box());
        player_collision_box
            .max_penetration(terrain.chain(lifts))
            .map(Into::into)
    }

    fn collides_against_hazard(&self) -> bool {
        if self.player.position().y > self.water.position() {
            return true;
        }
        let player_collision_box = self.player.collision_box();
        let hazards = coords(player_collision_box)
            .filter(|c| matches!(self.level.grid.get(*c), Some(Cell::Hazard)))
            .map(|[x, y]| {
                Aabb::from_min_max(
                    [x as f32 + 0.1, y as f32 + 0.1],
                    [x as f32 + 0.8, x as f32 + 0.8],
                )
            });
        player_collision_box.collides_any(hazards)
    }
}

fn coords(bounding_box: Aabb) -> impl Iterator<Item = [usize; 2]> {
    let [min_x, max_x] = bounding_box.x.into();
    let [min_y, max_y] = bounding_box.y.into();
    ((floorf(min_x) as usize)..=(ceilf(max_x) as usize)).flat_map(move |x| {
        ((floorf(min_y) as usize)..=(ceilf(max_y) as usize)).map(move |y| [x, y])
    })
}

game_loop!(Game);
