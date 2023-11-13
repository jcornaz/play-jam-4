use core::time::Duration;

use anyhow::anyhow;

use crankit_graphics::image::{self, Image};

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE};

pub struct Images {
    height: i32,
    body: Image,
    surface: Image,
}

impl Images {
    pub fn load() -> anyhow::Result<Self> {
        let body = Image::load("img/water/body")
            .map_err(|err| anyhow!("cannot load water body image: {err}"))?;
        let surface = Image::load("img/water/surface")
            .map_err(|err| anyhow!("cannot load water surface image: {err}"))?;
        let [_, height] = body.size();
        Ok(Self {
            height,
            body,
            surface,
        })
    }
}

pub struct Water {
    level: f32,
}

const RAISE_SPEED: f32 = 0.2;

/// Offset of the image relative to the level
const IMAGE_OFFSET: i32 = 7;
impl Water {
    pub fn new() -> Self {
        Self { level: 0.0 }
    }

    pub fn update(&mut self, delta_time: Duration) {
        self.level += RAISE_SPEED * delta_time.as_secs_f32();
    }

    pub fn vertical_position(&self) -> f32 {
        (SCREEN_HEIGHT as f32 / TILE_SIZE) - self.level
    }

    pub fn draw(&self, images: &Images) {
        image::with_draw_mode(image::DrawMode::XOR, || {
            let mut y = SCREEN_HEIGHT - (self.level * TILE_SIZE) as i32 - IMAGE_OFFSET;
            images
                .surface
                .draw_tiled([0, y], [SCREEN_WIDTH, images.height]);
            y += images.height;
            images
                .body
                .draw_tiled([0, y], [SCREEN_WIDTH, SCREEN_HEIGHT - y]);
        });
    }
}
