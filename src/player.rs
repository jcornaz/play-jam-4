use core::time::Duration;

use anyhow::anyhow;

use collision::Aabb;
use crankit_graphics::image::{Flip, Image};
use crankit_input::{Button, ButtonState};
use grid::Grid;

use crate::{
    animation::Animation, collides_against_terrain, level::Cell, IVector, Vector, TILE_SIZE,
};

const RUN_SPEED: f32 = 5.;
const ANIMATION_FPS: f32 = 10.0;
const RUN_ANIMATION_LEN: usize = 4;

/// Top-left of the collision bounding box relative to the player position
const BOUNDING_BOX_MIN: Vector = Vector::new(-6. / TILE_SIZE, -12. / TILE_SIZE);

/// Bottom-right of the collision bounding box relative to the player position
const BOUNDING_BOX_MAX: Vector = Vector::new(6. / TILE_SIZE, 0.);

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
    flip: bool,
}

impl Player {
    pub fn new(position: Vector) -> Self {
        Self {
            position,
            state: State::Idle,
            flip: false,
        }
    }

    pub fn handle_input(&mut self, buttons: ButtonState) {
        let horizontal_input = horizontal_input(buttons);
        if horizontal_input != 0 {
            let new_velocity = horizontal_input as f32 * RUN_SPEED;
            match &mut self.state {
                State::Idle => self.state = State::start_running(new_velocity),
                State::Running { velocity, .. } => *velocity = new_velocity,
            }
            self.flip = new_velocity < 0.;
        } else {
            self.state = State::Idle;
        }
    }

    pub fn update(&mut self, delta_time: Duration, grid: &Grid<Cell>) {
        match &mut self.state {
            State::Idle => (),
            State::Running {
                velocity,
                animation,
            } => {
                animation.update(delta_time);
                self.position.x += *velocity * delta_time.as_secs_f32();
            }
        }
        let collision_shape = Aabb::from_min_max(
            self.position + BOUNDING_BOX_MIN,
            self.position + BOUNDING_BOX_MAX,
        );
        if let Some(penetration) = collides_against_terrain(grid, collision_shape) {
            self.position += penetration;
        }
    }

    pub fn draw(&self, images: &Images) {
        let image = match &self.state {
            State::Idle => &images.idle,
            State::Running { animation, .. } => &images.running[animation.current_frame],
        };
        let pos = (self.position * TILE_SIZE).as_vector_i32() + images.top_left;
        let flip = if self.flip {
            Flip::FlippedX
        } else {
            Flip::Unflipped
        };
        image.draw_with_flip(pos, flip);
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
    Running { velocity: f32, animation: Animation },
    // AirBorne { velocity: Vector },
}

impl State {
    fn start_running(velocity: f32) -> Self {
        Self::Running {
            velocity,
            animation: Animation::new(RUN_ANIMATION_LEN, ANIMATION_FPS),
        }
    }
}
