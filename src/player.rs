use core::time::Duration;

use anyhow::anyhow;

use crankit_graphics::image::Image;
use crankit_input::{Button, ButtonState};
use grid::Grid;

use crate::{animation::Animation, level::Cell, IVector, Vector, TILE_SIZE};

const RUN_SPEED: f32 = 5.;
const ANIMATION_FPS: f32 = 10.0;
const RUN_ANIMATION_LEN: usize = 4;

pub struct Images {
    /// Vector from the origin of the player to the top-left of the images
    top_left: IVector,
    idle: Image,
    running: [Image; RUN_ANIMATION_LEN],
}

impl Images {
    pub fn load() -> anyhow::Result<Self> {
        let sheet = &Image::load("img/player-sheet")
            .map_err(|err| anyhow!("cannot load player images: {err}"))?;
        let mut images = sheet.split_columns(7);
        let idle = images.next().unwrap();
        let running = [
            images.next().unwrap(),
            images.next().unwrap(),
            images.next().unwrap(),
            images.next().unwrap(),
        ];
        let _falling = images.next().unwrap();
        let _dying = images.next().unwrap();
        let [w, h] = idle.size();
        let top_left = IVector::new(-w / 2, -h);
        Ok(Self {
            idle,
            running,
            top_left,
        })
    }
}

pub struct Player {
    position: Vector,
    state: State,
}

impl Player {
    pub fn new(position: Vector) -> Self {
        Self {
            position,
            state: State::Idle,
        }
    }

    pub fn update(&mut self, delta_time: Duration, buttons: ButtonState, _grid: &Grid<Cell>) {
        let horizontal_input = horizontal_input(buttons);
        if horizontal_input != 0 {
            match &mut self.state {
                State::Idle => self.state = State::running(),
                State::Running { animation } => animation.update(delta_time),
            };
            let delta =
                Vector::X * (horizontal_input as f32 * RUN_SPEED * delta_time.as_secs_f32());
            self.position += delta;
        } else {
            self.state = State::Idle;
        }
    }

    pub fn draw(&self, images: &Images) {
        let image = match &self.state {
            State::Idle => &images.idle,
            State::Running { animation } => &images.running[animation.current_frame],
        };
        let pos = (self.position * TILE_SIZE).as_vector_i32() + images.top_left;
        image.draw(pos);
    }
}

fn horizontal_input(buttons: ButtonState) -> i32 {
    if buttons.is_pressed(Button::Right) {
        1
    } else if buttons.is_pressed(Button::Left) {
        -1
    } else {
        0
    }
}

enum State {
    Idle,
    Running { animation: Animation },
}

impl State {
    fn running() -> Self {
        Self::Running {
            animation: Animation::new(RUN_ANIMATION_LEN, ANIMATION_FPS),
        }
    }
}
