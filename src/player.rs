use core::time::Duration;

use anyhow::anyhow;

use collision::Aabb;
use crankit_graphics::image::{Flip, Image};
use crankit_input::{Button, ButtonState};

use crate::{animation::Animation, IVector, Vector, TILE_SIZE};

const RUN_SPEED: f32 = 5.;
const ANIMATION_FPS: f32 = 10.0;
const RUN_ANIMATION_LEN: usize = 4;
const JUMP_VELOCITY: f32 = 10.;
const GRAVITY: f32 = 25.;

/// Top-left of the collision bounding box relative to the player position
const COLLISION_BOX_TOP_LEFT: Vector = Vector::new(-6. / TILE_SIZE, -12. / TILE_SIZE);

/// Bottom-right of the collision bounding box relative to the player position
const COLLISION_BOX_BOTTOM_RIGHT: Vector = Vector::new(6. / TILE_SIZE, 0.);

pub struct Images {
    /// Vector from the origin of the player to the top-left of the images
    top_left: IVector,
    idle: Image,
    running: [Image; RUN_ANIMATION_LEN],
    falling: Image,
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
        let falling = images.next().unwrap();
        let _dying = images.next().unwrap();
        let [w, h] = idle.size();
        let top_left = IVector::new(-w / 2, -h);
        Ok(Self {
            top_left,
            idle,
            running,
            falling,
        })
    }
}

pub struct Player {
    position: Vector,
    velocity: Vector,
    is_on_ground: bool,
    run_animation: Option<Animation>,
}

impl Player {
    pub fn new(position: Vector) -> Self {
        Self {
            position,
            is_on_ground: false,
            velocity: Vector::ZERO,
            run_animation: None,
        }
    }

    pub fn handle_input(&mut self, buttons: ButtonState) {
        let jump = buttons.is_just_pressed(Button::A);
        if jump && self.is_on_ground {
            self.velocity.y = -JUMP_VELOCITY;
            self.is_on_ground = false;
        }
        self.velocity.x = horizontal_speed_input(buttons);
    }

    pub fn update(&mut self, delta_time: Duration) {
        self.update_animation(delta_time);
        let delta_seconds = delta_time.as_secs_f32();
        self.velocity.y += GRAVITY * delta_seconds;
        self.position += self.velocity * delta_seconds;
    }

    pub fn move_by(&mut self, delta: Vector) {
        self.position += delta;
    }

    pub fn on_floor_hit(&mut self) {
        if self.velocity.y >= 0.0 {
            self.velocity.y = 0.0;
            self.is_on_ground = true;
        }
    }

    pub fn on_roof_hit(&mut self) {
        if self.velocity.y < 0.0 {
            self.velocity.y = 0.0;
        }
    }

    pub fn collision_box(&self) -> Aabb {
        Aabb::from_min_max(
            self.position + COLLISION_BOX_TOP_LEFT,
            self.position + COLLISION_BOX_BOTTOM_RIGHT,
        )
    }

    fn update_animation(&mut self, delta_time: Duration) {
        let is_running = self.is_on_ground && libm::fabsf(self.velocity.x) > 0.0;
        match (is_running, &mut self.run_animation) {
            (true, None) => {
                self.run_animation = Some(Animation::new(RUN_ANIMATION_LEN, ANIMATION_FPS))
            }
            (true, Some(animation)) => animation.update(delta_time),
            (false, Some(_)) => self.run_animation = None,
            (false, None) => (),
        }
    }

    pub fn draw(&self, images: &Images) {
        let image = if !self.is_on_ground {
            &images.falling
        } else if let Some(anim) = &self.run_animation {
            &images.running[anim.current_frame]
        } else {
            &images.idle
        };
        let pos = (self.position * TILE_SIZE).as_vector_i32() + images.top_left;
        let flip = if self.velocity.x < 0.0 {
            Flip::FlippedX
        } else {
            Flip::Unflipped
        };
        image.draw_with_flip(pos, flip);
    }
}

fn horizontal_speed_input(buttons: ButtonState) -> f32 {
    if buttons.is_pressed(Button::Right) {
        RUN_SPEED
    } else if buttons.is_pressed(Button::Left) {
        -RUN_SPEED
    } else {
        0.
    }
}
