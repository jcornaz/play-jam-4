use core::time::Duration;

use anyhow::anyhow;
use crankit_graphics::image::Image;
use crankit_input::{Button, ButtonState};
use grid::Grid;
use math2d::Point;

use crate::{animation::Animation, coord_to_world, level::Cell, world_to_coord, Vec2};

const RUN_SPEED: f32 = 200.;
const ANIMATION_FPS: f32 = 10.0;
const RUN_ANIMATION_LEN: usize = 4;
const HALF_WIDTH: f32 = 6.0;

pub struct Images {
    idle: Image,
    running: [Image; RUN_ANIMATION_LEN],
    _falling: Image,
    _dying: Image,
}

impl Images {
    pub fn load() -> anyhow::Result<Self> {
        let sheet = &Image::load("img/player-sheet")
            .map_err(|err| anyhow!("cannot load player images: {err}"))?;
        let mut images = sheet.split_columns(7);
        let idle = images.next().unwrap();
        let _running = [
            images.next().unwrap(),
            images.next().unwrap(),
            images.next().unwrap(),
            images.next().unwrap(),
        ];
        let _falling = images.next().unwrap();
        let _dying = images.next().unwrap();
        Ok(Self {
            idle,
            running: _running,
            _falling,
            _dying,
        })
    }
}

pub struct Player {
    position: Point,
    state: State,
}

impl Player {
    pub fn new(position: Point) -> Self {
        Self {
            position,
            state: State::Idle,
        }
    }

    pub fn update(&mut self, delta_time: Duration, buttons: ButtonState, grid: &Grid<Cell>) {
        let horizontal_input = horizontal_input(buttons);
        if horizontal_input != 0 {
            match &mut self.state {
                State::Idle => self.state = State::running(),
                State::Running { animation } => animation.update(delta_time),
            };
            let delta = Vec2::X * (horizontal_input as f32 * RUN_SPEED * delta_time.as_secs_f32());
            let mut next_pos = self.position + delta;
            let next_coord = world_to_coord(next_pos);
            if let Some(Cell::Terrain) = grid.get(next_coord) {
                next_pos.x = coord_to_world(next_coord).x - HALF_WIDTH
            }
            self.position = next_pos;
        } else {
            self.state = State::Idle;
        }
    }

    pub fn draw(&self, images: &Images) {
        let image = match &self.state {
            State::Idle => &images.idle,
            State::Running { animation } => &images.running[animation.current_frame],
        };
        image.draw_from_center(self.position.as_point_i32());
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
