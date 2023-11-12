#![cfg_attr(not(test), no_std)]

extern crate alloc;

use alloc::vec::Vec;
use core::time::Duration;

use anyhow::anyhow;
use playdate_sys::println;

use crankit_game_loop::game_loop;
use crankit_graphics::{image::Image, Color};
use crankit_input::{button_state, crank_change};
use crankit_time::reset_elapsed_time;
use level::Definition;

use crate::level::Level;

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
    level: Level,
    #[cfg(feature = "draw-fps")]
    frame_durations: Vec<Duration>,
}

#[cfg(feature = "draw-fps")]
const FRAME_WINDOW: usize = 30;

impl crankit_game_loop::Game for Game {
    fn new() -> Self {
        let level = Definition::load(0).unwrap().into();
        let images = Images::load().unwrap();
        Self {
            images,
            level,
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
        let crank_change = crank_change();
        self.level.update(delta_time, buttons, crank_change);
    }

    fn draw(&mut self) {
        crankit_graphics::clear(Color::black());
        self.level.draw(&self.images);
        #[cfg(feature = "draw-fps")]
        crankit_graphics::draw_fps([0, 0]);
    }
}

game_loop!(Game);
