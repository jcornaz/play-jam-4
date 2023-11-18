#![cfg_attr(not(test), no_std)]

extern crate alloc;

#[cfg(feature = "draw-fps")]
use alloc::vec::Vec;
use core::time::Duration;
use crankit_input::{ButtonsStateSource, CrankStateSource};
use playdate_sys::ffi::PlaydateAPI;

use anyhow::anyhow;
#[cfg(feature = "draw-fps")]
use playdate_sys::println;

use crankit_game_loop::game_loop;
use crankit_graphics::{image::Image, Color};
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
    player: player::Images,
    water: water::Images,
    lift: Image,
    key: Image,
}

impl Images {
    fn load() -> anyhow::Result<Self> {
        let player =
            player::Images::load().map_err(|err| anyhow!("cannot load player image: {err}"))?;
        let water =
            water::Images::load().map_err(|err| anyhow!("cannot load water images: {err}"))?;
        let lift =
            Image::load("img/lift").map_err(|err| anyhow!("cannot load lift image: {err}"))?;
        let key = Image::load("img/key").map_err(|err| anyhow!("cannot load key image: {err}"))?;
        Ok(Self {
            player,
            water,
            lift,
            key,
        })
    }
}

struct Game {
    images: Images,
    thank_you_image: Image,
    level: Option<Level>,
    #[cfg(feature = "draw-fps")]
    frame_durations: Vec<Duration>,
}

#[cfg(feature = "draw-fps")]
const FRAME_WINDOW: usize = 30;

impl crankit_game_loop::Game for Game {
    fn new(_: &PlaydateAPI) -> Self {
        let level = Definition::load(0).unwrap().into();
        let images = Images::load().unwrap();
        let thank_you_image = Image::load("img/thanks").unwrap();
        Self {
            images,
            thank_you_image,
            level: Some(level),
            #[cfg(feature = "draw-fps")]
            frame_durations: Vec::with_capacity(FRAME_WINDOW),
        }
    }

    fn update(&mut self, playdate: &PlaydateAPI) {
        let delta_time = reset_elapsed_time();
        self.update(delta_time, playdate);
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
    fn update(&mut self, delta_time: Duration, playdate: &PlaydateAPI) {
        if let Some(level) = &mut self.level {
            let buttons = playdate.buttons_state();
            let crank_change = playdate.crank_change_deg();
            level.update(delta_time, buttons, crank_change);
            if level.is_over() {
                self.level = self.level.take().and_then(|l| l.next());
            }
        }
    }

    fn draw(&mut self) {
        match &self.level {
            None => self.thank_you_image.draw([0, 0]),
            Some(level) => {
                crankit_graphics::clear(Color::black());
                level.draw(&self.images)
            }
        }
        #[cfg(feature = "draw-fps")]
        crankit_graphics::draw_fps([0, 0]);
    }
}

game_loop!(Game);
